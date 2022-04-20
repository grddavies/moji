use crate::PromptStyle;
use console::Emoji;

pub fn interactive(style: PromptStyle) {
    eprintln!(
        "{} Interactive search function not yet implemented {}",
        style.error.apply_to("[ERROR]:"),
        Emoji("\u{1F480}", "")
    )
}
