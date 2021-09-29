use std::path::PathBuf;

pub fn handle_command(ipt: PathBuf, q: bool) {
    println!("handle Disable: search({}), input({:?})", q, ipt);
}
