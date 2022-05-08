use clap::Parser;

use rhostman::cli::{Cli, Command};
use rhostman::commands;
use rhostman::errors;

fn main() -> errors::RhostmanResult<()> {
    let args: Cli = Cli::parse();

    handle_subcommand(args)
}

fn handle_subcommand(opt: Cli) -> errors::RhostmanResult<()> {
    match opt.commands {
        Command::Add { path, hosts } => commands::add::handle_command(path, hosts),
        Command::Remove { path, host } => commands::remove::handle_command(path, host),
        Command::Import { path, url } => commands::import::handle_command(path, url),
        Command::Disable { path, host } => commands::disable::handle_command(path, host),
        Command::Backup { path, output } => commands::backup::handle_command(path, output),
    }
}
