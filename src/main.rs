use console::Style;
use dialoguer::Confirm;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{env, fs};

/// Add an emoji shortcode spell checker to your git repo
fn main() -> io::Result<()> {
    let mut hook_dir = env::current_dir()?;
    let hook_name = Path::new("commit-msg");
    hook_dir.push(Path::new(".git/hooks"));
    if ask(&hook_dir, hook_name) {
        let hook_path = hook_dir.join(hook_name);
        generate_hook(&hook_path).unwrap();
        println!("'commit-msg' added to .git/hooks");
        match fs::set_permissions(&hook_path, PermissionsExt::from_mode(0o770)) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("{}", e);
                println!(
                    "Run 'chmod +x {}' to set the script as executable",
                    hook_path.display()
                )
            }
        }
    }
    Ok(())
}

/// Ask if a file can be written
fn ask(dir: &Path, fname: &Path) -> bool {
    let warn = Style::new().yellow().bold();
    let error = Style::new().red().bold();
    let dim = Style::new().dim();
    let blue = Style::new().bright().blue();
    if dir.join(fname).exists() {
        println!(
            "{} '{}' hook already exists!",
            warn.apply_to("[WARNING]:"),
            blue.apply_to(fname.display())
        );
        return Confirm::new()
            .with_prompt(format!(
                "Do you want to overwrite it? {}",
                dim.apply_to("(irreversible)")
            ))
            .default(false)
            .interact()
            .unwrap_or_default();
    }
    // Check if parent folders exist
    let parent = dir.parent().unwrap();
    if !parent.exists() {
        println!(
            "{} '{}' not found!",
            error.apply_to("[ERROR]:"),
            parent.to_str().unwrap()
        );
        return false;
    }
    return Confirm::new()
        .with_prompt(format!(
            "Add '{}' script to '{}' ",
            blue.apply_to(fname.display()),
            blue.apply_to(".git/hooks"),
        ))
        .default(false)
        .interact()
        .unwrap_or_default();
}

/// Copy script into git hooks folder
fn generate_hook(target: &PathBuf) -> io::Result<()> {
    let script = include_bytes!("hooks/commit-msg.sh");
    fs::write(target, script)
}
