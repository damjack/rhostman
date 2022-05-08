pub fn find_line(host: &String, contents: &str) -> bool {
    for line in contents.lines() {
        if line.contains(host) {
            return true;
        }
    }
    false
}
