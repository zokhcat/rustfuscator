use std::collections;

use rand::{thread_rng, Rng};
use regex::{Captures, Regex};

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = thread_rng();
    (0..length)
    .map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    }).collect()
}

pub fn obfuscate_vars(js_code: &str) -> String {
    let re = Regex::new(r"\b(let|var|const|function)\s+(a-zA-Z_)\w*").unwrap();
    let mut replacements = collections::HashMap::new();

    let obfuscated_code = re.replace_all(js_code, |caps: &Captures| {
        let var_name = &caps[2];

        let obfuscated_name = replacements.entry(var_name.to_string()).or_insert_with(|| generate_random_string(8));

        format!("{} {}", &caps[1], obfuscated_name)
    });

    obfuscated_code.to_string()
}