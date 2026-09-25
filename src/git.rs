use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Runs `git` in the current directory and returns trimmed stdout, or trimmed stderr on failure.
pub fn run(args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("could not run git: {e}"))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim_end().to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

pub fn run_with_stdin(args: &[&str], input: &str) -> Result<String, String> {
    let mut child = Command::new("git")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("could not run git: {e}"))?;
    child.stdin.take().unwrap().write_all(input.as_bytes()).map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim_end().to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

/// Runs git with inherited stdio so the user sees git's own output.
pub fn passthrough(args: &[&str]) -> Result<(), String> {
    let status = Command::new("git").args(args).status().map_err(|e| format!("could not run git: {e}"))?;
    if status.success() { Ok(()) } else { Err(format!("git {} failed", args.join(" "))) }
}

pub fn get_config(key: &str) -> Option<String> {
    run(&["config", "--get", key]).ok().filter(|v| !v.is_empty())
}

pub fn toplevel() -> Option<PathBuf> {
    run(&["rev-parse", "--show-toplevel"]).ok().map(PathBuf::from)
}

/// Makes the repository root the current directory, so root-relative paths from git work from any subdirectory.
pub fn enter_toplevel() -> Result<PathBuf, String> {
    let root = toplevel().ok_or("not inside a git repository")?;
    std::env::set_current_dir(&root).map_err(|e| format!("cannot enter {}: {e}", root.display()))?;
    Ok(root)
}

/// `core.commentString` (git >= 2.45) wins over `core.commentChar`; `auto` falls back to `#`.
pub fn comment_string() -> String {
    get_config("core.commentString")
        .or_else(|| get_config("core.commentChar"))
        .filter(|c| c != "auto")
        .unwrap_or_else(|| "#".to_string())
}

pub fn on_path(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}
