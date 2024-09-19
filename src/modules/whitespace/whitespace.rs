use regex::Regex;

fn whitespace_and_comments(js_code: &str) -> String {
    let re_comment = Regex::new(r"//.*?$|/\*[\s\S]*?\*/").unwrap();

    let without_comments = re_comment.replace_all(js_code, "");

    let re_whitespace = Regex::new(r"\s+").unwrap();
    
    let minified_code = re_whitespace.replace_all(&without_comments, " ").trim().to_string();

    minified_code
}