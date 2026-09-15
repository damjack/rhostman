use std::path::PathBuf;

use crate::errors::{RhostmanError, RhostmanResult};
use crate::hosts_file::io;
use crate::remote;
use crate::sources::SourcesConfig;

fn blocklist_lines(url: &str) -> RhostmanResult<Vec<String>> {
    let domains = remote::fetch_blocklist_domains(url)?;
    Ok(domains.iter().map(|d| format!("0.0.0.0 {}", d)).collect())
}

pub fn handle_add(path: PathBuf, config: PathBuf, name: String, url: String) -> RhostmanResult<()> {
    let mut cfg = SourcesConfig::load(&config)?;
    if cfg.get(&name).is_some() {
        return Err(RhostmanError::SourceExists(name));
    }

    let block_lines = blocklist_lines(&url)?;

    io::auto_backup(&path)?;
    let mut doc = io::read(&path)?;
    doc.replace_source_block(&name, &block_lines)?;
    io::write_atomic(&path, &doc)?;

    cfg.add(&name, &url)?;
    cfg.save(&config)?;

    println!("Tracking '{}' ({} entries) from {}", name, block_lines.len(), url);
    Ok(())
}

pub fn handle_update(path: PathBuf, config: PathBuf, name: Option<String>) -> RhostmanResult<()> {
    let cfg = SourcesConfig::load(&config)?;

    let targets: Vec<_> = match &name {
        Some(name) => {
            let source = cfg
                .get(name)
                .ok_or_else(|| RhostmanError::SourceNotFound(name.clone()))?;
            vec![source.clone()]
        }
        None => cfg.sources.clone(),
    };

    // Fetch every target before touching the hosts file, so a failed fetch
    // never leaves a partially-updated file.
    let mut updates = Vec::with_capacity(targets.len());
    for source in &targets {
        updates.push((source.name.clone(), blocklist_lines(&source.url)?));
    }

    io::auto_backup(&path)?;
    let mut doc = io::read(&path)?;
    for (name, block_lines) in &updates {
        doc.replace_source_block(name, block_lines)?;
    }
    io::write_atomic(&path, &doc)?;

    println!("Updated {} tracked source(s)", updates.len());
    Ok(())
}

pub fn handle_remove(path: PathBuf, config: PathBuf, name: String) -> RhostmanResult<()> {
    let mut cfg = SourcesConfig::load(&config)?;
    if cfg.get(&name).is_none() {
        return Err(RhostmanError::SourceNotFound(name));
    }

    io::auto_backup(&path)?;
    let mut doc = io::read(&path)?;
    doc.remove_source_block(&name)?;
    io::write_atomic(&path, &doc)?;

    cfg.remove(&name)?;
    cfg.save(&config)?;

    println!("Stopped tracking '{}'", name);
    Ok(())
}

pub fn handle_list(config: PathBuf) -> RhostmanResult<()> {
    let cfg = SourcesConfig::load(&config)?;
    if cfg.sources.is_empty() {
        println!("No tracked sources.");
        return Ok(());
    }
    for source in &cfg.sources {
        println!("{}\t{}", source.name, source.url);
    }
    Ok(())
}
