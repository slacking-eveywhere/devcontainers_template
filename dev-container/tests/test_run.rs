mod common;

use common::{TestEnv, assert_failure, assert_stderr_contains};

#[test]
fn run_fails_without_name() {
    let env = TestEnv::new();
    let out = env.run(&["run"]);
    assert_failure(&out);
}

#[test]
fn run_fails_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["run", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn run_fails_on_invalid_project_name() {
    let env = TestEnv::new();
    let out = env.run(&["run", "bad name!"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "invalid character");
}

#[test]
fn run_fails_when_env_file_missing() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::remove_file(env.env_file("myproject")).expect("failed to remove .env");

    let out = env.run(&["run", "myproject"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "missing configuration file");
}

#[test]
fn run_fails_when_compose_file_missing() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::remove_file(env.compose_file("myproject"))
        .expect("failed to remove docker-compose.yml");

    let out = env.run(&["run", "myproject"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "missing configuration file");
}

#[test]
fn run_fails_when_ssh_key_file_does_not_exist() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run(&["run", "myproject", "--ssh-key", "/nonexistent/path/key.pub"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "not found");
}

#[test]
fn run_fails_when_ssh_key_is_not_a_public_key() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let key_path = env.home().join("bad_key.pub");
    std::fs::write(&key_path, "this is not a valid public key\n")
        .expect("failed to write fake key");

    let out = env.run(&["run", "myproject", "--ssh-key", key_path.to_str().unwrap()]);
    assert_failure(&out);
    assert_stderr_contains(&out, "valid SSH public key");
}

#[test]
fn run_fails_when_ssh_key_is_a_private_key() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let key_path = env.home().join("id_ed25519");
    std::fs::write(
        &key_path,
        "-----BEGIN OPENSSH PRIVATE KEY-----\nb3BlbnNzaC1rZXktdjEAAAAA\n-----END OPENSSH PRIVATE KEY-----\n",
    )
    .expect("failed to write fake private key");

    let out = env.run(&["run", "myproject", "--ssh-key", key_path.to_str().unwrap()]);
    assert_failure(&out);
    assert_stderr_contains(&out, "valid SSH public key");
}

#[test]
fn run_fails_when_ssh_key_is_empty_file() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let key_path = env.home().join("empty.pub");
    std::fs::write(&key_path, "").expect("failed to write empty key file");

    let out = env.run(&["run", "myproject", "--ssh-key", key_path.to_str().unwrap()]);
    assert_failure(&out);
    assert_stderr_contains(&out, "valid SSH public key");
}

#[test]
fn run_accepts_valid_ed25519_public_key_format() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let key_path = env.home().join("id_ed25519.pub");
    std::fs::write(
        &key_path,
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl user@host\n",
    )
    .expect("failed to write ed25519 public key");

    let out = env.run(&["run", "myproject", "--ssh-key", key_path.to_str().unwrap()]);

    let stderr = common::stderr(&out);
    assert!(
        !stderr.contains("valid SSH public key"),
        "key validation should have passed, but got: {}",
        stderr
    );
}

#[test]
fn run_accepts_valid_rsa_public_key_format() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let key_path = env.home().join("id_rsa.pub");
    std::fs::write(
        &key_path,
        "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC2 user@host\n",
    )
    .expect("failed to write rsa public key");

    let out = env.run(&["run", "myproject", "--ssh-key", key_path.to_str().unwrap()]);

    let stderr = common::stderr(&out);
    assert!(
        !stderr.contains("valid SSH public key"),
        "key validation should have passed, but got: {}",
        stderr
    );
}

#[test]
fn run_accepts_valid_ecdsa_public_key_format() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let key_path = env.home().join("id_ecdsa.pub");
    std::fs::write(
        &key_path,
        "ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBHuOn6e9 user@host\n",
    )
    .expect("failed to write ecdsa public key");

    let out = env.run(&["run", "myproject", "--ssh-key", key_path.to_str().unwrap()]);

    let stderr = common::stderr(&out);
    assert!(
        !stderr.contains("valid SSH public key"),
        "key validation should have passed, but got: {}",
        stderr
    );
}

#[test]
fn run_ssh_key_short_flag_accepted() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let key_path = env.home().join("id_ed25519.pub");
    std::fs::write(
        &key_path,
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl user@host\n",
    )
    .expect("failed to write ed25519 public key");

    let out = env.run(&["run", "myproject", "-s", key_path.to_str().unwrap()]);

    let stderr = common::stderr(&out);
    assert!(
        !stderr.contains("valid SSH public key"),
        "key validation should have passed with -s short flag, but got: {}",
        stderr
    );
}

#[test]
fn run_fails_when_ssh_key_has_only_whitespace() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let key_path = env.home().join("whitespace.pub");
    std::fs::write(&key_path, "   \n  \n\n").expect("failed to write whitespace-only key file");

    let out = env.run(&["run", "myproject", "--ssh-key", key_path.to_str().unwrap()]);
    assert_failure(&out);
    assert_stderr_contains(&out, "valid SSH public key");
}

#[test]
fn run_fails_when_ssh_key_missing_key_data() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let key_path = env.home().join("incomplete.pub");
    std::fs::write(&key_path, "ssh-ed25519\n").expect("failed to write incomplete key file");

    let out = env.run(&["run", "myproject", "--ssh-key", key_path.to_str().unwrap()]);
    assert_failure(&out);
    assert_stderr_contains(&out, "valid SSH public key");
}
