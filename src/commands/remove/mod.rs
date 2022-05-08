use crate::errors;
use crate::utils::hosts;
use crate::utils::search;
use std::path::PathBuf;

pub fn handle_command(path: PathBuf, host: String) -> errors::RhostmanResult<()> {
    let content = hosts::hosts_to_string(path)?;
    if search::find_line(&host, &content) {
        println!("{}", &host);
    }

    Ok(())
}
