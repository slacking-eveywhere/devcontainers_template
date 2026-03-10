use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod config;
mod docker;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {:#}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    commands::dispatch(cli.command)
}
