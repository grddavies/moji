use crate::PromptStyle;
use clap::{ArgEnum, Parser};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use dialoguer::Confirm;
use std::{env, io, fs};

#[derive(Debug, Parser)]
pub struct HookCli {
    #[clap(arg_enum)]
    pub hook: GitHook,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum GitHook {
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

pub fn try_add_git_hook(hook: GitHook, style: PromptStyle) {
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
    if ask_file_write(&full_hook_path, style).unwrap() {
        write_hook_script(&hook, &full_hook_path)?;
        println!(
            "'{}' added to '{}'",
            style.path.apply_to(hook.as_str()),
            style.path.apply_to(hook_dir.to_str().unwrap()),
        );
        match fs::set_permissions(&hook_path, PermissionsExt::from_mode(0o770)) {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("{}: {}", style.error.apply_to("[ERROR]:"), e);
                let cmd = style
                    .code
                    .apply_to(format!("chmod +x {}", hook_path.display()));
                println!("Run `{}` to set the script as executable", cmd);
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}

/// Ask if a file can be written in a location
///
/// # Arguments
/// * `target` Reference to Path slice to get permission to write to
fn ask_file_write(target: &Path, style: &PromptStyle) -> Option<bool> {
    // Ask if a user wants to overrite exiting file
    if target.exists() {
        println!(
            "{} '{}' already exists!",
            style.warning.apply_to("[WARNING]:"),
            style.path.apply_to(target.display())
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
    let target_dir = target.parent()?;
    if !target_dir.exists() {
        println!(
            "{} '{}' not found!",
            style.error.apply_to("[ERROR]:"),
            style.path.apply_to(target_dir.display())
        );
        return Some(false);
    }
    return Some(
        Confirm::new()
            .with_prompt(format!(
                "Write '{}' to '{}'?",
                style.path.apply_to(target.file_name()?.to_str()?),
                style.path.apply_to(target.parent()?.display()),
            ))
            .default(false)
            .interact()
            .unwrap_or_default(),
    );
}

/// Copy script into git hooks folder
fn write_hook_script(hook: &GitHook, target: &Path) -> io::Result<()> {
    let script: Result<&[u8], ()> = match hook {
        GitHook::CommitMsg => Ok(include_bytes!("hooks/commit-msg.sh")),
        _ => unimplemented!(),
    };
    fs::write(target, script.unwrap())
}
