use clap::Parser;

use rhostman::cli::{Cli, Command, TrackAction};
use rhostman::commands;
use rhostman::errors;
use rhostman::sources;

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
        Command::Track { action } => handle_track_action(action),
    }
}

fn handle_track_action(action: TrackAction) -> errors::RhostmanResult<()> {
    match action {
        TrackAction::Add {
            path,
            config,
            name,
            url,
        } => commands::track::handle_add(path, resolve_config(config), name, url),
        TrackAction::Update { path, config, name } => {
            commands::track::handle_update(path, resolve_config(config), name)
        }
        TrackAction::Remove { path, config, name } => {
            commands::track::handle_remove(path, resolve_config(config), name)
        }
        TrackAction::List { config } => commands::track::handle_list(resolve_config(config)),
    }
}

fn resolve_config(config: Option<std::path::PathBuf>) -> std::path::PathBuf {
    config.unwrap_or_else(sources::default_sources_config_path)
}
