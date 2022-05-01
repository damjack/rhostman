use crate::cli::{Cli, CliError, Command};

pub mod cli;
pub mod commands;
pub mod utils;

fn main() {
    let args: Cli = Cli::parse();

    println!("Args {:?}", args);
    handle_subcommand(args);
}

fn handle_subcommand(opt: Cli) {
    let file_data = utils::get_base_file(opt.path);
    println!("file data {:?}", file_data);
    // handle subcommands
    match opt.commands {
        Command::Add { input, inline, host } => {
            commands::add::handle_command(input, inline, host);
        }
        Command::Check { host } => {
            commands::check::handle_command(host);
        },
        Command::Remove { host, address, input } => {
            commands::remove::handle_command(input, host, address);
        }
        Command::Import { url } => {
            commands::import::handle_command(url);
        }
        Command::Disable { search, input } => {
            commands::disable::handle_command(input, search);
        }
        Command::Backup { input } => commands::backup::handle_command(input),
    }
}
