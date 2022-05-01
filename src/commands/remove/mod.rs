use std::path::PathBuf;
use crate::errors;
use crate::utils;

pub fn handle_command(path: PathBuf, host: String) -> errors::RhostmanResult<()> {
    let content = utils::hosts_to_string(path)?;

    for line in content.lines() {
        if line.contains(&host) {
            println!("{}", line);
        }
    }

    Ok(())
}
