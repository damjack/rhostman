use std::path::PathBuf;

pub fn handle_command(path: String, ipt: PathBuf, host: bool, add: bool) {
    println!("handle Remove: path: {:?}, host({}), address({}), input({:?})", path, host, add, ipt);
}
