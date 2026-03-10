use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dev-container",
    version,
    about = "Devcontainer lifecycle manager",
    long_about = None,
    disable_help_subcommand = false,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a new project with the given name
    New {
        /// Project name
        name: String,

        /// Overwrite the project if it already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Edit a project's configuration files
    Edit {
        /// Project name
        name: String,

        /// Open docker-compose.yml instead of .env
        #[arg(short, long = "compose")]
        compose: bool,

        /// Do not reload the container after editing
        #[arg(short = 'q', long = "no-reload")]
        no_reload: bool,
    },

    /// Inspect project configuration and status
    Inspect {
        /// Project name
        name: String,
    },

    /// Run a devcontainer project in the background
    Run {
        /// Project name
        name: String,

        /// Path to an SSH public key to inject into the container's authorized_keys
        #[arg(short = 's', long = "ssh-key", value_name = "KEY_PATH")]
        ssh_key: Option<String>,
    },

    /// Stop a running devcontainer project
    Stop {
        /// Project name
        name: String,

        /// Return immediately without waiting for the container to stop
        #[arg(short, long)]
        detach: bool,

        /// Force-kill the container
        #[arg(short, long)]
        force: bool,
    },

    /// Connect to a running devcontainer via SSH
    Connect {
        /// Project name
        name: String,

        /// Path to the SSH private key to use for authentication
        #[arg(short = 's', long = "ssh-key", value_name = "KEY_PATH")]
        ssh_key: Option<String>,
    },

    /// List all devcontainer projects
    List,

    /// Print the version
    Version,
}
