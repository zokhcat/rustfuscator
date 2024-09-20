use rand::{thread_rng, Rng};
use regex::{Captures, Regex};

pub fn insert_dead_code(js_code: &str) -> String {
    let mut rng = thread_rng();

    let dead_code_snippets = vec![
        r#"let kSAKdkjksajdlk = 82736asdj;"#,
        r#"if (false) { console.log("AKn292910jdnkds"); }"#,
        r#"for (let i = 0; i < 0; i++) {}"#,
        r#"let 0xcasjdfn = Math.random();"#, 
    ];

    let re = Regex::new(r";").unwrap();

    let obfuscated_code = re.replace_all(js_code, |_: &Captures| {
        let random_dead_code = dead_code_snippets[rng.gen_range(0..dead_code_snippets.len())];
        format!("; {}", random_dead_code)
    });

    obfuscated_code.to_string()
}