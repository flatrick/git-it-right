use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Runs `git` in the current directory and returns trimmed stdout, or trimmed stderr on failure.
pub fn run(args: &[&str]) -> Result<String, String> {
    output(Command::new("git").args(args))
}

/// Runs `git` and returns its stdout exactly as written, for output that holds file contents
/// or paths, which need not be UTF-8 and may end in whitespace.
pub fn run_raw<S: AsRef<OsStr>>(args: &[S]) -> Result<Vec<u8>, String> {
    let out = Command::new("git").args(args).stdin(Stdio::null()).output().map_err(|e| format!("could not run git: {e}"))?;
    if out.status.success() { Ok(out.stdout) } else { Err(String::from_utf8_lossy(&out.stderr).trim().to_string()) }
}

pub fn run_with_stdin(args: &[&str], input: &str) -> Result<String, String> {
    output_with_stdin(Command::new("git").args(args), input.as_bytes())
}

/// Runs git with inherited stdio so the user sees git's own output.
pub fn passthrough(args: &[&str]) -> Result<(), String> {
    status(Command::new("git").args(args), args)
}

fn output(cmd: &mut Command) -> Result<String, String> {
    let out = cmd.stdin(Stdio::null()).output().map_err(|e| format!("could not run git: {e}"))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim_end().to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

fn output_with_stdin(cmd: &mut Command, input: &[u8]) -> Result<String, String> {
    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("could not run git: {e}"))?;
    child.stdin.take().unwrap().write_all(input).map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim_end().to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

fn status(cmd: &mut Command, args: &[&str]) -> Result<(), String> {
    let status = cmd.status().map_err(|e| format!("could not run git: {e}"))?;
    if status.success() { Ok(()) } else { Err(format!("git {} failed", args.join(" "))) }
}

/// An index file of gir's own in the git directory, so commits can be built without
/// touching the staged changes. The file is removed when this is dropped.
pub struct TempIndex(PathBuf);

impl TempIndex {
    pub fn new(name: &str) -> Result<TempIndex, String> {
        let dir = run(&["rev-parse", "--absolute-git-dir"])?;
        Ok(TempIndex(Path::new(&dir).join(format!("{name}-{}", std::process::id()))))
    }

    fn git(&self, args: &[&str]) -> Command {
        let mut cmd = Command::new("git");
        cmd.args(args).env("GIT_INDEX_FILE", &self.0);
        cmd
    }

    pub fn run(&self, args: &[&str]) -> Result<String, String> {
        output(&mut self.git(args))
    }

    pub fn run_with_stdin(&self, args: &[&str], input: &[u8]) -> Result<String, String> {
        output_with_stdin(&mut self.git(args), input)
    }

    pub fn passthrough(&self, args: &[&str]) -> Result<(), String> {
        status(&mut self.git(args), args)
    }

    pub fn run_raw<S: AsRef<OsStr>>(&self, args: &[S]) -> Result<Vec<u8>, String> {
        let out = Command::new("git")
            .args(args)
            .env("GIT_INDEX_FILE", &self.0)
            .stdin(Stdio::null())
            .output()
            .map_err(|e| format!("could not run git: {e}"))?;
        if out.status.success() { Ok(out.stdout) } else { Err(String::from_utf8_lossy(&out.stderr).trim().to_string()) }
    }
}

impl Drop for TempIndex {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
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
