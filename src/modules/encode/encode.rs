use base64::{Engine as _, engine::general_purpose::STANDARD};
use regex::{Captures, Regex};

fn encode_str(js_code: &str) -> String {
    let re = Regex::new(r#""([^"]*)""#).unwrap();

    let encode_js = re.replace_all(js_code, |caps: &Captures| {
        let original_str = &caps[1];
        let encoded_str = STANDARD.encode(original_str);
        format!(r#""{}""#, encoded_str)
    });

    encode_js.to_string()
}