use regex::Regex;
use std::collections::HashMap;

/// A Precommit hook which throws an error if en emoji shortcode is misspelled
fn main() {
    let commit_msg = std::env::args().nth(1).expect("No commit message!");
    let re: Regex = Regex::new(r#"(?::)(\S{1,})(?::)"#).unwrap();
    for x in re.captures_iter(&commit_msg) {
        if !lookup(x.get(1).unwrap().as_str()) {
            std::process::exit(1);
        }
    }
}

fn lookup(text: &str) -> bool {
    let bytes = include_bytes!("../emojis.json");
    let emojis: HashMap<String, String> = serde_json::from_slice(bytes).unwrap();
    emojis.contains_key(text)
}
