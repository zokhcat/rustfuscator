use rand::{thread_rng, Rng};
use regex::Regex;

fn find_functions(js_code: &str) -> Vec<String> {
    let re = Regex::new(r"function\s+\w+\s*\([^)]*\)\s*\{([^}]*)\}").unwrap();
    re
        .captures_iter(js_code)
        .map(|cap| cap[0].to_string())
        .collect()
}

pub fn split_function(js_code: &str) -> String {
    let e_funcs = find_functions(js_code);

    let mut obfuscated_code = js_code.to_string();

    let mut rng = thread_rng();

    for func in e_funcs {
        let body_regex = Regex::new(r"\{(.*)\}").unwrap();
        if let Some(cap) = body_regex.captures(&func) {
            let body = &cap[1];
            let statements: Vec<&str> = body.split(';').collect();
            let mid = statements.len() / 2;
            let (part1, part2) = statements.split_at(mid);

            // Generate new function names and bodies
            let new_func1 = format!(
                "function func_{}() {{{}}}",
                rng.gen_range(1000..9999),
                part1.join(";")
            );

            let new_func2 = format!(
                "function func_{}() {{{}}}",
                rng.gen_range(1000..9999),
                part2.join(";")
            );

            // Replace the original function with the new split functions
            obfuscated_code = obfuscated_code.replace(&func, &format!("{}\n{}", new_func1, new_func2));
        }
    }
    obfuscated_code
}