mod common;

use common::{TestEnv, assert_failure, assert_stderr_contains};

#[test]
fn stop_fails_without_name() {
    let env = TestEnv::new();
    let out = env.run(&["stop"]);
    assert_failure(&out);
}

#[test]
fn stop_fails_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["stop", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn stop_fails_on_invalid_project_name() {
    let env = TestEnv::new();
    let out = env.run(&["stop", "bad name!"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "invalid character");
}

#[test]
fn stop_detach_flag_accepted_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["stop", "--detach", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn stop_force_flag_accepted_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["stop", "--force", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn stop_detach_short_flag_accepted_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["stop", "-d", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn stop_force_short_flag_accepted_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["stop", "-f", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn stop_detach_and_force_combined_on_nonexistent_project() {
    let env = TestEnv::new();
    let out = env.run(&["stop", "--detach", "--force", "ghost"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "does not exist");
}

#[test]
fn stop_validates_name_before_checking_existence() {
    let env = TestEnv::new();
    let out = env.run(&["stop", "inv@lid"]);
    assert_failure(&out);
    assert_stderr_contains(&out, "invalid character");
}

#[test]
fn stop_project_exists_but_not_running_does_not_panic() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run(&["stop", "myproject"]);
    assert_failure(&out);
}

#[test]
fn stop_project_exists_but_not_running_with_force_does_not_panic() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run(&["stop", "--force", "myproject"]);
    assert_failure(&out);
}

#[test]
fn stop_project_exists_but_not_running_with_detach_does_not_panic() {
    let env = TestEnv::new();
    env.run(&["new", "myproject"]);

    let out = env.run(&["stop", "--detach", "myproject"]);
    assert_failure(&out);
}
