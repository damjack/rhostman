use crate::errors;
use crate::utils;
use std::path::PathBuf;

pub fn handle_command(path: PathBuf, hosts: Vec<String>) -> errors::RhostmanResult<()> {
    let content = utils::hosts_to_string(path)?;

    for pattern in hosts {
        for line in content.lines() {
            if line.contains(&pattern) {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
