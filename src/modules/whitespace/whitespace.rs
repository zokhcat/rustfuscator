use regex::Regex;

pub fn whitespace_and_comments(js_code: &str) -> String {
    let re_line_comment = Regex::new(r"//.*").unwrap();
    let re_block_comment = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    
    let without_line_comments = re_line_comment.replace_all(js_code, "");
    let without_comments = re_block_comment.replace_all(&without_line_comments, "");

    let re_whitespace = Regex::new(r"\s+").unwrap();
    
    let minified_code = re_whitespace.replace_all(&without_comments, " ").trim().to_string();

    minified_code
}