use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::git;

pub fn fixup(target: Option<String>, dry_run: bool) -> Result<i32, String> {
    git::enter_toplevel()?;
    if git::run(&["diff", "--cached", "--name-only"])?.is_empty() {
        return Err("nothing staged; `git add` the fix first (more: gir explain fixup)".into());
    }
    let base = find_base();
    let sha = match target {
        Some(t) => {
            let sha = git::run(&["rev-parse", "--verify", "--quiet", &format!("{t}^{{commit}}")])
                .map_err(|_| format!("`{t}` is not a commit"))?;
            if git::run(&["merge-base", "--is-ancestor", &sha, "HEAD"]).is_err() {
                return Err(format!("`{t}` is not in the current branch's history"));
            }
            if base.as_deref().is_some_and(|b| git::run(&["merge-base", "--is-ancestor", &sha, b]).is_ok()) {
                return Err(format!("`{t}` is already on the base branch"));
            }
            sha
        }
        None => auto_target(base.as_deref())?,
    };
    let subject = git::run(&["log", "-1", "--format=%s", &sha])?;
    let short = &sha[..10];
    if dry_run {
        println!("{short} {subject}");
        return Ok(0);
    }
    git::passthrough(&["commit", "--quiet", &format!("--fixup={sha}")])?;
    let base_hint = base.as_deref().map_or("<base>".to_string(), |b| b[..10].to_string());
    eprintln!("gir: created fixup! for {short} {subject}");
    eprintln!("  fold: git rebase --autosquash {base_hint}");
    Ok(0)
}

/// The commit where this branch forked from the default branch, or from its upstream when
/// already on the default branch. `None` means no restriction could be determined.
fn find_base() -> Option<String> {
    let head = git::run(&["rev-parse", "HEAD"]).ok()?;
    let candidates = ["refs/remotes/origin/HEAD", "refs/heads/main", "refs/heads/master", "@{upstream}"];
    for c in candidates {
        if let Ok(base) = git::run(&["merge-base", "HEAD", c])
            && base != head
        {
            return Some(base);
        }
    }
    None
}

#[derive(Debug)]
struct Hunk {
    path: String,
    lines: Vec<u32>,
}

fn auto_target(base: Option<&str>) -> Result<String, String> {
    let diff = git::run(&[
        "-c", "core.quotePath=false", "diff", "--cached", "-U0", "--no-color", "--no-ext-diff",
        "--no-renames", "--src-prefix=a/", "--dst-prefix=b/",
    ])?;
    let hunks = parse_hunks(&diff)?;
    let traced: BTreeSet<&str> = hunks.iter().map(|h| h.path.as_str()).collect();
    let staged = git::run(&["-c", "core.quotePath=false", "diff", "--cached", "--name-only", "--no-renames"])?;
    if let Some(path) = staged.lines().find(|p| !traced.contains(p)) {
        return Err(format!("cannot tell which commit {path} belongs to; pass one: gir fixup <commit>"));
    }
    let allowed: Option<HashSet<String>> = match base {
        Some(b) => Some(git::run(&["rev-list", &format!("{b}..HEAD")])?.lines().map(str::to_string).collect()),
        None => None,
    };

    let mut targets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for hunk in &hunks {
        let blamed = blame(&hunk.path, &hunk.lines);
        let first_line = hunk.lines.first().copied().unwrap_or(1);
        let place = format!("{}:{first_line}", hunk.path);
        if blamed.is_empty() {
            return Err(format!("cannot tell which commit {place} belongs to; pass one: gir fixup <commit>"));
        }
        let in_range: BTreeSet<&String> = blamed.iter().filter(|s| allowed.as_ref().is_none_or(|a| a.contains(*s))).collect();
        if in_range.is_empty() {
            let outside = blamed.iter().next().unwrap();
            return Err(format!(
                "{place} was last changed by {} which is already on the base branch; commit it normally instead",
                &outside[..10]
            ));
        }
        for sha in in_range {
            targets.entry(sha.clone()).or_default().push(place.clone());
        }
    }

    if targets.len() == 1 {
        return Ok(targets.into_keys().next().unwrap());
    }
    let mut msg = String::from("staged changes belong to several commits:");
    for (sha, places) in &targets {
        let subject = git::run(&["log", "-1", "--format=%s", sha]).unwrap_or_default();
        msg.push_str(&format!("\n  {} {subject}  <- {}", &sha[..10], places.join(" ")));
    }
    msg.push_str("\n  split: git restore --staged . && git add -p, then one gir fixup per commit");
    if git::on_path("git-absorb") {
        msg.push_str("\n  or: git absorb (installed) creates one fixup per commit");
    }
    Err(msg)
}

/// Lines in the HEAD version each hunk touches. A pure insertion has no old
/// lines, so the lines around it stand in for it.
fn parse_hunks(diff: &str) -> Result<Vec<Hunk>, String> {
    let mut hunks = Vec::new();
    let mut path: Option<String> = None;
    for line in diff.lines() {
        if let Some(p) = line.strip_prefix("--- ") {
            path = match p {
                "/dev/null" => None,
                p => Some(p.trim_matches('"').strip_prefix("a/").unwrap_or(p).to_string()),
            };
        } else if line.starts_with("+++ b/") && path.is_none() {
            let new = line.trim_start_matches("+++ b/");
            return Err(format!("{new} is a new file, so it has no earlier commit; pass one: gir fixup <commit>"));
        } else if let Some(h) = line.strip_prefix("@@ -") {
            let old = h.split_whitespace().next().unwrap_or("");
            let (start, count) = match old.split_once(',') {
                Some((s, c)) => (s.parse::<u32>().unwrap_or(0), c.parse::<u32>().unwrap_or(0)),
                None => (old.parse::<u32>().unwrap_or(0), 1),
            };
            let lines = if count == 0 {
                [start, start + 1].into_iter().filter(|l| *l >= 1).collect()
            } else {
                (start..start + count).collect()
            };
            if let Some(p) = &path {
                hunks.push(Hunk { path: p.clone(), lines });
            }
        }
    }
    Ok(hunks)
}

fn blame(path: &str, lines: &[u32]) -> BTreeSet<String> {
    let mut shas = BTreeSet::new();
    for line in lines {
        let range = format!("{line},{line}");
        if let Ok(out) = git::run(&["blame", "-l", "-s", "-L", &range, "HEAD", "--", path])
            && let Some(sha) = out.split_whitespace().next()
        {
            shas.insert(sha.trim_start_matches('^').to_string());
        }
    }
    shas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_modifications_and_insertions() {
        let diff = "diff --git a/x b/x\n--- a/src/x.rs\n+++ b/src/x.rs\n@@ -3,2 +3,2 @@\n-a\n-b\n+c\n+d\n@@ -10,0 +11 @@\n+e\n@@ -20 +21 @@\n-f\n+g\n";
        let hunks = parse_hunks(diff).unwrap();
        assert_eq!(hunks.iter().map(|h| h.lines.clone()).collect::<Vec<_>>(), vec![vec![3, 4], vec![10, 11], vec![20]]);
        assert!(hunks.iter().all(|h| h.path == "src/x.rs"));
    }

    #[test]
    fn new_files_need_an_explicit_target() {
        let diff = "--- /dev/null\n+++ b/new.rs\n@@ -0,0 +1 @@\n+x\n";
        assert!(parse_hunks(diff).unwrap_err().contains("new.rs is a new file"));
    }
}
