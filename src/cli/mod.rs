use clap::{Parser, Subcommand};

use std::path::PathBuf;

pub const ETC_HOSTS: &str = "/etc/hosts";

#[derive(Parser, Debug, PartialEq)]
#[command(name = "rhostman")]
#[command(version, about = "A CLI to manage hosts file", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Command,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Command {
    #[command(arg_required_else_help = true)]
    Import {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        #[arg(required = true)]
        url: String,
    },
    #[command(arg_required_else_help = true)]
    Add {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        #[arg(required = true, num_args = 1..)]
        hosts: Vec<String>,
    },
    #[command(arg_required_else_help = true)]
    Remove {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        #[arg(required = true)]
        host: String,
    },
    #[command(arg_required_else_help = true)]
    Disable {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        #[arg(required = true)]
        host: String,
    },
    #[command(arg_required_else_help = true)]
    Backup {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        #[arg(required = true)]
        output: PathBuf,
    },
}
