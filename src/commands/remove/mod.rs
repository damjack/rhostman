use std::path::PathBuf;

use crate::cli::Selector;
use crate::errors::{RhostmanError, RhostmanResult};
use crate::hosts_file::io;

pub fn handle_command(path: PathBuf, host: Option<String>, domain: Option<String>) -> RhostmanResult<()> {
    let selector = Selector::from_args(host, domain).map_err(RhostmanError::GenericError)?;

    io::auto_backup(&path)?;
    let mut doc = io::read(&path)?;
    let removed = match &selector {
        Selector::Host(host) => doc.remove_by_host(host)?,
        Selector::Domain(domain) => doc.remove_by_domain(domain)?,
    };
    io::write_atomic(&path, &doc)?;

    println!("Removed {} entr{}", removed, if removed == 1 { "y" } else { "ies" });
    Ok(())
}
