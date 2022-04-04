use clap::{ArgEnum, Parser, Subcommand};
use console::{Emoji, Style};
use dialoguer::Confirm;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, fs, str};

/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[clap(name = "moji")]
#[clap(about = "Emoji toolkit for developers")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Parser)]
struct HookCli {
    #[clap(arg_enum)]
    hook: GitHook,
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

#[derive(ArgEnum, Clone, Debug)]
enum GitHook {
    /// A commit-msg hook that spell-checks emoji shortcodes
    CommitMsg,
    /// TODO
    PreCommit,
}

impl GitHook {
    fn as_str(&self) -> &'static str {
        match self {
            GitHook::CommitMsg => "commit-msg",
            GitHook::PreCommit => "pre-commit",
        }
    }

    fn as_path(&self) -> &Path {
        Path::new(self.as_str())
    }
}

struct PromptStyle {
    warning: Style,
    error: Style,
    path: Style,
    secondary: Style,
    code: Style,
}

/// Add an emoji shortcode spell checker to your git repo
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
        Some(Command::Hook(HookCli { hook })) => {
            let cwd = env::current_dir().unwrap();
            if check_is_git_repo(&cwd) {
                add_git_hook(hook, &cwd, &style).unwrap();
            } else {
                eprintln!(
                    "{} is not a git repo!",
                    style.path.apply_to(cwd.to_str().unwrap())
                );
            }
        }
        None => eprintln!(
            "{} Interactive search function not yet implemented {}",
            style.error.apply_to("[ERROR]:"),
            Emoji("\u{1F480}", "")
        ),
    };
}

fn check_is_git_repo(dir: &Path) -> bool {
    dir.join(Path::new(".git")).exists()
}

fn add_git_hook(hook: GitHook, repo_root: &Path, style: &PromptStyle) -> io::Result<()> {
    assert!(
        matches!(hook, GitHook::CommitMsg),
        "Only `commit-msg` hook is implemented!"
    );
    let hook_dir = Path::new(".git/hooks");
    let hook_path = hook_dir.join(hook.as_path());
    let full_hook_path = repo_root.join(&hook_path);
    if ask_file_write(&full_hook_path, hook_dir, style).unwrap() {
        write_hook_script(&hook, &hook_path)?;
        println!(
            "'{}' added to {}",
            style.path.apply_to(hook.as_str()),
            style.path.apply_to(hook_dir.to_str().unwrap()),
        );
        match fs::set_permissions(&hook_path, PermissionsExt::from_mode(0o770)) {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("{}: {}", style.error.apply_to("[ERROR]:"), e);
                println!(
                    "Run `{}` to set the script as executable",
                    style
                        .code
                        .apply_to(format!("chmod +x {}", hook_path.display()))
                );
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}

/// Ask if a file can be written
fn ask_file_write(rel_path: &Path, base_dir: &Path, style: &PromptStyle) -> Option<bool> {
    let target = base_dir.join(rel_path);
    // Ask if a user wants to overrite exiting file
    if target.exists() {
        println!(
            "{} '{}' already exists!",
            style.warning.apply_to("[WARNING]:"),
            style.path.apply_to(rel_path.display())
        );
        return Some(
            Confirm::new()
                .with_prompt(format!(
                    "Do you want to overwrite it? {}",
                    style.secondary.apply_to("(irreversible)")
                ))
                .default(false)
                .interact()
                .unwrap_or_default(),
        );
    }
    // Check if target directory exists
    if !base_dir.exists() {
        println!(
            "{} '{}' not found!",
            style.error.apply_to("[ERROR]:"),
            style.error.apply_to(base_dir.display())
        );
        return Some(false);
    }
    return Some(
        Confirm::new()
            .with_prompt(format!(
                "Write '{}' to '{}' ",
                style.path.apply_to(rel_path.file_name()?.to_str()?),
                style.path.apply_to(rel_path.parent()?.display()),
            ))
            .default(false)
            .interact()
            .unwrap_or_default(),
    );
}

/// Copy script into git hooks folder
fn write_hook_script(hook: &GitHook, target: &Path) -> io::Result<()> {
    // FIXME doesn't match to script type
    let script: Result<&[u8], ()> = match hook {
        GitHook::CommitMsg => Ok(include_bytes!("hooks/commit-msg.sh")),
        _ => unimplemented!()
    };
    fs::write(target, script.unwrap())
}
