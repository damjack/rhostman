use clap::Parser;

use rhostman::cli::{Cli, Command};
use rhostman::commands;
use rhostman::errors;

fn main() {
    let args: Cli = Cli::parse();

    if let Err(err) = handle_subcommand(args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn handle_subcommand(opt: Cli) -> errors::RhostmanResult<()> {
    match opt.commands {
        Command::Add {
            path,
            ip,
            hosts,
            comment,
        } => commands::add::handle_command(path, ip, hosts, comment),
        Command::Remove { path, host, domain } => commands::remove::handle_command(path, host, domain),
        Command::Import { path, url } => commands::import::handle_command(path, url),
        Command::Disable { path, host, domain } => commands::disable::handle_command(path, host, domain),
        Command::Backup { path, output } => commands::backup::handle_command(path, output),
    }
}
