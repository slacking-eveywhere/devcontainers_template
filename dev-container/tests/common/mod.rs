#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub struct TestEnv {
    pub home_dir: tempfile::TempDir,
}

impl TestEnv {
    pub fn new() -> Self {
        let base = std::env::var_os("CARGO_TARGET_TMPDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/workdir/target/tmp"));

        std::fs::create_dir_all(&base).expect("failed to create tmpdir base");

        let home_dir = tempfile::Builder::new()
            .prefix("devcontainer-test-")
            .tempdir_in(&base)
            .expect("failed to create temp home dir");

        let devcontainer_dir = home_dir.path().join(".local/share/devcontainer");
        std::fs::create_dir_all(devcontainer_dir.join("projects"))
            .expect("failed to create projects dir");
        std::fs::create_dir_all(devcontainer_dir.join("templates"))
            .expect("failed to create templates dir");

        Self { home_dir }
    }

    pub fn home(&self) -> &Path {
        self.home_dir.path()
    }

    pub fn projects_dir(&self) -> PathBuf {
        self.home().join(".local/share/devcontainer/projects")
    }

    pub fn project_dir(&self, name: &str) -> PathBuf {
        self.projects_dir().join(name)
    }

    pub fn env_file(&self, name: &str) -> PathBuf {
        self.project_dir(name).join(".env")
    }

    pub fn compose_file(&self, name: &str) -> PathBuf {
        self.project_dir(name).join("docker-compose.yml")
    }

    pub fn volumes_dir(&self, name: &str) -> PathBuf {
        self.project_dir(name).join("volumes")
    }

    pub fn run(&self, args: &[&str]) -> Output {
        Command::new(bin_path())
            .args(args)
            .env("HOME", self.home())
            .env_remove("VISUAL")
            .env_remove("EDITOR")
            .output()
            .expect("failed to run dev-container binary")
    }

    pub fn run_with_env(&self, args: &[&str], extra_env: &[(&str, &str)]) -> Output {
        let mut cmd = Command::new(bin_path());
        cmd.args(args)
            .env("HOME", self.home())
            .env_remove("VISUAL")
            .env_remove("EDITOR");

        for (key, val) in extra_env {
            cmd.env(key, val);
        }

        cmd.output().expect("failed to run dev-container binary")
    }
}

pub fn bin_path() -> PathBuf {
    env!("CARGO_BIN_EXE_dev-container").into()
}

pub fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

pub fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

pub fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "expected success, got exit code {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        stdout(output),
        stderr(output),
    );
}

pub fn assert_failure(output: &Output) {
    assert!(
        !output.status.success(),
        "expected failure, got exit code {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        stdout(output),
        stderr(output),
    );
}

pub fn assert_stderr_contains(output: &Output, needle: &str) {
    let err = stderr(output);
    assert!(
        err.contains(needle),
        "expected stderr to contain {:?}\nstderr: {}",
        needle,
        err,
    );
}

pub fn assert_stdout_contains(output: &Output, needle: &str) {
    let out = stdout(output);
    assert!(
        out.contains(needle),
        "expected stdout to contain {:?}\nstdout: {}",
        needle,
        out,
    );
}
