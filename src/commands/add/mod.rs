use std::path::PathBuf;

use crate::errors::RhostmanResult;
use crate::hosts_file::io;

pub fn handle_command(path: PathBuf, ip: String, hosts: Vec<String>, comment: Option<String>) -> RhostmanResult<()> {
    io::auto_backup(&path)?;
    let mut doc = io::read(&path)?;
    doc.add_entry(&ip, &hosts, comment.as_deref())?;
    io::write_atomic(&path, &doc)?;

    println!("Added {} -> {}", ip, hosts.join(", "));
    Ok(())
}
