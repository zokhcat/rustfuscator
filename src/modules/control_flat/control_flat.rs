use std::collections::HashMap;

use regex::Regex;

pub fn control_flat(js_code: &str) -> String {
    let re = Regex::new(r"function\s+([a-zA-Z_]\w*)\s*\((.*?)\)\s*\{([\s\S]*?)\}").unwrap();
    let mut flattened_code = String::new();
    let mut state_counter = 0;

    // Find all function definitions
    for caps in re.captures_iter(js_code) {
        let function_name = &caps[1];
        let params = &caps[2];
        let body = &caps[3];
        
        // Split function body into basic blocks
        let blocks: Vec<&str> = body.split(';').collect();
        let mut state_mapping: HashMap<usize, usize> = HashMap::new();

        // Start building the flattened function
        flattened_code.push_str(&format!("function {}({}) {{\n", function_name, params));
        flattened_code.push_str("    let state = 0;\n");
        flattened_code.push_str("    while (true) {\n");
        flattened_code.push_str("        switch (state) {\n");

        for (i, block) in blocks.iter().enumerate() {
            state_counter += 1;
            state_mapping.insert(i, state_counter);
            flattened_code.push_str(&format!("            case {}: \n", i));
            flattened_code.push_str(&format!("                {};\n", block.trim()));
            flattened_code.push_str(&format!("                state = {};\n", state_mapping.get(&(i + 1)).unwrap_or(&0)));
            flattened_code.push_str("                break;\n");
        }

        // Add final case to terminate the loop
        flattened_code.push_str("            case 0: return result;\n");
        flattened_code.push_str("        }\n");
        flattened_code.push_str("    }\n");
        flattened_code.push_str("}\n");
    }

    flattened_code
}