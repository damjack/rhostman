use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::PathBuf;

use crate::errors;

pub fn get_hosts_lines(path: PathBuf) -> errors::RhostmanResult<io::Lines<io::BufReader<File>>> {
    match get_hosts_file(path) {
        Ok(host_file) => Ok(io::BufReader::new(host_file).lines()),
        Err(err) => return Err(errors::RhostmanError::GenericError(format!("{}", err))),
    }
}

pub fn get_hosts_file(path: PathBuf) -> errors::RhostmanResult<std::fs::File> {
    match File::open(path.clone()) {
        Ok(file) => Ok(file),
        Err(err) => {
            return Err(errors::RhostmanError::GenericError(format!(
                "couldn't open `{:?}`: {}",
                path, err
            )))
        }
    }
}

pub fn hosts_to_string(path: PathBuf) -> errors::RhostmanResult<String> {
    match read_to_string(path.clone()) {
        Ok(content) => Ok(content),
        Err(error) => {
            return Err(errors::RhostmanError::GenericError(format!(
                "Error reading `{:?}`: {}",
                path, error
            )))
        }
    }
}
