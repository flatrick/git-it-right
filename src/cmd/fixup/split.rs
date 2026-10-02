use std::collections::BTreeSet;
use std::ffi::OsStr;

use super::{Hunk, Mode, Trace, Traced, base_hint, blame, cancelled, pick_branch_commit, subject, targets};
use crate::git;
use crate::pick::{self, Choice};

/// Whether `split` can go ahead: no hunk changes lines that several commits last changed.
/// An insertion between two commits' lines can still be asked about.
pub(super) fn can_split(hunks: &[Traced]) -> bool {
    !hunks.iter().any(|t| t.shas.len() > 1 && !t.hunk.insertion)
}

/// One autosquash commit per target, each holding only that target's hunks, and the whole
/// staged version of each file that could not be traced, in the commit chosen for it. Every
/// round resets a temporary index to the original `HEAD` and applies the hunks of this and
/// all earlier targets, so hunk line numbers always match and nothing is traced twice. The
/// working tree and the real index are never touched. `ask` allows questions for what
/// tracing cannot decide; without it, those cases are refused.
pub(super) fn split(mode: Mode, trace: Trace, base: Option<&str>, dry_run: bool, ask: bool) -> Result<i32, String> {
    let Trace { mut hunks, untraced } = trace;
    if let Some(spanning) = hunks.iter().find(|t| t.shas.len() > 1 && !t.hunk.insertion) {
        return Err(format!("{} spans several commits; split it with git add -p", spanning.hunk.place()));
    }
    if !ask {
        if let Some(u) = untraced.first() {
            return Err(format!(
                "{}; --split cannot place it: commit it on its own or unstage it (git restore --staged -- {}), then run gir {} --split again",
                u.reason,
                String::from_utf8_lossy(&u.path),
                mode.name()
            ));
        }
        if let Some(t) = hunks.iter().find(|t| t.shas.len() > 1) {
            let named: Vec<String> = neighbours(t).iter().map(|sha| format!("{} {}", &sha[..10], subject(sha))).collect();
            return Err(format!(
                "{} is an insertion between lines of {}; run gir {m} --split in a terminal to choose, or stage it on its own and run gir {m} <commit>",
                t.hunk.place(),
                named.join(" and "),
                m = mode.name()
            ));
        }
    }
    let mut placed: Vec<(Vec<u8>, String)> = Vec::new();
    for u in &untraced {
        match pick_branch_commit(&format!("{}; pick the commit it belongs to:", u.reason), base)? {
            Some(sha) => placed.push((u.path.clone(), sha)),
            None => return Ok(cancelled()),
        }
    }
    for t in hunks.iter_mut().filter(|t| t.shas.len() > 1) {
        let order = neighbours(t);
        let items: Vec<String> = order
            .iter()
            .zip(["line above", "line below"])
            .map(|(sha, side)| format!("{} {}  ({side})", &sha[..10], subject(sha)))
            .collect();
        eprintln!("gir: {} is an insertion between lines of two commits; pick the one it belongs to:", t.hunk.place());
        match pick::choose(&items, None, std::io::stdin().lock(), std::io::stderr()) {
            Choice::Item(i) => t.shas = BTreeSet::from([order[i].clone()]),
            Choice::Split | Choice::Cancel => return Ok(cancelled()),
        }
    }
    let order: Vec<String> = targets(&hunks).into_keys().chain(placed.iter().map(|(_, sha)| sha.clone())).collect::<BTreeSet<_>>().into_iter().collect();
    if dry_run {
        for sha in &order {
            crate::outln!("{} {}", &sha[..10], subject(sha));
        }
        return Ok(0);
    }
    let orig = git::run(&["rev-parse", "HEAD"])?;
    let goal = git::run(&["write-tree"])?;
    let result = commit_each(mode, &hunks, &placed, &order, &orig, &goal).and_then(|()| {
        if git::run(&["rev-parse", "HEAD^{tree}"])? != goal {
            return Err("the split commits do not add up to the staged changes".to_string());
        }
        Ok(())
    });
    if let Err(e) = result {
        let restored = git::run(&["reset", "-q", "--soft", &orig]);
        return Err(match restored {
            Ok(_) => format!("{e}; restored HEAD and the index"),
            Err(r) => format!("{e}; restoring failed ({r}); your staged changes are tree {goal} on top of {orig}"),
        });
    }
    for sha in &order {
        eprintln!("gir: created {} for {} {}", mode.prefix(), &sha[..10], subject(sha));
    }
    eprintln!("  fold: git rebase --autosquash {}", base_hint(base));
    Ok(0)
}

/// The eligible commits of an insertion's neighbours, the line above first.
fn neighbours(t: &Traced) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for line in &t.hunk.lines {
        for sha in blame(&t.hunk.path, &[*line]) {
            if t.shas.contains(&sha) && !found.contains(&sha) {
                found.push(sha);
            }
        }
    }
    found
}

fn commit_each(mode: Mode, hunks: &[Traced], placed: &[(Vec<u8>, String)], order: &[String], orig: &str, goal: &str) -> Result<(), String> {
    let index = git::TempIndex::new("gir-split-index")?;
    for (k, sha) in order.iter().enumerate() {
        let done = &order[..=k];
        let patch = patch(hunks.iter().filter(|t| t.shas.iter().all(|s| done.contains(s))).map(|t| &t.hunk));
        index.run(&["read-tree", orig])?;
        if !patch.is_empty() {
            index.run_with_stdin(&["apply", "--cached", "--unidiff-zero", "--whitespace=nowarn", "-"], &patch)?;
        }
        for (path, _) in placed.iter().filter(|(_, s)| done.contains(s)) {
            place_whole_file(&index, goal, path)?;
        }
        index.passthrough(&["commit", "--quiet", &mode.commit_arg(sha)])?;
    }
    Ok(())
}

/// Sets `path` in `index` to its entry in the staged tree `goal`, or removes it when the
/// staged tree has none.
fn place_whole_file(index: &git::TempIndex, goal: &str, path: &[u8]) -> Result<(), String> {
    let path = git::os_path(path);
    let entry = index.run_raw(&[OsStr::new("ls-tree"), OsStr::new(goal), OsStr::new("--"), &path])?;
    let entry = String::from_utf8_lossy(&entry);
    let fields: Vec<&str> = entry.split_whitespace().take(3).collect();
    let args: Vec<&OsStr> = match fields[..] {
        [mode, _, object] => vec![OsStr::new("update-index"), OsStr::new("--add"), OsStr::new("--cacheinfo"), OsStr::new(mode), OsStr::new(object), &path],
        _ => vec![OsStr::new("update-index"), OsStr::new("--force-remove"), OsStr::new("--"), &path],
    };
    index.run_raw(&args).map(|_| ())
}

/// Hunks in diff order, each file's header written once before its first hunk. git
/// places a zero-context hunk by its new-side start, which assumes every earlier hunk
/// was applied, so that start is recomputed from the hunks this patch includes.
fn patch<'a>(hunks: impl Iterator<Item = &'a Hunk>) -> Vec<u8> {
    let mut out = Vec::new();
    let mut last_header: Option<&[u8]> = None;
    let mut offset: i64 = 0;
    for h in hunks {
        if last_header != Some(h.header.as_slice()) {
            out.extend_from_slice(&h.header);
            last_header = Some(&h.header);
            offset = 0;
        }
        let first = i64::from(if h.old_count == 0 { h.old_start + 1 } else { h.old_start }) + offset;
        let new_start = if h.new_count == 0 { first - 1 } else { first };
        out.extend_from_slice(format!("@@ -{},{} +{new_start},{} @@\n", h.old_start, h.old_count, h.new_count).as_bytes());
        out.extend_from_slice(&h.changes);
        offset += i64::from(h.new_count) - i64::from(h.old_count);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::fixup::parse_hunks;

    #[test]
    fn renumbers_the_new_side_from_the_hunks_included() {
        let diff = "diff --git a/f b/f\n--- a/f\n+++ b/f\n@@ -1,0 +2 @@\n+X\n@@ -3,0 +5 @@\n+Y\n@@ -6,0 +9 @@\n+Z\n@@ -8 +10,0 @@\n-gone\n";
        let hunks = parse_hunks(diff.as_bytes());
        let header = "diff --git a/f b/f\n--- a/f\n+++ b/f\n";
        assert_eq!(patch(hunks.iter()), format!("{header}@@ -1,0 +2,1 @@\n+X\n@@ -3,0 +5,1 @@\n+Y\n@@ -6,0 +9,1 @@\n+Z\n@@ -8,1 +10,0 @@\n-gone\n").into_bytes());
        let skip_y = [&hunks[0], &hunks[2], &hunks[3]];
        assert_eq!(patch(skip_y.into_iter()), format!("{header}@@ -1,0 +2,1 @@\n+X\n@@ -6,0 +8,1 @@\n+Z\n@@ -8,1 +9,0 @@\n-gone\n").into_bytes());
        assert_eq!(patch(std::iter::once(&hunks[1])), format!("{header}@@ -3,0 +4,1 @@\n+Y\n").into_bytes());
    }
}
