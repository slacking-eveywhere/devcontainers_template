mod common;

use common::{TestEnv, assert_failure, assert_stderr_contains, assert_stdout_contains};

#[test]
fn inspect_fails_without_name() {
    let env = TestEnv::new();
    let out = env.run(&["inspect"]);
    assert_failure(&out);
}

#[test]
fn inspect_fails_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["inspect", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn inspect_fails_on_invalid_project_name() {
    let env = TestEnv::new();
    let out = env.run(&["inspect", "bad name!"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "invalid character");
}

#[test]
fn inspect_fails_when_env_file_missing() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::remove_file(env.env_file("myproject")).expect("failed to remove .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "missing configuration file");
}

#[test]
fn inspect_fails_when_compose_file_missing() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::remove_file(env.compose_file("myproject"))
        .expect("failed to remove docker-compose.yml");

    let out = env.run(&["inspect", "myproject"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "missing configuration file");
}

#[test]
fn inspect_displays_project_name() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let env_path = env.env_file("myproject");
    std::fs::write(
        &env_path,
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "myproject");
}

#[test]
fn inspect_displays_image_from_env() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "ubuntu:22.04");
}

#[test]
fn inspect_displays_ssh_port_from_env() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=33100\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "33100");
}

#[test]
fn inspect_displays_user_from_env() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "devuser");
}

#[test]
fn inspect_shows_not_set_for_missing_image() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "(not set)");
}

#[test]
fn inspect_displays_hostname_when_set() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nHOSTNAME=remote.example.com\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "remote.example.com");
}

#[test]
fn inspect_shows_localhost_default_when_hostname_missing() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "localhost");
}

#[test]
fn inspect_displays_enable_ssh_true() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "true");
}

#[test]
fn inspect_displays_enable_ssh_false() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=22224\nENABLE_SSH=false\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "false");
}

#[test]
fn inspect_displays_project_path() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "myproject");
}

#[test]
fn inspect_ignores_comments_in_env_file() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "# This is a comment\nUSER=devuser\n# Another comment\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=alpine:3.18\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "devuser");
    assert_stdout_contains(&out, "alpine:3.18");
}

#[test]
fn inspect_handles_env_file_with_blank_lines() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "\nUSER=devuser\n\nUSER_ID=1000\n\nDEVCONTAINER_IMAGE_NAME=debian:12\n\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["inspect", "myproject"]);
    assert_stdout_contains(&out, "devuser");
    assert_stdout_contains(&out, "debian:12");
}
