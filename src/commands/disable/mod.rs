use std::path::PathBuf;

use crate::cli::Selector;
use crate::errors::{RhostmanError, RhostmanResult};
use crate::hosts_file::io;

pub fn handle_command(path: PathBuf, host: Option<String>, domain: Option<String>) -> RhostmanResult<()> {
    let selector = Selector::from_args(host, domain).map_err(RhostmanError::GenericError)?;

    io::auto_backup(&path)?;
    let mut doc = io::read(&path)?;
    let disabled = match &selector {
        Selector::Host(host) => doc.disable_by_host(host)?,
        Selector::Domain(domain) => doc.disable_by_domain(domain)?,
    };
    io::write_atomic(&path, &doc)?;

    println!("Disabled {} entr{}", disabled, if disabled == 1 { "y" } else { "ies" });
    Ok(())
}
