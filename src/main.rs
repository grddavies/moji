use clap::{Parser, Subcommand};
use std::str;
mod prompt;
use crate::prompt::PromptStyle;
mod hook;
mod search;

#[derive(Debug, Parser)]
#[clap(name = "moji", version, about = "Emoji toolkit for developers")]
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
    Hook(hook::HookCli),
}

fn main() {
    let args = Cli::parse();
    let style = PromptStyle::new();
    match args.command {
        Some(Command::Hook(hook_cli)) => hook::try_add_git_hook(hook_cli.hook, style),
        None => search::main(style),
    };
}
