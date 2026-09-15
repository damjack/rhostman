use std::path::PathBuf;

use crate::errors::RhostmanResult;
use crate::hosts_file::{io, HostsLine};
use crate::remote;

/// Fetches `url` once and merges its entries into the hosts file. This is a
/// one-shot, unmarked merge: unlike `track add`, it does not remember where
/// the entries came from, so there is nothing to later `track update` or
/// `track remove`.
pub fn handle_command(path: PathBuf, url: String) -> RhostmanResult<()> {
    let fetched = remote::fetch_entries(&url)?;

    io::auto_backup(&path)?;
    let mut doc = io::read(&path)?;
    for entry in &fetched {
        if let HostsLine::Entry { ip, hostnames, .. } = entry {
            doc.add_entry(ip, hostnames, None)?;
        }
    }
    io::write_atomic(&path, &doc)?;

    println!("Imported {} entries from {}", fetched.len(), url);
    Ok(())
}
