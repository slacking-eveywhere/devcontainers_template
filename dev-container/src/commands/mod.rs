use anyhow::Result;

use crate::cli::Command;

mod connect;
mod edit;
mod inspect;
mod list;
mod new;
mod run;
mod stop;

pub fn dispatch(command: Command) -> Result<()> {
    match command {
        Command::New { name, force } => new::execute(&name, force),
        Command::Edit {
            name,
            compose,
            no_reload,
        } => edit::execute(&name, compose, no_reload),
        Command::Inspect { name } => inspect::execute(&name),
        Command::Run { name, ssh_key } => run::execute(&name, ssh_key.as_deref()),
        Command::Stop {
            name,
            detach,
            force,
        } => stop::execute(&name, detach, force),
        Command::Connect { name, ssh_key } => connect::execute(&name, ssh_key.as_deref()),
        Command::List => list::execute(),
        Command::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}
