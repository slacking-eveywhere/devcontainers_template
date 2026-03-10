mod common;

use common::{TestEnv, assert_failure, assert_stderr_contains};

#[test]
fn connect_fails_without_name() {
    let env = TestEnv::new();
    let out = env.run(&["connect"]);
    assert_failure(&out);
}

#[test]
fn connect_fails_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["connect", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn connect_fails_on_invalid_project_name() {
    let env = TestEnv::new();
    let out = env.run(&["connect", "bad name!"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "invalid character");
}

#[test]
fn connect_fails_when_env_file_missing() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::remove_file(env.env_file("myproject")).expect("failed to remove .env");

    let out = env.run(&["connect", "myproject"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "missing configuration file");
}

#[test]
fn connect_fails_when_ssh_key_file_does_not_exist() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nHOSTNAME=localhost\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&[
        "connect",
        "myproject",
        "--ssh-key",
        "/nonexistent/path/id_ed25519",
    ]);
    assert_failure(&out);
    assert_stderr_contains(&out, "not found");
}

#[test]
fn connect_ssh_key_short_flag_validates_existence() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nHOSTNAME=localhost\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["connect", "myproject", "-s", "/nonexistent/path/id_ed25519"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "not found");
}

#[test]
fn connect_warns_when_hostname_not_set_in_env() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["connect", "myproject"]);
    let stderr = common::stderr(&out);
    assert!(
        stderr.contains("warning") && stderr.contains("HOSTNAME"),
        "expected warning about missing HOSTNAME, got stderr: {}",
        stderr
    );
}

#[test]
fn connect_warns_when_ssh_port_not_set_in_env() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nHOSTNAME=localhost\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["connect", "myproject"]);
    let stderr = common::stderr(&out);
    assert!(
        stderr.contains("warning") && stderr.contains("SSH_PORT"),
        "expected warning about missing SSH_PORT, got stderr: {}",
        stderr
    );
}

#[test]
fn connect_does_not_warn_when_hostname_is_set() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nHOSTNAME=remote.example.com\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["connect", "myproject"]);
    let stderr = common::stderr(&out);
    assert!(
        !stderr.contains("HOSTNAME"),
        "expected no HOSTNAME warning when hostname is set, got stderr: {}",
        stderr
    );
}

#[test]
fn connect_does_not_warn_when_ssh_port_is_set() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nHOSTNAME=localhost\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["connect", "myproject"]);
    let stderr = common::stderr(&out);
    assert!(
        !stderr.contains("SSH_PORT"),
        "expected no SSH_PORT warning when port is set, got stderr: {}",
        stderr
    );
}

#[test]
fn connect_uses_default_port_when_ssh_port_missing() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nHOSTNAME=localhost\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["connect", "myproject"]);
    let stderr = common::stderr(&out);
    assert!(
        stderr.contains("22224"),
        "expected default port 22224 in warning, got stderr: {}",
        stderr
    );
}

#[test]
fn connect_uses_default_hostname_when_hostname_missing() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["connect", "myproject"]);
    let stderr = common::stderr(&out);
    assert!(
        stderr.contains("localhost"),
        "expected 'localhost' default in warning, got stderr: {}",
        stderr
    );
}

#[test]
fn connect_validates_name_before_checking_existence() {
    let env = TestEnv::new();
    let out = env.run(&["connect", "inv@lid"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "invalid character");
}

#[test]
fn connect_fails_with_empty_env_file() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(env.env_file("myproject"), "").expect("failed to write empty .env");

    let out = env.run(&["connect", "myproject"]);
    let stderr = common::stderr(&out);
    assert!(
        stderr.contains("warning") || !out.status.success(),
        "expected either warnings or failure on empty .env, got stderr: {}",
        stderr
    );
}

#[test]
fn connect_parses_ssh_port_correctly_from_env() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nHOSTNAME=localhost\nSSH_PORT=33300\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["connect", "myproject"]);
    let stderr = common::stderr(&out);
    assert!(
        !stderr.contains("SSH_PORT"),
        "expected no SSH_PORT warning when port 33300 is explicitly set, got: {}",
        stderr
    );
}
