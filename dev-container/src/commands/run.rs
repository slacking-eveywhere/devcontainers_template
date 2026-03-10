use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::config::{EnvConfig, ProjectPaths, project_exists, validate_project_name};
use crate::docker;

const DEFAULT_SSH_PORT: u16 = 22224;

pub fn execute(name: &str, ssh_key: Option<&str>) -> Result<()> {
    validate_project_name(name)?;

    let paths = ProjectPaths::resolve(name)?;

    if !project_exists(&paths) {
        bail!("project '{}' does not exist", name);
    }

    if !paths.env_file.exists() {
        bail!(
            "missing configuration file: {}; run 'dev-container new {}' first",
            paths.env_file.display(),
            name
        );
    }

    if !paths.compose_file.exists() {
        bail!(
            "missing configuration file: {}; run 'dev-container new {}' first",
            paths.compose_file.display(),
            name
        );
    }

    let env = EnvConfig::load(&paths.env_file).context("failed to parse .env file")?;

    let ssh_public_key_path = match ssh_key {
        Some(p) => {
            let key_path = Path::new(p);
            if !key_path.exists() {
                bail!("SSH public key file not found: {}", p);
            }
            validate_ssh_public_key(key_path)?;
            Some(key_path)
        }
        None => None,
    };

    create_volumes(&paths, &env)?;

    let mut compose_env: HashMap<String, String> = HashMap::new();
    compose_env.insert(
        "PROJECTS_TOP_DIR".to_string(),
        paths.projects_top_dir.to_string_lossy().into_owned(),
    );
    compose_env.insert("PROJECT_NAME".to_string(), name.to_string());

    let ssh_port = env.ssh_port.unwrap_or(DEFAULT_SSH_PORT);
    compose_env.insert("SSH_PORT".to_string(), ssh_port.to_string());

    for (k, v) in &env.raw {
        compose_env.entry(k.clone()).or_insert_with(|| v.clone());
    }

    let base_compose = paths.templates_dir.join("docker-compose-ssh.yml");
    let compose_files: Vec<&Path> = if base_compose.exists() {
        vec![base_compose.as_path(), paths.compose_file.as_path()]
    } else {
        eprintln!(
            "warning: base template not found at {}; using project compose only",
            base_compose.display()
        );
        vec![paths.compose_file.as_path()]
    };

    println!("Starting project '{}'...", name);

    docker::compose_up(name, &compose_files, &compose_env).with_context(|| {
        format!(
            "failed to start project '{}'; check your docker-compose.yml and .env configuration",
            name
        )
    })?;

    let container = docker::container_id(name).with_context(|| {
        format!(
            "container started but could not be located for project '{}'",
            name
        )
    })?;

    println!("Container running: {}", container);

    if let Some(key_path) = ssh_public_key_path {
        inject_ssh_key(&container, key_path, &env)?;
    }

    print_summary(name, &env, ssh_port);

    Ok(())
}

fn create_volumes(paths: &ProjectPaths, env: &EnvConfig) -> Result<()> {
    fs::create_dir_all(&paths.volumes_dir).with_context(|| {
        format!(
            "failed to create volumes directory: {}",
            paths.volumes_dir.display()
        )
    })?;

    let workdir_vol = paths.volumes_dir.join("workdir");
    fs::create_dir_all(&workdir_vol)
        .with_context(|| format!("failed to create workdir volume: {}", workdir_vol.display()))?;

    if let Some(user) = &env.user {
        if !user.is_empty() {
            let user_vol = paths.volumes_dir.join(user);
            fs::create_dir_all(&user_vol)
                .with_context(|| format!("failed to create user volume: {}", user_vol.display()))?;
        }
    }

    Ok(())
}

fn validate_ssh_public_key(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read SSH public key: {}", path.display()))?;

    let first_line = content.lines().find(|l| !l.trim().is_empty()).unwrap_or("");

    let key_type = first_line.split_whitespace().next().unwrap_or("");

    let valid_types = [
        "ssh-rsa",
        "ssh-ed25519",
        "ecdsa-sha2-nistp256",
        "ecdsa-sha2-nistp384",
        "ecdsa-sha2-nistp521",
        "sk-ssh-ed25519@openssh.com",
        "sk-ecdsa-sha2-nistp256@openssh.com",
    ];

    if !valid_types.contains(&key_type) {
        bail!(
            "'{}' does not appear to be a valid SSH public key (unrecognised key type '{}'); \
             expected one of: {}",
            path.display(),
            key_type,
            valid_types.join(", ")
        );
    }

    let tokens: Vec<&str> = first_line.split_whitespace().collect();
    if tokens.len() < 2 {
        bail!(
            "'{}' does not appear to be a valid SSH public key (missing key data)",
            path.display()
        );
    }

    Ok(())
}

fn inject_ssh_key(container_id: &str, key_path: &Path, env: &EnvConfig) -> Result<()> {
    println!("Injecting SSH public key into container...");

    let user = env.user.as_deref().unwrap_or("root");
    let ssh_dir = format!("/home/{}/.ssh", user);
    let authorized_keys = format!("{}/authorized_keys", ssh_dir);
    let tmp_key = format!("{}/injected_key.pub", ssh_dir);

    let mkdir_output =
        docker::docker_exec(container_id, ["sh", "-c", &format!("mkdir -p {}", ssh_dir)])
            .context("failed to create .ssh directory in container")?;

    if !mkdir_output.status.success() {
        let stderr = String::from_utf8_lossy(&mkdir_output.stderr);
        bail!(
            "failed to create .ssh directory in container: {}",
            stderr.trim()
        );
    }

    let dest = format!("{}:{}", container_id, tmp_key);
    docker::docker_cp(key_path, &dest).context("failed to copy SSH public key into container")?;

    let append_cmd = format!(
        "cat '{tmp}' >> '{auth}' && chmod 600 '{auth}' && rm -f '{tmp}'",
        tmp = tmp_key,
        auth = authorized_keys
    );

    let append_output = docker::docker_exec(container_id, ["sh", "-c", &append_cmd])
        .context("failed to configure authorized_keys in container")?;

    if !append_output.status.success() {
        let stderr = String::from_utf8_lossy(&append_output.stderr);
        bail!(
            "failed to append SSH key to authorized_keys: {}",
            stderr.trim()
        );
    }

    println!("SSH public key injected into {}", authorized_keys);

    Ok(())
}

fn print_summary(name: &str, env: &EnvConfig, ssh_port: u16) {
    let separator = "─".repeat(60);
    println!("{}", separator);
    println!("  Project '{}' is running", name);
    println!();

    if let Some(image) = &env.devcontainer_image_name {
        println!("  Image     : {}", image);
    }

    let host = env.hostname.as_deref().unwrap_or("localhost");
    println!("  SSH       : {}:{}", host, ssh_port);

    if let Some(user) = &env.user {
        println!("  User      : {}", user);
        println!();
        println!("  Connect   : dev-container connect {}", name);
    }

    println!("{}", separator);
}
