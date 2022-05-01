use std::path::PathBuf;

pub fn handle_command(path: String, ipt: Vec<PathBuf>, iln: bool, host: Option<String>) {
    println!("handle Add: path: {:?}, input({:?}), inline({}, host({:?}))", path, ipt, iln, host);
}
