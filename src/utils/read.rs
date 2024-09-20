use std::{fs::File, io::{self, Read}};

pub fn read_js_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(contents)
}