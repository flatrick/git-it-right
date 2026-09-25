use std::path::Path;

use super::templates::{self, HOOKS};
use crate::config::{self, Config};
use crate::git;

/// Converges the repo to the gir setup; re-running changes nothing that is already right.
pub fn init(force: bool) -> Result<i32, String> {
    let root = git::enter_toplevel()?;
    let mut conflicts = 0;

    let mut hook_paths = Vec::new();
    for name in HOOKS {
        let rel = format!(".githooks/{name}");
        let outcome = write(&root, &rel, &templates::hook_shim(name), force)?;
        conflicts += i32::from(outcome == WriteOutcome::Kept);
        let entry = git::run(&["ls-files", "--stage", "--", &rel])?;
        if outcome == WriteOutcome::Written || !entry.starts_with("100755 ") {
            hook_paths.push(rel);
        }
    }
    conflicts += i32::from(write(&root, config::FILE, &config::default_file(), force)? == WriteOutcome::Kept);
    let cfg = Config::load_from(&root.join(config::FILE))?;
    conflicts += i32::from(write(&root, "cliff.toml", &templates::cliff_toml(&cfg), force)? == WriteOutcome::Kept);

    if !hook_paths.is_empty() {
        let mut add = vec!["add", "--chmod=+x", "--"];
        add.extend(hook_paths.iter().map(String::as_str));
        git::run(&add)?;
        eprintln!("gir: staged {} as executable", hook_paths.join(" "));
    }

    if git::get_config("core.hooksPath").as_deref() != Some(".githooks") {
        git::run(&["config", "--local", "core.hooksPath", ".githooks"])?;
        eprintln!("gir: set core.hooksPath=.githooks (this clone)");
    }
    eprintln!("gir: next: gir doctor");
    Ok(i32::from(conflicts > 0))
}

#[derive(PartialEq, Eq)]
enum WriteOutcome {
    Unchanged,
    Kept,
    Written,
}

fn write(root: &Path, rel: &str, content: &str, force: bool) -> Result<WriteOutcome, String> {
    let path = root.join(rel);
    match std::fs::read_to_string(&path) {
        Ok(existing) if existing.replace("\r\n", "\n") == content => return Ok(WriteOutcome::Unchanged),
        Ok(_) if !force => {
            eprintln!("gir: kept {rel} (differs from the template; --force overwrites)");
            return Ok(WriteOutcome::Kept);
        }
        _ => {}
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, content).map_err(|e| format!("cannot write {rel}: {e}"))?;
    #[cfg(unix)]
    if rel.starts_with(".githooks/") {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).map_err(|e| e.to_string())?;
    }
    eprintln!("gir: wrote {rel}");
    Ok(WriteOutcome::Written)
}
