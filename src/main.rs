use crate::cli::{Cli, CliError, Command};
use structopt::StructOpt;

pub mod cli;
pub mod add;
pub mod backup;
pub mod disable;
pub mod import;
pub mod remove;
pub mod utils;

fn main() {
    let args: Cli = Cli::from_args();

    println!("Args {:?}", args);

    handle_subcommand(args);
}

fn handle_subcommand(opt: Cli){
    let file_data = utils::get_base_file(opt.path);
    println!("file data {:?}", file_data);
    // handle subcommands
    match  opt.commands{
        Command::Add {input, inline, host} => {
            add::handle_command(input, inline, host);
        },
        Command::Remove {host, address, input} => {
            remove::handle_command(input, host, address);
        },
        Command::Import {url} => {
            import::handle_command(url);
        },
        Command::Disable {search, input} => {
            disable::handle_command(input, search);
        },
        Command::Backup {input} => {
            backup::handle_command(input)
        },
    }
}
