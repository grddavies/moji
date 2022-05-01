use crate::PromptStyle;
use console::{Key, Term};

pub fn interactive(style: PromptStyle) {
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
    term.write_line(&format!("{}", style.secondary.apply_to("Searched:")))
        .unwrap();
    term.write_line(&query).unwrap();
}
