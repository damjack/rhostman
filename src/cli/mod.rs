use clap::{Parser, Subcommand};
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "rhostname")]
#[clap(about = "A CLI to manage hosts file", long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    pub path: Option<String>,
    #[clap(subcommand)]
    pub commands: Command,
}

#[derive(Debug, Subcommand)]
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
        #[clap(short, long)]
        host: Option<String>,
    },
    #[clap(arg_required_else_help = true)]
    Check {
        #[clap(short, long)]
        host: Option<String>,
    },
    #[clap(arg_required_else_help = true)]
    Remove {
        #[clap(short, long, required_unless("address"))]
        host: bool,
        #[clap(short, long, required_unless("host"))]
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
