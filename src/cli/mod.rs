use clap::{Parser, Subcommand};

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;

pub const ETC_HOSTS: &str = "/etc/hosts";

#[derive(Parser, Debug, PartialEq)]
#[clap(name = "rhostname")]
#[clap(version, about = "A CLI to manage hosts file", long_about = None)]
pub struct Cli {
    #[clap(short, long, default_value = ETC_HOSTS)]
    pub path: String,
    #[clap(subcommand)]
    pub commands: Command,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Command {
    #[clap(arg_required_else_help = true)]
    Import {
        #[clap(required = true, parse(from_os_str))]
        url: PathBuf,
    },
    #[clap(arg_required_else_help = true)]
    Add {
        #[clap(required = true, parse(from_os_str))]
        input: Vec<PathBuf>,
        #[clap(short, long)]
        inline: bool,
        #[clap(short = 'H', long)]
        host: Option<String>,
    },
    #[clap(arg_required_else_help = true)]
    Check {
        #[clap(short = 'H', long)]
        host: Option<String>,
    },
    #[clap(arg_required_else_help = true)]
    Remove {
        #[clap(short = 'H', long)]
        host: bool,
        #[clap(short, long)]
        address: bool,
        #[clap(required = true, parse(from_os_str))]
        input: PathBuf,
    },
    #[clap(arg_required_else_help = true)]
    Disable {
        #[clap(short, long)]
        search: bool,
        #[clap(required = true, parse(from_os_str))]
        input: PathBuf,
    },
    #[clap(arg_required_else_help = true)]
    Backup {
        #[clap(required = true, parse(from_os_str))]
        input: PathBuf,
    },
}

#[derive(Debug)]
pub enum CliError {
    WrongParameters,
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
