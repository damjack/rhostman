use clap::Parser;

use crate::cli::{Cli, Command};

pub mod cli;
pub mod commands;
pub mod utils;

fn main() {
    let args: Cli = Cli::parse();

    handle_subcommand(args);
}

fn handle_subcommand(opt: Cli) {
    match opt.commands {
        Command::Add { input, inline, host } => {
            commands::add::handle_command(opt.path, input, inline, host);
        }
        Command::Check { host } => {
            commands::check::handle_command(opt.path, host);
        }
        Command::Remove { host, address, input } => {
            commands::remove::handle_command(opt.path, input, host, address);
        }
        Command::Import { url } => {
            commands::import::handle_command(opt.path, url);
        }
        Command::Disable { search, input } => {
            commands::disable::handle_command(opt.path, input, search);
        }
        Command::Backup { input } => commands::backup::handle_command(opt.path, input),
    }
}
