use crate::errors;
use std::path::PathBuf;

pub fn handle_command(path: PathBuf, backup_file: PathBuf) -> errors::RhostmanResult<()> {
    println!("handle Backup: path: {:?}, input{:?}", path, backup_file);

    Ok(())
}
