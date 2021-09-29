use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "the host mnager")]
pub struct Cli {
    #[structopt(short, long)]
    pub path: Option<String>,
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    pub commands: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "import")]
    Import {
        #[structopt(parse(from_os_str))]
        url: PathBuf,
    },
    #[structopt(name = "add")]
    Add {
        #[structopt(parse(from_os_str))]
        input: Vec<PathBuf>,
        #[structopt(short, long)]
        inline: bool,
        #[structopt(short, long)]
        host: Option<String>,
    },
    #[structopt(name = "remove")]
    Remove {
        #[structopt(short, long, required_unless("address"))]
        host: bool,
        #[structopt(short, long, required_unless("host"))]
        address: bool,
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
    #[structopt(name = "disable")]
    Disable {
        #[structopt(short, long)]
        search: bool,
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
    #[structopt(name = "backup")]
    Backup {
        #[structopt(parse(from_os_str))]
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
