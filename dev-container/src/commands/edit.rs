use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::config::{ProjectPaths, project_exists, validate_project_name};
use crate::docker;

pub fn execute(name: &str, compose: bool, no_reload: bool) -> Result<()> {
    validate_project_name(name)?;

    let paths = ProjectPaths::resolve(name)?;

    if !project_exists(&paths) {
        bail!("project '{}' does not exist", name);
    }

    let target = if compose {
        &paths.compose_file
    } else {
        &paths.env_file
    };

    if !target.exists() {
        bail!("configuration file not found: {}", target.display());
    }

    open_in_editor(target)?;

    if !no_reload {
        reload_if_running(name, &paths)?;
    }

    Ok(())
}

fn open_in_editor(path: &Path) -> Result<()> {
    let file = path.to_str().context("path is not valid UTF-8")?;

    if let Ok(visual) = std::env::var("VISUAL") {
        if !visual.is_empty() {
            return exec_editor(&visual, file);
        }
    }

    if let Ok(editor) = std::env::var("EDITOR") {
        if !editor.is_empty() {
            return exec_editor(&editor, file);
        }
    }

    if let Ok(status) = Command::new("xdg-open").arg(file).status() {
        if status.success() {
            return Ok(());
        }
    }

    if let Ok(status) = Command::new("open").arg(file).status() {
        if status.success() {
            return Ok(());
        }
    }

    bail!("no editor found; set the VISUAL or EDITOR environment variable, or install xdg-open");
}

fn exec_editor(editor: &str, file: &str) -> Result<()> {
    let status = Command::new(editor)
        .arg(file)
        .status()
        .with_context(|| format!("failed to launch editor '{}'", editor))?;

    if !status.success() {
        bail!(
            "editor '{}' exited with status {}",
            editor,
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}

fn reload_if_running(name: &str, paths: &ProjectPaths) -> Result<()> {
    let container_running = docker::container_id(name).is_ok();

    if !container_running {
        return Ok(());
    }

    println!("Reloading container for project '{}'...", name);

    let mut env_vars = std::collections::HashMap::new();
    env_vars.insert(
        "PROJECTS_TOP_DIR".to_string(),
        paths.projects_top_dir.to_string_lossy().into_owned(),
    );
    env_vars.insert("PROJECT_NAME".to_string(), name.to_string());

    let base_compose = paths.templates_dir.join("docker-compose-ssh.yml");
    let compose_files: Vec<&std::path::Path> = if base_compose.exists() {
        vec![base_compose.as_path(), paths.compose_file.as_path()]
    } else {
        vec![paths.compose_file.as_path()]
    };

    docker::compose_up(name, &compose_files, &env_vars)
        .context("failed to reload container after edit")?;

    println!("Container reloaded.");
    Ok(())
}
