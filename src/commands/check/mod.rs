use crate::utils;

pub fn handle_command(path: String, host: Option<String>) {
    println!("handle Check: host({:?}))", host);

    let _lines = utils::get_hosts_lines(path);
}
