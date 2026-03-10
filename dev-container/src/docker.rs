use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use anyhow::{Context, Result, bail};

pub fn check_docker() -> Result<()> {
    let status = Command::new("docker")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("docker is not installed or not in PATH")?;

    if !status.success() {
        bail!("docker is not running or not accessible");
    }

    let status = Command::new("docker")
        .args(["compose", "version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("docker compose plugin is not installed")?;

    if !status.success() {
        bail!("docker compose v2 plugin is not available; please update Docker");
    }

    Ok(())
}

pub fn compose_up(
    project_name: &str,
    compose_files: &[&Path],
    env_vars: &HashMap<String, String>,
) -> Result<()> {
    let mut cmd = base_compose_cmd(project_name, compose_files, env_vars);
    cmd.args(["up", "-d"]);

    run_passthrough(&mut cmd, "docker compose up")
}

pub fn compose_down(
    project_name: &str,
    force: bool,
    wait: bool,
    env_vars: &HashMap<String, String>,
) -> Result<()> {
    let mut cmd = Command::new("docker");
    cmd.args(["compose", "-p", project_name]);

    for (k, v) in env_vars {
        cmd.env(k, v);
    }

    cmd.arg("down");

    if force {
        cmd.args(["--timeout", "0"]);
    }

    if !wait {
        cmd.arg("--timeout=0");
    }

    run_passthrough(&mut cmd, "docker compose down")
}

pub fn compose_config(
    project_name: &str,
    compose_files: &[&Path],
    env_vars: &HashMap<String, String>,
) -> Result<String> {
    let mut cmd = base_compose_cmd(project_name, compose_files, env_vars);
    cmd.arg("config");

    let output = cmd
        .output()
        .context("failed to execute docker compose config")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("docker compose config failed: {}", stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn compose_ls_json() -> Result<String> {
    let output = Command::new("docker")
        .args(["compose", "ls", "--format", "json"])
        .output()
        .context("failed to execute docker compose ls")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("docker compose ls failed: {}", stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn container_id(project_name: &str) -> Result<String> {
    let filter = format!("name={}-maindevcontainer-1", project_name);
    let output = Command::new("docker")
        .args(["ps", "-q", "--filter", &filter])
        .output()
        .context("failed to execute docker ps")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("docker ps failed: {}", stderr.trim());
    }

    let id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if id.is_empty() {
        bail!(
            "no running container found for project '{}'; is it started?",
            project_name
        );
    }

    Ok(id)
}

pub fn docker_cp(src: &Path, dest: &str) -> Result<()> {
    let status = Command::new("docker")
        .arg("cp")
        .arg(src)
        .arg(dest)
        .status()
        .context("failed to execute docker cp")?;

    if !status.success() {
        bail!("docker cp failed: {} -> {}", src.display(), dest);
    }

    Ok(())
}

pub fn docker_exec<I, S>(container_id: &str, args: I) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("docker")
        .arg("exec")
        .arg(container_id)
        .args(args)
        .output()
        .context("failed to execute docker exec")?;

    Ok(output)
}

fn base_compose_cmd(
    project_name: &str,
    compose_files: &[&Path],
    env_vars: &HashMap<String, String>,
) -> Command {
    let mut cmd = Command::new("docker");
    cmd.arg("compose");

    for file in compose_files {
        cmd.arg("-f");
        cmd.arg(file);
    }

    cmd.args(["-p", project_name]);

    for (k, v) in env_vars {
        cmd.env(k, v);
    }

    cmd
}

fn run_passthrough(cmd: &mut Command, context: &'static str) -> Result<()> {
    let status = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("failed to execute {}", context))?;

    if !status.success() {
        bail!(
            "{} exited with status {}",
            context,
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}
