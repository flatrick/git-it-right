use std::io::BufRead;
use std::path::Path;

use super::lint::{self, Options};
use crate::config::Config;

pub fn commit_msg(path: &str, cfg: &Config) -> Result<i32, String> {
    lint::lint_file(Path::new(path), &Options { fix: true, json: false }, cfg, "commit rejected")
}

/// Reads `<local ref> <local sha> <remote ref> <remote sha>` lines from stdin, as git passes them.
pub fn pre_push(remote: &str, cfg: &Config) -> Result<i32, String> {
    let mut outcomes = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let [_, local_sha, _, remote_sha] = parts[..] else { continue };
        if is_zero(local_sha) {
            continue;
        }
        let not_remote = format!("--remotes={remote}");
        let range = format!("{remote_sha}..{local_sha}");
        let commits = if is_zero(remote_sha) {
            lint::commits(&[local_sha, "--not", &not_remote])?
        } else {
            lint::commits(&[&range]).or_else(|_| lint::commits(&[local_sha, "--not", &not_remote]))?
        };
        outcomes.extend(lint::lint_recorded(&commits, cfg, true));
    }
    Ok(lint::emit(&outcomes, &Options { fix: false, json: false }, "push rejected"))
}

fn is_zero(sha: &str) -> bool {
    sha.bytes().all(|b| b == b'0')
}
