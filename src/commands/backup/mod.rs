use std::fs;
use std::path::PathBuf;

use crate::errors::RhostmanResult;

pub fn handle_command(path: PathBuf, output: PathBuf) -> RhostmanResult<()> {
    fs::copy(&path, &output)?;
    println!("Backed up {} to {}", path.display(), output.display());
    Ok(())
}
