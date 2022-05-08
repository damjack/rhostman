use crate::errors;
use crate::commands::common_type;
use crate::utils::hosts;
use crate::utils::search;
use std::path::PathBuf;

pub fn handle_command(path: PathBuf, hosts: Vec<String>) -> errors::RhostmanResult<()> {
    let content = hosts::hosts_to_string(path)?;
    let only_new_hosts: Vec<common_type::Host> = hosts
        .into_iter()
        .map(Into::into)
        .filter(|host| !search::find_line(host, &content))
        .collect();

    for pattern in only_new_hosts {
        println!("{}", &pattern);
    }

    Ok(())
}
