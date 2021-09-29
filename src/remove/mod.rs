use std::path::PathBuf;

pub fn handle_command(ipt: PathBuf, host: bool, add: bool) {
    println!("handle Remove: host({}), address({}), input({:?})", host, add, ipt);
}
