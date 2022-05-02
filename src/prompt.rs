use console::Style;

/// Styling for command line output
pub struct PromptStyle {
    pub warning: Style,
    pub error: Style,
    pub info: Style,
    pub path: Style,
    pub secondary: Style,
    pub code: Style,
}

impl PromptStyle {
    pub fn new() -> PromptStyle {
        PromptStyle {
            warning: Style::new().yellow(),
            error: Style::new().bright().red(),
            info: Style::new().bright().blue(),
            path: Style::new().bright().blue(),
            secondary: Style::new().dim(),
            code: Style::new().magenta(),
        }
    }
}
