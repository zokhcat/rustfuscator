use std::{fs::File, io::{self, Read}};

fn main() {
    let file_path = "./test-js/test1.js";

    match read_js_file(file_path) {
        Ok(js_code) => println!("Javascript code:\n{}", js_code),
        Err(e) => eprintln!("Error reading file: {}", e),
    } 
}


fn read_js_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(contents)
}