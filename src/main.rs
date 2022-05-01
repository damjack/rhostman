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
        Command::Add { hosts } => commands::add::handle_command(opt.path, hosts),
        Command::Remove { host } => commands::remove::handle_command(opt.path, host),
        Command::Import { raw_url } => commands::import::handle_command(opt.path, raw_url),
        Command::Disable { host } => commands::disable::handle_command(opt.path, host),
        Command::Backup { backup_file } => commands::backup::handle_command(opt.path, backup_file),
    }
}
