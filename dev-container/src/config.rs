use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

const DEFAULT_BASE_DIR: &str = ".local/share/devcontainer";
const PROJECTS_DIR: &str = "projects";
const TEMPLATES_DIR: &str = "templates";

pub struct ProjectPaths {
    pub project_dir: PathBuf,
    pub env_file: PathBuf,
    pub compose_file: PathBuf,
    pub volumes_dir: PathBuf,
    pub projects_top_dir: PathBuf,
    pub templates_dir: PathBuf,
}

impl ProjectPaths {
    pub fn resolve(name: &str) -> Result<Self> {
        let home = home_dir()?;
        let base = home.join(DEFAULT_BASE_DIR);
        let projects_top_dir = base.join(PROJECTS_DIR);
        let templates_dir = base.join(TEMPLATES_DIR);
        let project_dir = projects_top_dir.join(name);

        Ok(Self {
            env_file: project_dir.join(".env"),
            compose_file: project_dir.join("docker-compose.yml"),
            volumes_dir: project_dir.join("volumes"),
            project_dir,
            projects_top_dir,
            templates_dir,
        })
    }
}

pub fn home_dir() -> Result<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME environment variable is not set")
}

pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("project name cannot be empty");
    }

    let invalid = name
        .chars()
        .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'));

    if let Some(c) = invalid {
        bail!(
            "project name contains invalid character '{}'; only letters, digits, '-' and '_' are allowed",
            c
        );
    }

    Ok(())
}

pub fn project_exists(paths: &ProjectPaths) -> bool {
    paths.project_dir.is_dir()
}

#[derive(Debug, Default)]
pub struct EnvConfig {
    pub user: Option<String>,
    pub user_id: Option<String>,
    pub devcontainer_image_name: Option<String>,
    pub resources_ram: Option<String>,
    pub hostname: Option<String>,
    pub ssh_port: Option<u16>,
    pub enable_ssh: Option<bool>,
    pub raw: HashMap<String, String>,
}

impl EnvConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read env file: {}", path.display()))?;

        let mut cfg = EnvConfig::default();

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let Some(eq_pos) = trimmed.find('=') else {
                continue;
            };

            let key = trimmed[..eq_pos].trim().to_string();
            let value = trimmed[eq_pos + 1..].trim().to_string();

            if key.is_empty() {
                continue;
            }

            match key.as_str() {
                "USER" if !value.is_empty() => cfg.user = Some(value.clone()),
                "USER_ID" if !value.is_empty() => cfg.user_id = Some(value.clone()),
                "DEVCONTAINER_IMAGE_NAME" if !value.is_empty() => {
                    cfg.devcontainer_image_name = Some(value.clone())
                }
                "RESOURCES_RAM" if !value.is_empty() => cfg.resources_ram = Some(value.clone()),
                "HOSTNAME" if !value.is_empty() => cfg.hostname = Some(value.clone()),
                "SSH_PORT" if !value.is_empty() => {
                    cfg.ssh_port = value.parse::<u16>().ok();
                }
                "ENABLE_SSH" if !value.is_empty() => {
                    cfg.enable_ssh = match value.to_lowercase().as_str() {
                        "true" | "1" | "yes" => Some(true),
                        "false" | "0" | "no" => Some(false),
                        _ => None,
                    };
                }
                _ => {}
            }

            if !value.is_empty() {
                cfg.raw.insert(key, value);
            }
        }

        Ok(cfg)
    }
}
