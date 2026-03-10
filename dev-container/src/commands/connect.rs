use std::os::unix::process::CommandExt;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::config::{EnvConfig, ProjectPaths, project_exists, validate_project_name};

const DEFAULT_SSH_PORT: u16 = 22224;
const DEFAULT_HOST: &str = "localhost";

pub fn execute(name: &str, ssh_key: Option<&str>) -> Result<()> {
    validate_project_name(name)?;

    let paths = ProjectPaths::resolve(name)?;

    if !project_exists(&paths) {
        bail!("project '{}' does not exist", name);
    }

    if !paths.env_file.exists() {
        bail!(
            "missing configuration file: {}; project may be corrupted",
            paths.env_file.display()
        );
    }

    let env = EnvConfig::load(&paths.env_file).context("failed to parse .env file")?;

    let host = match &env.hostname {
        Some(h) if !h.is_empty() => h.clone(),
        _ => {
            eprintln!(
                "warning: HOSTNAME not set in .env; using default '{}'",
                DEFAULT_HOST
            );
            DEFAULT_HOST.to_string()
        }
    };

    let port = match env.ssh_port {
        Some(p) => p,
        None => {
            eprintln!(
                "warning: SSH_PORT not set in .env; using default '{}'",
                DEFAULT_SSH_PORT
            );
            DEFAULT_SSH_PORT
        }
    };

    let user = env.user.as_deref().unwrap_or("");

    let destination = if user.is_empty() {
        host.clone()
    } else {
        format!("{}@{}", user, host)
    };

    let mut cmd = Command::new("ssh");

    cmd.arg("-A");
    cmd.args(["-p", &port.to_string()]);
    cmd.args(["-o", "StrictHostKeyChecking=no"]);
    cmd.args(["-o", "UserKnownHostsFile=/dev/null"]);

    if let Some(key) = ssh_key {
        let key_path = std::path::Path::new(key);
        if !key_path.exists() {
            bail!("SSH private key file not found: {}", key);
        }
        cmd.arg("-i");
        cmd.arg(key);
    }

    cmd.arg(&destination);

    let err = cmd.exec();

    Err(err).with_context(|| format!("failed to exec ssh to '{}' on port {}", destination, port))
}
