mod common;

use common::{
    TestEnv, assert_failure, assert_stderr_contains, assert_stdout_contains, assert_success,
};

#[test]
fn new_creates_project_directory() {
    let env = TestEnv::new();
    let out = env.run(&["new", "myproject"]);
    assert_success(&out);
    assert!(env.project_dir("myproject").is_dir());
}

#[test]
fn new_creates_env_file() {
    let env = TestEnv::new();
    let out = env.run(&["new", "myproject"]);
    assert_success(&out);
    assert!(env.env_file("myproject").exists());
}

#[test]
fn new_creates_compose_file() {
    let env = TestEnv::new();
    let out = env.run(&["new", "myproject"]);
    assert_success(&out);
    assert!(env.compose_file("myproject").exists());
}

#[test]
fn new_creates_volumes_directory() {
    let env = TestEnv::new();
    let out = env.run(&["new", "myproject"]);
    assert_success(&out);
    assert!(env.volumes_dir("myproject").is_dir());
}

#[test]
fn new_env_file_contains_expected_keys() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);
    let content =
        std::fs::read_to_string(env.env_file("myproject")).expect("failed to read .env file");
    assert!(content.contains("USER="));
    assert!(content.contains("USER_ID="));
    assert!(content.contains("DEVCONTAINER_IMAGE_NAME="));
    assert!(content.contains("RESOURCES_RAM="));
    assert!(content.contains("HOSTNAME="));
    assert!(content.contains("SSH_PORT="));
    assert!(content.contains("ENABLE_SSH=true"));
}

#[test]
fn new_compose_file_contains_expected_keys() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);
    let content = std::fs::read_to_string(env.compose_file("myproject"))
        .expect("failed to read docker-compose.yml");
    assert!(content.contains("services:"));
    assert!(content.contains("maindevcontainer:"));
    assert!(content.contains("env_file:"));
}

#[test]
fn new_prints_created_project_name() {
    let env = TestEnv::new();
    let out = env.run(&["new", "myproject"]);
    assert_success(&out);
    assert_stdout_contains(&out, "myproject");
}

#[test]
fn new_fails_without_name() {
    let env = TestEnv::new();
    let out = env.run(&["new"]);
    assert_failure(&out);
}

#[test]
fn new_fails_if_project_already_exists() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);
    let out = env.run(&["new", "myproject"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "already exists");
}

#[test]
fn new_force_overwrites_existing_project() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let env_path = env.env_file("myproject");
    std::fs::write(&env_path, "CORRUPTED=yes\n").expect("failed to corrupt .env");

    let out = env.run(&["new", "--force", "myproject"]);
    assert_success(&out);

    let content = std::fs::read_to_string(&env_path).expect("failed to read .env");
    assert!(
        content.contains("USER="),
        "expected .env to be restored after --force; got: {}",
        content
    );
    assert!(
        !content.contains("CORRUPTED=yes"),
        "expected corrupted content to be gone after --force"
    );
}

#[test]
fn new_force_short_flag_overwrites_existing_project() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let env_path = env.env_file("myproject");
    std::fs::write(&env_path, "CORRUPTED=yes\n").expect("failed to corrupt .env");

    let out = env.run(&["new", "-f", "myproject"]);
    assert_success(&out);

    let content = std::fs::read_to_string(&env_path).expect("failed to read .env");
    assert!(content.contains("USER="));
}

#[test]
fn new_force_on_nonexistent_project_still_creates_it() {
    let env = TestEnv::new();
    let out = env.run(&["new", "--force", "brandnew"]);
    assert_success(&out);
    assert!(env.project_dir("brandnew").is_dir());
    assert!(env.env_file("brandnew").exists());
    assert!(env.compose_file("brandnew").exists());
}

#[test]
fn new_rejects_name_with_special_characters() {
    let env = TestEnv::new();
    let out = env.run(&["new", "my project!"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "invalid character");
}

#[test]
fn new_rejects_name_with_slash() {
    let env = TestEnv::new();
    let out = env.run(&["new", "foo/bar"]);
    assert_failure(&out);
}

#[test]
fn new_accepts_name_with_hyphens_and_underscores() {
    let env = TestEnv::new();
    let out = env.run(&["new", "my-project_01"]);
    assert_success(&out);
    assert!(env.project_dir("my-project_01").is_dir());
}

#[test]
fn new_accepts_name_with_uppercase_letters() {
    let env = TestEnv::new();
    let out = env.run(&["new", "MyProject"]);
    assert_success(&out);
    assert!(env.project_dir("MyProject").is_dir());
}

#[test]
fn new_multiple_different_projects() {
    let env = TestEnv::new();
    assert_success(&env.run(&["new", "alpha"]));
    assert_success(&env.run(&["new", "beta"]));
    assert_success(&env.run(&["new", "gamma"]));
    assert!(env.project_dir("alpha").is_dir());
    assert!(env.project_dir("beta").is_dir());
    assert!(env.project_dir("gamma").is_dir());
}
