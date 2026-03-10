use std::collections::HashMap;

use anyhow::{Context, Result, bail};

use crate::config::{EnvConfig, ProjectPaths, project_exists, validate_project_name};
use crate::docker;

pub fn execute(name: &str) -> Result<()> {
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

    if !paths.compose_file.exists() {
        bail!(
            "missing configuration file: {}; project may be corrupted",
            paths.compose_file.display()
        );
    }

    let env = EnvConfig::load(&paths.env_file).context("failed to parse project .env file")?;

    print_env_section(name, &paths, &env);
    print_compose_section(name, &paths)?;

    Ok(())
}

fn print_env_section(name: &str, paths: &ProjectPaths, env: &EnvConfig) {
    let separator = "─".repeat(60);

    println!("{}", separator);
    println!("  Project : {}", name);
    println!("  Path    : {}", paths.project_dir.display());
    println!("{}", separator);

    println!();
    println!("  [Environment Configuration]");
    println!();

    print_field("USER", env.user.as_deref());
    print_field("USER_ID", env.user_id.as_deref());
    print_field("IMAGE", env.devcontainer_image_name.as_deref());
    print_field("RESOURCES_RAM", env.resources_ram.as_deref());
    print_field(
        "SSH_PORT",
        env.ssh_port.as_ref().map(|p| p.to_string()).as_deref(),
    );
    print_field(
        "HOSTNAME",
        env.hostname.as_deref().or(Some("localhost (default)")),
    );
    print_field(
        "ENABLE_SSH",
        env.enable_ssh
            .as_ref()
            .map(|b| if *b { "true" } else { "false" })
            .or(Some("true (default)")),
    );

    println!();

    let volumes_dir = &paths.volumes_dir;
    if volumes_dir.is_dir() {
        println!("  [Volumes]");
        println!();

        match std::fs::read_dir(volumes_dir) {
            Ok(entries) => {
                let mut found = false;
                for entry in entries.flatten() {
                    println!("    {}", entry.path().display());
                    found = true;
                }
                if !found {
                    println!("    (no volume directories created yet)");
                }
            }
            Err(e) => println!("    (could not read volumes dir: {})", e),
        }

        println!();
    }
}

fn print_field(label: &str, value: Option<&str>) {
    match value {
        Some(v) if !v.is_empty() => println!("    {:<22} {}", format!("{}:", label), v),
        _ => println!("    {:<22} (not set)", format!("{}:", label)),
    }
}

fn print_compose_section(name: &str, paths: &ProjectPaths) -> Result<()> {
    let separator = "─".repeat(60);

    println!("  [Merged Compose Configuration]");
    println!("{}", separator);

    let mut env_vars: HashMap<String, String> = HashMap::new();
    env_vars.insert(
        "PROJECTS_TOP_DIR".to_string(),
        paths.projects_top_dir.to_string_lossy().into_owned(),
    );
    env_vars.insert("PROJECT_NAME".to_string(), name.to_string());

    let base_compose = paths.templates_dir.join("docker-compose-ssh.yml");
    let compose_files: Vec<&std::path::Path> = if base_compose.exists() {
        vec![base_compose.as_path(), paths.compose_file.as_path()]
    } else {
        eprintln!(
            "warning: base template not found at {}; showing project compose only",
            base_compose.display()
        );
        vec![paths.compose_file.as_path()]
    };

    let config_output = docker::compose_config(name, &compose_files, &env_vars)
        .context("failed to retrieve merged compose configuration")?;

    for line in config_output.lines() {
        println!("  {}", line);
    }

    println!("{}", separator);

    Ok(())
}
