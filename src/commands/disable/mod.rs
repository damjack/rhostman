use std::path::PathBuf;

pub fn handle_command(path: String, ipt: PathBuf, q: bool) {
    println!("handle Disable: path: {:?}, search({}), input({:?})", path, q, ipt);
}
