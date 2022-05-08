use crate::errors;
use crate::utils::hosts;
use crate::utils::search;
use std::path::PathBuf;

pub fn handle_command(path: PathBuf, hosts: Vec<String>) -> errors::RhostmanResult<()> {
    let content = hosts::hosts_to_string(path)?;

    for pattern in hosts {
        if search::find_line(&pattern, &content) {
            println!("{}", &pattern);
        }
    }

    Ok(())
}
