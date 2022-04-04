use clap::{Parser, Subcommand};
use console::{Emoji, Style};
use std::str;

mod hook;
use crate::hook::HookCli;

#[derive(Debug, Parser)]
#[clap(name = "moji")]
#[clap(about = "Emoji toolkit for developers")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Add git hooks to help emojify your git repo
    ///
    /// Available hooks:
    ///
    /// `commit-msg` : spell check emoji shortcodes in your commit message
    Hook(HookCli),
}

/// Styling for command line output
pub struct PromptStyle {
    warning: Style,
    error: Style,
    path: Style,
    secondary: Style,
    code: Style,
}

fn main() {
    let args = Cli::parse();
    let style: PromptStyle = PromptStyle {
        warning: Style::new().yellow().bold(),
        error: Style::new().red().bold(),
        path: Style::new().bright().blue(),
        secondary: Style::new().dim(),
        code: Style::new().magenta(),
    };
    match args.command {
        Some(Command::Hook(HookCli { hook })) => hook::try_add_git_hook(hook, style),
        None => eprintln!(
            "{} Interactive search function not yet implemented {}",
            style.error.apply_to("[ERROR]:"),
            Emoji("\u{1F480}", "")
        ),
    };
}
