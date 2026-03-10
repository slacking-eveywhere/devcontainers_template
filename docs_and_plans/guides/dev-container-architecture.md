# dev-container Architecture Reference

## Overview

`dev-container` is a standalone Rust binary that manages devcontainer lifecycles. It replaces
the original `bin/devcontainer` bash script. The binary is fully self-contained with no runtime
dependencies beyond the system's `docker` and `ssh` binaries.

---

## Repository Layout

```
dev-container/
├── Cargo.toml              ← package manifest, dependency declarations, release profile
├── Cargo.lock              ← locked dependency tree
├── Makefile                ← developer workflow targets
├── src/
│   ├── main.rs             ← entry point: parse CLI, call dispatch, print errors, set exit code
│   ├── cli.rs              ← clap derive structs: Cli, Command enum, per-command arg structs
│   ├── config.rs           ← path resolution, .env parser, project name validator
│   ├── docker.rs           ← thin subprocess wrappers for docker and docker compose
│   └── commands/
│       ├── mod.rs          ← dispatch table: Command variant → handler function
│       ├── new.rs          ← create project directory, write .env and compose templates
│       ├── edit.rs         ← open config file in system editor, optionally reload container
│       ├── inspect.rs      ← display formatted env config and merged compose output
│       ├── run.rs          ← start container, create volumes, inject SSH public key
│       ├── stop.rs         ← stop container with detach and force options
│       ├── connect.rs      ← exec-replace process with ssh, resolve host/port from .env
│       └── list.rs         ← list all projects in a table with status, port, image
└── tests/
    ├── common/
    │   └── mod.rs          ← TestEnv helper, binary path, assert helpers
    ├── test_new.rs
    ├── test_edit.rs
    ├── test_inspect.rs
    ├── test_run.rs
    ├── test_stop.rs
    ├── test_connect.rs
    └── test_list.rs
```

---

## Runtime Data Layout

All data lives under `$HOME/.local/share/devcontainer/`:

```
~/.local/share/devcontainer/
├── templates/
│   ├── docker-compose-ssh.yml      ← base compose template (volume mounts, ports, image)
│   ├── docker-compose-vscode.yml   ← vscode-specific compose overlay
│   └── devcontainer.json           ← IDE devcontainer descriptor
└── projects/
    └── <name>/
        ├── .env                    ← user configuration (image, user, ports, SSH)
        ├── docker-compose.yml      ← project overlay (env_file pointer, overrides)
        └── volumes/
            ├── workdir/            ← bind-mounted as /workdir in the container
            └── <USER>/             ← bind-mounted as /home/<USER> in the container
```

The `$HOME` variable is the only runtime path anchor. No paths are compiled into the binary.

---

## Module Responsibilities

### `main.rs`

Single responsibility: parse the CLI with `clap`, call `commands::dispatch`, and translate
`anyhow::Error` into a stderr message and a non-zero exit code. No business logic lives here.

### `cli.rs`

Defines the complete CLI surface as clap derive structs. All argument types, short/long flag
names, and help strings are declared here. No logic.

Command surface:

| Command   | Positional  | Flags                                      |
|-----------|-------------|---------------------------------------------|
| `new`     | `<name>`    | `-f/--force`                                |
| `edit`    | `<name>`    | `-c/--compose`, `-q/--no-reload`            |
| `inspect` | `<name>`    | —                                           |
| `run`     | `<name>`    | `-s/--ssh-key <path>`                       |
| `stop`    | `<name>`    | `-d/--detach`, `-f/--force`                 |
| `connect` | `<name>`    | `-s/--ssh-key <path>`                       |
| `list`    | —           | —                                           |
| `version` | —           | —                                           |

### `config.rs`

Three public items:

- **`ProjectPaths::resolve(name: &str) -> Result<ProjectPaths>`** — builds all filesystem
  paths for a named project from `$HOME`. Returns a struct with `project_dir`, `env_file`,
  `compose_file`, `volumes_dir`, `projects_top_dir`, and `templates_dir`.

- **`EnvConfig::load(path: &Path) -> Result<EnvConfig>`** — parses a `.env` file line by
  line. Skips blank lines and comments (`#`). Splits on the first `=`. Populates typed fields
  for all known keys; stores everything in a `raw: HashMap<String, String>` for forwarding
  to compose as environment variables.

- **`validate_project_name(name: &str) -> Result<()>`** — rejects empty names and names
  containing characters outside `[a-zA-Z0-9_-]`.

`EnvConfig` typed fields:

| Field                     | Type            | `.env` key                   |
|---------------------------|-----------------|-------------------------------|
| `user`                    | `Option<String>`| `USER`                        |
| `user_id`                 | `Option<String>`| `USER_ID`                     |
| `devcontainer_image_name` | `Option<String>`| `DEVCONTAINER_IMAGE_NAME`     |
| `resources_ram`           | `Option<String>`| `RESOURCES_RAM`               |
| `hostname`                | `Option<String>`| `HOSTNAME`                    |
| `ssh_port`                | `Option<u16>`   | `SSH_PORT`                    |
| `enable_ssh`              | `Option<bool>`  | `ENABLE_SSH`                  |

### `docker.rs`

Thin wrappers around `std::process::Command`. No business logic. Returns `anyhow::Result`.

| Function             | Description                                              |
|----------------------|----------------------------------------------------------|
| `check_docker()`     | Verify `docker version` and `docker compose version` succeed |
| `compose_up()`       | `docker compose -f ... -p <name> up -d`                  |
| `compose_down()`     | `docker compose -p <name> down [--timeout 0]`            |
| `compose_config()`   | `docker compose -f ... config` → returns stdout string   |
| `compose_ls_json()`  | `docker compose ls --format json` → returns stdout string|
| `container_id()`     | `docker ps -q --filter name=<name>-maindevcontainer-1`   |
| `docker_cp()`        | `docker cp <src> <dest>`                                 |
| `docker_exec()`      | `docker exec <container> <args...>` → returns `Output`   |

`compose_up` and `compose_down` use `Stdio::inherit` so the user sees live docker output.
`compose_config` and `compose_ls_json` capture stdout for processing.

### `commands/new.rs`

1. `validate_project_name`
2. Check project existence; bail if exists and `!force`
3. `fs::create_dir_all` for project directory and `volumes/`
4. Write `.env` template (embedded `const &str` in source, matches SPECS exactly)
5. Write `docker-compose.yml` template (embedded `const &str`, extended overlay format)
6. Print paths of created files

Templates are embedded as string literals — the binary carries them internally. No template
files need to exist on disk.

### `commands/edit.rs`

1. Validate name, check project exists, check target file exists
2. Resolve target: `.env` by default, `docker-compose.yml` if `--compose`
3. Open in editor: try `$VISUAL`, then `$EDITOR`, then `xdg-open`, then `open`, then error
4. After editor exits: if `!no_reload`, call `compose_up` if the container is currently running
   (detected via `docker::container_id`)

### `commands/inspect.rs`

1. Validate name, check project and both config files exist
2. `EnvConfig::load` → format all known fields as a table, mark unset fields as `(not set)`
3. `docker::compose_config` → print the merged compose YAML below the env section
4. If the base template is missing, emit a warning and show the project overlay only

### `commands/run.rs`

1. Validate name, check project and config files exist
2. `EnvConfig::load`
3. If `--ssh-key` given: check file exists, validate public key type prefix
4. `create_volumes` — ensure `volumes/workdir` and `volumes/<USER>` exist
5. Build compose env map: `PROJECTS_TOP_DIR`, `PROJECT_NAME`, `SSH_PORT`, plus all `raw` values
6. `docker::compose_up` with base template + project overlay
7. `docker::container_id` to retrieve the running container
8. If SSH key provided: `docker cp` key into container, `docker exec sh -c` to append to
   `authorized_keys` and set permissions
9. Print startup summary

**SSH public key validation** is a prefix check against known key types:
`ssh-rsa`, `ssh-ed25519`, `ecdsa-sha2-nistp256/384/521`,
`sk-ssh-ed25519@openssh.com`, `sk-ecdsa-sha2-nistp256@openssh.com`.
A valid key must have at least two whitespace-separated tokens (type + base64 data).
No cryptographic verification is performed.

**Post-start hooks** (dotfile installation, sshd start, zoxide) from the original bash script
are intentionally omitted. `run` is pure infrastructure. Image-level initialisation belongs
in the container image's entrypoint or init system.

### `commands/stop.rs`

1. Validate name, check project exists
2. `docker::compose_down(name, force, wait=!detach, env_vars)`
   - `force=true` passes `--timeout 0` to docker compose
   - `wait=true` (default) blocks until compose reports down

### `commands/connect.rs`

1. Validate name, check project exists, check `.env` exists
2. `EnvConfig::load` — warn to stderr if `HOSTNAME` missing (use `localhost`),
   warn if `SSH_PORT` missing (use `22224`)
3. Build `ssh` invocation: `-A -p <port> -o StrictHostKeyChecking=no
   -o UserKnownHostsFile=/dev/null [-i <key>] [<user>@]<host>`
4. `Command::exec()` — replaces the current process with ssh so the terminal is a proper PTY.
   This call never returns on success; any returned error is fatal.

`StrictHostKeyChecking=no` is deliberate: devcontainers are ephemeral; host key checking
provides no security benefit and actively breaks reconnects after a container is recreated.

### `commands/list.rs`

1. `docker::check_docker()` — non-fatal; emits a warning and degrades to "unknown" status
   if Docker is unavailable
2. Read all subdirectories of `projects_top_dir`
3. `docker::compose_ls_json()` — parse JSON with a hand-rolled object splitter and field
   extractor (no `serde` dependency). Builds a `HashMap<project_name, status_string>`
4. For each project dir: load `.env` for image and port, look up status in the map
5. Print a dynamically-sized ASCII table with columns: `NAME`, `STATUS`, `SSH_PORT`, `IMAGE`

The JSON parser (`split_json_objects` + `extract_json_string_field`) handles only the known
schema of `docker compose ls --format json` output. It is not a general-purpose JSON parser.

---

## Error Handling

- All commands return `anyhow::Result<()>`.
- `main` prints `error: {:#}` to stderr and exits with code 1 on any error.
- Warnings are printed to stderr with the prefix `warning:` and do not abort execution.
- `unwrap()` and `expect()` are banned in production paths.
- Docker subprocess failures include the exit code or captured stderr in the error message.

---

## Binary Size Strategy

`Cargo.toml` release profile:

```toml
[profile.release]
opt-level = "z"      # optimise for size, not speed
lto = true           # link-time optimisation removes dead code across crates
codegen-units = 1    # single codegen unit enables maximum LTO
panic = "abort"      # removes panic unwinding machinery
strip = true         # strip debug symbols from the final binary
```

`clap` is included with `default-features = false`, enabling only `derive`, `std`, `help`,
`usage`, and `error-context`. Color support, environment variable parsing, and shell completion
are not included.

`anyhow` is included with `default-features = false, features = ["std"]`.

There is no async runtime, no serialisation library, and no HTTP client.

---

## Testing Strategy

Tests are external integration tests in `tests/`. Each test:

1. Constructs a `TestEnv` which creates a temporary directory under
   `CARGO_TARGET_TMPDIR` (or `/workdir/target/tmp` as fallback) to avoid `noexec` mount
   issues
2. Sets `HOME` to the temp directory so all path resolution is isolated
3. Invokes the compiled binary via `std::process::Command` using `CARGO_BIN_EXE_dev-container`
4. Asserts on exit code, stdout content, and stderr content
5. Drops the `TempDir` on scope exit (automatic cleanup)

Tests that require a running Docker daemon are annotated `#[ignore]` and run with
`cargo test -- --include-ignored` (or `make test-docker`).

Non-Docker tests cover: argument parsing, project name validation, file creation, template
content, editor invocation, SSH key validation, env file parsing, table output format, and
all error paths that do not require a live container.

---

## Makefile Targets

| Target          | Description                                               |
|-----------------|-----------------------------------------------------------|
| `make build`    | `cargo build --release`                                   |
| `make build-dev`| `cargo build` (debug profile)                             |
| `make build-static` | Static musl binary (`x86_64-unknown-linux-musl`)      |
| `make test`     | Build release then run all non-Docker tests               |
| `make test-verbose` | Same with `--nocapture`                               |
| `make test-docker`  | Run all tests including `#[ignore]`-gated Docker tests|
| `make check`    | `cargo check` (no binary produced)                        |
| `make fmt`      | `cargo fmt`                                               |
| `make lint`     | `cargo clippy -- -D warnings`                             |
| `make install`  | Copy release binary to `~/.local/bin/dev-container`       |
| `make uninstall`| Remove `~/.local/bin/dev-container`                       |
| `make clean`    | `cargo clean`                                             |

---

## Known Limitations and Future Work

- **Remote host support via SSH tunnel**: SPECS mentions the possibility of proxying through
  an SSH tunnel for remote Docker contexts. This is not implemented. The current design
  passes `HOSTNAME` to `connect` for SSH access but does not tunnel `docker compose` commands
  through SSH. This would require a `DOCKER_HOST=ssh://user@remote` env var approach or
  explicit SSH tunnel management.

- **`run` post-start hooks**: The original bash script installed dotfiles, zoxide, and started
  sshd inside the container. These are image-specific concerns and have been removed. If this
  behaviour is needed, it should live in the container image's entrypoint.

- **`edit` container reload**: `edit` without `--no-reload` will call `compose up` if the
  container is running. This performs a rolling restart, not a full `down`/`up` cycle. This
  may not pick up all configuration changes (e.g., volume changes require a full restart).

- **Windows support**: `commands/connect.rs` uses `std::os::unix::process::CommandExt::exec`.
  The binary is Linux/macOS only by design.