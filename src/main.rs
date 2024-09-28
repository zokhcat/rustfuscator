mod modules;
mod utils;

use clap::{Arg, Command};
use utils::{
    read::read_js_file,
    write::write_js_file
};

use modules::{
    dead_code::dead_code::insert_dead_code, 
    encode::encode::encode_str, 
    number_obs::number_obs::number_obs, 
    split_function::split_function::split_function, 
    variables::variable_rename::obfuscate_vars,
    whitespace::whitespace::whitespace_and_comments,
    array_flat::array_flat::flatten_array,
    control_flat::control_flat::control_flat
};


fn main() {
    let matches = Command::new("rustfuscator")
        .version("0.1.0")
        .author("zoheb khan")
        .about("Obfuscates JavaScript code")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .required(true),
        )
        .get_matches();
    
    
    let in_file_path = matches.get_one::<String>("input").expect("Input file path is required");
    let out_file_path = matches.get_one::<String>("output").expect("Output file path is required");

    match read_js_file(in_file_path){
        Ok(mut js_code) => {            
            js_code = whitespace_and_comments(&js_code);
            js_code = insert_dead_code(&js_code);
            js_code = obfuscate_vars(&js_code);
            js_code = encode_str(&js_code);
            js_code = number_obs(&js_code);
            js_code = split_function(&js_code);
            js_code = flatten_array(&js_code);
            js_code = control_flat(&js_code);

            match write_js_file(out_file_path, &js_code) {
                Ok(_) => println!("Obfuscated JavaScript code saved to: {}", out_file_path),
                Err(e) => eprintln!("Error writing file: {}", e),
            }

        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}