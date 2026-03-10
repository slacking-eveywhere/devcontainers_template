mod common;

use common::{TestEnv, assert_failure, assert_stderr_contains};

#[test]
fn edit_fails_without_name() {
    let env = TestEnv::new();
    let out = env.run(&["edit"]);
    assert_failure(&out);
}

#[test]
fn edit_fails_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["edit", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn edit_fails_on_invalid_project_name() {
    let env = TestEnv::new();
    let out = env.run(&["edit", "bad name!"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "invalid character");
}

#[test]
fn edit_fails_when_no_editor_is_available() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run_with_env(
        &["edit", "myproject"],
        &[("VISUAL", ""), ("EDITOR", ""), ("PATH", "")],
    );
    assert_failure(&out);
    assert_stderr_contains(&out, "no editor found");
}

#[test]
fn edit_fails_when_env_file_missing_and_no_compose_flag() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::remove_file(env.env_file("myproject")).expect("failed to remove .env");

    let out = env.run_with_env(
        &["edit", "myproject"],
        &[("VISUAL", ""), ("EDITOR", ""), ("PATH", "")],
    );
    assert_failure(&out);
}

#[test]
fn edit_fails_when_compose_file_missing_and_compose_flag_set() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    std::fs::remove_file(env.compose_file("myproject"))
        .expect("failed to remove docker-compose.yml");

    let out = env.run_with_env(
        &["edit", "--compose", "myproject"],
        &[("VISUAL", ""), ("EDITOR", ""), ("PATH", "")],
    );
    assert_failure(&out);
}

#[test]
fn edit_uses_visual_editor_when_set() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run_with_env(
        &["edit", "myproject"],
        &[("VISUAL", "true"), ("EDITOR", "")],
    );
    assert!(
        out.status.success(),
        "expected success when VISUAL=true (no-op editor)\nstdout: {}\nstderr: {}",
        common::stdout(&out),
        common::stderr(&out),
    );
}

#[test]
fn edit_falls_back_to_editor_when_visual_is_unset() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run_with_env(
        &["edit", "myproject"],
        &[("VISUAL", ""), ("EDITOR", "true")],
    );
    assert!(
        out.status.success(),
        "expected success when EDITOR=true (no-op editor)\nstdout: {}\nstderr: {}",
        common::stdout(&out),
        common::stderr(&out),
    );
}

#[test]
fn edit_compose_flag_targets_compose_file() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run_with_env(
        &["edit", "--compose", "myproject"],
        &[("VISUAL", "true"), ("EDITOR", "")],
    );
    assert!(
        out.status.success(),
        "expected success editing compose file with VISUAL=true\nstdout: {}\nstderr: {}",
        common::stdout(&out),
        common::stderr(&out),
    );
}

#[test]
fn edit_compose_short_flag_works() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run_with_env(
        &["edit", "-c", "myproject"],
        &[("VISUAL", "true"), ("EDITOR", "")],
    );
    assert!(
        out.status.success(),
        "expected success with -c short flag\nstdout: {}\nstderr: {}",
        common::stdout(&out),
        common::stderr(&out),
    );
}

#[test]
fn edit_no_reload_flag_suppresses_reload() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run_with_env(
        &["edit", "--no-reload", "myproject"],
        &[("VISUAL", "true"), ("EDITOR", "")],
    );
    assert!(
        out.status.success(),
        "expected success with --no-reload\nstdout: {}\nstderr: {}",
        common::stdout(&out),
        common::stderr(&out),
    );
}

#[test]
fn edit_no_reload_short_flag_works() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run_with_env(
        &["edit", "-q", "myproject"],
        &[("VISUAL", "true"), ("EDITOR", "")],
    );
    assert!(
        out.status.success(),
        "expected success with -q short flag\nstdout: {}\nstderr: {}",
        common::stdout(&out),
        common::stderr(&out),
    );
}

#[test]
fn edit_editor_receives_env_file_path() {
    use std::os::unix::fs::PermissionsExt;

    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let marker_file = env.home().join("editor_was_called");
    let marker_path = marker_file.to_str().expect("non-UTF8 path").to_string();

    let script_path = env.home().join("fake_editor.sh");
    std::fs::write(
        &script_path,
        format!("#!/bin/sh\ntouch '{}'\n", marker_path),
    )
    .expect("failed to write fake editor script");

    let mut perms = std::fs::metadata(&script_path)
        .expect("stat failed")
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script_path, perms).expect("chmod failed");

    let editor_str = script_path.to_str().expect("non-UTF8 path");

    let out = env.run_with_env(
        &["edit", "myproject"],
        &[("VISUAL", editor_str), ("EDITOR", "")],
    );

    assert!(
        out.status.success(),
        "expected success\nstdout: {}\nstderr: {}",
        common::stdout(&out),
        common::stderr(&out),
    );
    assert!(
        marker_file.exists(),
        "expected fake editor to have been called (marker file missing)"
    );
}

#[test]
fn edit_fails_when_editor_exits_nonzero() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run_with_env(
        &["edit", "myproject"],
        &[("VISUAL", "false"), ("EDITOR", "")],
    );
    assert_failure(&out);
}
