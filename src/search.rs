use crate::PromptStyle;
use console::{Key, Term};
use std::path::Path;
use sublime_fuzzy::{format_simple, best_match};

pub fn main(style: PromptStyle) {
    let term = Term::stdout();
    let mut query = String::new();
    term.write_str("\u{1F50E} ").unwrap();
    loop {
        match term.read_key().unwrap() {
            Key::Enter => break,
            Key::Backspace => {
                query.pop();
                term.clear_chars(1).unwrap()
            }
            Key::Char(c) => {
                query.push(c);
                let mut tmp = [0u8; 4];
                term.write_str(c.encode_utf8(&mut tmp)).unwrap()
            }
            _ => (),
        }
    }
    term.clear_line().unwrap();
    term.write_line(&format!("{}", style.secondary.apply_to("Result:")))
        .unwrap();
    let search_result = search(&query);
    term.write_line(&search_result).unwrap();
}

fn search(query: &String) -> String {
    let path = Path::new("/home/gethin/.moji/emojis.json");
    let emojis = std::fs::read_to_string(path).unwrap();
    // let result = FuzzySearch::new(&query, &emojis).best_match();
    let result = best_match(&query, &emojis);
    match result {
        Some(s) => format_simple(&s, &emojis, "<matched>", "</matched>"),
        None => String::new()
    }
}
