use rand::{thread_rng, Rng};
use regex::Regex;

pub fn number_obs(js_code: &str) -> String {
    let re = Regex::new(r"\b\d+\b").unwrap();

    re.replace_all(js_code, |caps: &regex::Captures| {
        let number_str = &caps[0];
        let number: i32 = number_str.parse().unwrap_or(0);

        let mut rng = thread_rng();
        let method = rng.gen_range(0..3);

        let obfuscated = match method {
            0 => format!("{} + {}", number - 1, 1),
            1 => format!("{} * {}", number / 2, 2),
            2 => format!("{} - {}", number + 1, 1),
            _ => number.to_string(),
        };

        obfuscated
    })
    .to_string()
}
