mod common;

use common::{TestEnv, assert_stdout_contains, assert_success};

#[test]
fn list_succeeds_with_no_projects() {
    let env = TestEnv::new();
    let out = env.run(&["list"]);
    assert_success(&out);
}

#[test]
fn list_prints_table_header_when_empty() {
    let env = TestEnv::new();
    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "NAME");
    assert_stdout_contains(&out, "STATUS");
    assert_stdout_contains(&out, "SSH_PORT");
    assert_stdout_contains(&out, "IMAGE");
}

#[test]
fn list_shows_no_projects_placeholder_when_empty() {
    let env = TestEnv::new();
    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "(no projects)");
}

#[test]
fn list_shows_project_name_after_new() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);
    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "myproject");
}

#[test]
fn list_shows_stopped_status_for_new_project() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);
    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "stopped");
}

#[test]
fn list_shows_image_from_env_file() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=22224\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "ubuntu:22.04");
}

#[test]
fn list_shows_ssh_port_from_env_file() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::write(
        env.env_file("myproject"),
        "USER=devuser\nUSER_ID=1000\nDEVCONTAINER_IMAGE_NAME=ubuntu:22.04\nSSH_PORT=33100\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "33100");
}

#[test]
fn list_shows_default_ssh_port_when_not_set() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "22224");
}

#[test]
fn list_shows_multiple_projects() {
    let env = TestEnv::new();
    env.run(&["new", "alpha"]);
    env.run(&["new", "beta"]);
    env.run(&["new", "gamma"]);

    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "alpha");
    assert_stdout_contains(&out, "beta");
    assert_stdout_contains(&out, "gamma");
}

#[test]
fn list_shows_not_set_for_missing_image() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "(not set)");
}

#[test]
fn list_table_has_separator_lines() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run(&["list"]);
    assert_success(&out);
    let stdout = common::stdout(&out);
    let separator_lines: Vec<&str> = stdout
        .lines()
        .filter(|l| l.starts_with('+') && l.ends_with('+'))
        .collect();
    assert!(
        separator_lines.len() >= 2,
        "expected at least 2 table separator lines, got:\n{}",
        stdout
    );
}

#[test]
fn list_project_with_all_env_fields_set() {
    let env = TestEnv::new();
    env.run(&["new", "fullproject"]);

    std::fs::write(
        env.env_file("fullproject"),
        "USER=alice\nUSER_ID=1001\nDEVCONTAINER_IMAGE_NAME=rust:1.83\nRESOURCES_RAM=4G\nHOSTNAME=devbox.local\nSSH_PORT=22300\nENABLE_SSH=true\n",
    )
    .expect("failed to write .env");

    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "fullproject");
    assert_stdout_contains(&out, "rust:1.83");
    assert_stdout_contains(&out, "22300");
}

#[test]
fn list_project_with_no_env_file_shows_placeholder() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::remove_file(env.env_file("myproject")).expect("failed to remove .env");

    let out = env.run(&["list"]);
    assert_success(&out);
    assert_stdout_contains(&out, "myproject");
    assert_stdout_contains(&out, "(no .env)");
}
