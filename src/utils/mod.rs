use std::fs;

pub fn get_base_file(path: Option<String>) {
    let contents = path.map(|p| fs::read_to_string(p).expect("Something went wrong reading hosts file"));

    println!("handle get base file: path({:?}))", contents);
}
