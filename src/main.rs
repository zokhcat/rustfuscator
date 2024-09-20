mod modules;
mod utils;

use utils::{
    read::read_js_file,
    write::write_js_file
};

use modules::{
    dead_code::dead_code::insert_dead_code, 
    whitespace::whitespace::whitespace_and_comments, 
    variables::variable_rename::obfuscate_vars,
    encode::encode::encode_str,
    number_obs::number_obs::number_obs
};


fn main() {
    let file_path = "./test-js/test1.js";
    let out_file_path = "./test-js/test1.obs.js";

    match read_js_file(&file_path){
        Ok(mut js_code) => {
            js_code = insert_dead_code(&js_code);

            js_code = whitespace_and_comments(&js_code);

            js_code = obfuscate_vars(&js_code);

            js_code = encode_str(&js_code);

            js_code = number_obs(&js_code);

            match write_js_file(out_file_path, &js_code) {
                Ok(_) => println!("Obfuscated JavaScript code saved to: {}", out_file_path),
                Err(e) => eprintln!("Error writing file: {}", e),
            }

        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}