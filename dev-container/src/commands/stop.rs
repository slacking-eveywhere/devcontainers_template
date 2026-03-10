use std::collections::HashMap;

use anyhow::{Result, bail};

use crate::config::{ProjectPaths, project_exists, validate_project_name};
use crate::docker;

pub fn execute(name: &str, detach: bool, force: bool) -> Result<()> {
    validate_project_name(name)?;

    let paths = ProjectPaths::resolve(name)?;

    if !project_exists(&paths) {
        bail!("project '{}' does not exist", name);
    }

    println!("Stopping project '{}'...", name);

    let wait = !detach;

    let env_vars: HashMap<String, String> = HashMap::new();

    docker::compose_down(name, force, wait, &env_vars)?;

    println!("Project '{}' stopped.", name);

    Ok(())
}
