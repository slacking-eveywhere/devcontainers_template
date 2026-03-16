use anyhow::{Result, bail};

use crate::config::{ProjectPaths, project_exists, validate_project_name};

const ENV_TEMPLATE: &str = "\
# --- User Configuration ---
# Set the username and UID for the devcontainer user.
USER=
USER_ID=

# --- Devcontainer Image ---
# Specify the Docker image to use for the development environment.
DEVCONTAINER_IMAGE_NAME=

# --- Resource Limits ---
# Optional: Define resource limits, e.g., RAM.
RESOURCES_RAM=

# --- SSH Configuration ---
# Optional: Define SSH port for accessing the devcontainer.
HOSTNAME=
SSH_PORT=
ENABLE_SSH=true
";

const COMPOSE_TEMPLATE: &str = "\
# docker compose file that can be modified to add some specs to the dev stack
services:
  maindevcontainer:
    # This service inherits its base configuration from the template.
    # You can add overrides here, such as volume mounts or port mappings.
    env_file:
      - .env
";

const COMPOSE_BASE_TEMPLATE: &str = r#"# docker compose file that is immutable and can be overwritten by COMPOSE_TEMPLATE
services:
  maindevcontainer:
    image: "${DEVCONTAINER_IMAGE_NAME:-bash:5}"
    volumes:
      - ${PROJECTS_TOP_DIR}/${PROJECT_NAME}/volumes/workdir:/workdir
      - ${PROJECTS_TOP_DIR}/${PROJECT_NAME}/volumes/${USER}:/home/${USER}
    ports:
      - "${SSH_PORT:-22224}:22"
    tmpfs: /tmp:exec,mode=1777
    tty: true
    stdin_open: true
    command: ["sleep", "infinity"]
    deploy:
      resources:
        reservations:
          memory: "${RESOURCES_RAM:-2G}"
        limits:
          memory: "${RESOURCES_RAM:-2G}"

"#;

pub fn execute(name: &str, force: bool) -> Result<()> {
    validate_project_name(name)?;

    let paths = ProjectPaths::resolve(name)?;

    ensure_base_template(&paths)?;

    if project_exists(&paths) && !force {
        bail!(
            "project '{}' already exists; use --force to overwrite",
            name
        );
    }

    std::fs::create_dir_all(&paths.project_dir)?;
    std::fs::create_dir_all(&paths.volumes_dir)?;

    write_template(&paths.env_file, ENV_TEMPLATE, force, ".env")?;
    write_template(
        &paths.compose_file,
        COMPOSE_TEMPLATE,
        force,
        "docker-compose.yml",
    )?;

    println!("Created project '{}'", name);
    println!("  {}", paths.env_file.display());
    println!("  {}", paths.compose_file.display());
    println!();
    println!(
        "Next: edit the configuration then run 'dev-container run {}'",
        name
    );

    Ok(())
}

fn ensure_base_template(paths: &ProjectPaths) -> Result<()> {
    std::fs::create_dir_all(&paths.templates_dir)?;

    let base_compose = paths.templates_dir.join("docker-compose-ssh.yml");
    if !base_compose.exists() {
        std::fs::write(&base_compose, COMPOSE_BASE_TEMPLATE)?;
    }

    Ok(())
}

fn write_template(path: &std::path::Path, content: &str, force: bool, label: &str) -> Result<()> {
    if path.exists() && !force {
        println!(
            "  skipping {} (already exists; use --force to overwrite)",
            label
        );
        return Ok(());
    }

    std::fs::write(path, content)?;
    Ok(())
}
