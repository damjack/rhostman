use std::path::PathBuf;

pub fn handle_command(ipt: Vec<PathBuf>, iln: bool, host: Option<String>) {
    println!("handle Add:  input({:?}), inline({}, host({:?}))", ipt, iln, host);
}
