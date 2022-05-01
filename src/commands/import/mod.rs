use crate::errors;
use std::path::PathBuf;

pub fn handle_command(path: PathBuf, raw_url: String) -> errors::RhostmanResult<()> {
    println!("handle Import: path: {:?}, url({:?})", path, raw_url);

    Ok(())
}
