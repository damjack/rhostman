use std::fs::File;
use std::io::{self, BufRead};

pub fn get_hosts_lines(path: String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let host_file = get_hosts_file(path);

    Ok(io::BufReader::new(host_file).lines())
}

pub fn get_hosts_file(path: String) -> std::fs::File {
    match File::open(path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    }
}
