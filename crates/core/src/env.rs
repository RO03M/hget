// Resolves {{variable}} placeholders in requests.
// Reads from .env files or environment-specific files.

use std::collections::HashMap;

pub type Env = HashMap<String, String>;

pub fn resolve(input: &str, env: &Env) -> String {
    let mut output = input.to_string();
    for (key, value) in env {
        output = output.replace(&format!("{{{{{key}}}}}"), value);
    }
    output
}
