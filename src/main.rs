use regex::Regex;
use std::collections::HashMap;

/// A Precommit hook which throws an error if en emoji shortcode is misspelled
fn main() {
    let commit_msg_path = std::env::args().nth(1).expect("Commit not found");
    let commit_msg = std::fs::read_to_string(commit_msg_path).unwrap();
    let re: Regex = Regex::new(r#"(?::)(\S{1,})(?::)"#).unwrap();
    for x in re
        .captures_iter(&commit_msg)
        .map(|x| x.get(1).unwrap().as_str())
    {
        if !is_valid_emoji_shortcode(x) {
            eprintln!("Unknown emoji shortcode: {}", x);
            std::process::exit(1);
        }
    }
}

fn is_valid_emoji_shortcode(text: &str) -> bool {
    let bytes = include_bytes!("emojis.json");
    let emojis: HashMap<String, String> = serde_json::from_slice(bytes).unwrap();
    emojis.contains_key(text)
}
