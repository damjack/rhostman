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
    #[clap(subcommand)]
    pub commands: Command,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Command {
    #[clap(arg_required_else_help = true)]
    Import {
        #[clap(short, long, default_value = ETC_HOSTS, parse(from_os_str))]
        path: PathBuf,
        #[clap(required = true)]
        url: String,
    },
    #[clap(arg_required_else_help = true)]
    Add {
        #[clap(short, long, default_value = ETC_HOSTS, parse(from_os_str))]
        path: PathBuf,
        #[clap(required = true)]
        hosts: Vec<String>,
    },
    #[clap(arg_required_else_help = true)]
    Remove {
        #[clap(short, long, default_value = ETC_HOSTS, parse(from_os_str))]
        path: PathBuf,
        #[clap(required = true)]
        host: String,
    },
    #[clap(arg_required_else_help = true)]
    Disable {
        #[clap(short, long, default_value = ETC_HOSTS, parse(from_os_str))]
        path: PathBuf,
        #[clap(required = true)]
        host: String,
    },
    #[clap(arg_required_else_help = true)]
    Backup {
        #[clap(short, long, default_value = ETC_HOSTS, parse(from_os_str))]
        path: PathBuf,
        #[clap(required = true, parse(from_os_str))]
        output: PathBuf,
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
