use regex::Regex;

pub fn flatten_array(input: &str) -> String {
    let re = Regex::new(r"\[|\]").unwrap();
    let flattened = re.replace_all(input, "");
    format!("[{}]", flattened.trim())
}