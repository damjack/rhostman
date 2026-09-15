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
    /// Fetch a raw hosts-format file from a URL and merge its entries in.
    #[command(arg_required_else_help = true)]
    Import {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        #[arg(required = true)]
        url: String,
    },
    /// Add a new entry to the hosts file.
    #[command(arg_required_else_help = true)]
    Add {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        /// IP address for the new entry.
        #[arg(required = true)]
        ip: String,
        /// One or more hostnames to associate with the IP.
        #[arg(required = true, num_args = 1..)]
        hosts: Vec<String>,
        /// Optional inline comment.
        #[arg(short, long)]
        comment: Option<String>,
    },
    /// Remove entries matching an exact host, or all entries under a domain.
    #[command(arg_required_else_help = true)]
    Remove {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        /// Remove the single entry with this exact IP or hostname.
        #[arg(long)]
        host: Option<String>,
        /// Remove every entry whose hostname matches this domain (or a subdomain of it).
        #[arg(long)]
        domain: Option<String>,
    },
    /// Comment out (disable) entries matching an exact host, or all entries under a domain.
    #[command(arg_required_else_help = true)]
    Disable {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        /// Disable the single entry with this exact IP or hostname.
        #[arg(long)]
        host: Option<String>,
        /// Disable every entry whose hostname matches this domain (or a subdomain of it).
        #[arg(long)]
        domain: Option<String>,
    },
    /// Write a copy of the current hosts file to OUTPUT.
    #[command(arg_required_else_help = true)]
    Backup {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        #[arg(required = true)]
        output: PathBuf,
    },
    /// Manage tracked remote block-list sources.
    Track {
        #[command(subcommand)]
        action: TrackAction,
    },
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum TrackAction {
    /// Register a new tracked source and immediately fetch+insert its block.
    #[command(arg_required_else_help = true)]
    Add {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        /// Override the tracked-sources config file location.
        #[arg(long, env = "RHOSTMAN_CONFIG")]
        config: Option<PathBuf>,
        #[arg(required = true)]
        name: String,
        #[arg(required = true)]
        url: String,
    },
    /// Re-fetch a tracked source and replace just its block in the hosts file.
    /// Omit NAME to update every tracked source.
    Update {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        #[arg(long, env = "RHOSTMAN_CONFIG")]
        config: Option<PathBuf>,
        name: Option<String>,
    },
    /// Stop tracking a source and remove its block from the hosts file.
    #[command(arg_required_else_help = true)]
    Remove {
        #[arg(short, long, default_value = ETC_HOSTS)]
        path: PathBuf,
        #[arg(long, env = "RHOSTMAN_CONFIG")]
        config: Option<PathBuf>,
        #[arg(required = true)]
        name: String,
    },
    /// List all tracked sources.
    List {
        #[arg(long, env = "RHOSTMAN_CONFIG")]
        config: Option<PathBuf>,
    },
}

/// A caller-supplied selector for `remove`/`disable`: exactly one of an
/// exact host match or a domain (subdomain-inclusive) match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    Host(String),
    Domain(String),
}

impl Selector {
    pub fn from_args(host: Option<String>, domain: Option<String>) -> Result<Selector, String> {
        match (host, domain) {
            (Some(host), None) => Ok(Selector::Host(host)),
            (None, Some(domain)) => Ok(Selector::Domain(domain)),
            (Some(_), Some(_)) => Err("specify exactly one of --host or --domain, not both".to_string()),
            (None, None) => Err("specify one of --host or --domain".to_string()),
        }
    }
}
