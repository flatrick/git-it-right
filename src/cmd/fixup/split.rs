use super::{Hunk, Mode, Traced, base_hint, subject, targets};
use crate::git;

/// One autosquash commit per target, each holding only that target's hunks. Every round
/// resets a temporary index to the original `HEAD` and applies the hunks of this and all
/// earlier targets, so hunk line numbers always match and nothing is traced twice. The
/// working tree and the real index are never touched.
pub(super) fn split(mode: Mode, hunks: &[Traced], base: Option<&str>, dry_run: bool) -> Result<i32, String> {
    if let Some(spanning) = hunks.iter().find(|t| t.shas.len() > 1) {
        return Err(format!("{} spans several commits; split it with git add -p", spanning.hunk.place()));
    }
    let order: Vec<String> = targets(hunks).into_keys().collect();
    if dry_run {
        for sha in &order {
            crate::outln!("{} {}", &sha[..10], subject(sha));
        }
        return Ok(0);
    }
    let orig = git::run(&["rev-parse", "HEAD"])?;
    let goal = git::run(&["write-tree"])?;
    let result = commit_each(mode, hunks, &order, &orig).and_then(|()| {
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

fn commit_each(mode: Mode, hunks: &[Traced], order: &[String], orig: &str) -> Result<(), String> {
    let index = git::TempIndex::new("gir-split-index")?;
    for (k, sha) in order.iter().enumerate() {
        let done = &order[..=k];
        let patch = patch(hunks.iter().filter(|t| t.shas.iter().all(|s| done.contains(s))).map(|t| &t.hunk));
        index.run(&["read-tree", orig])?;
        index.run_with_stdin(&["apply", "--cached", "--unidiff-zero", "-"], &patch)?;
        index.passthrough(&["commit", "--quiet", &mode.commit_arg(sha)])?;
    }
    Ok(())
}

/// Hunks in diff order, each file's header written once before its first hunk. git
/// places a zero-context hunk by its new-side start, which assumes every earlier hunk
/// was applied, so that start is recomputed from the hunks this patch includes.
fn patch<'a>(hunks: impl Iterator<Item = &'a Hunk>) -> String {
    let mut out = String::new();
    let mut last_header: Option<&str> = None;
    let mut offset: i64 = 0;
    for h in hunks {
        if last_header != Some(h.header.as_str()) {
            out.push_str(&h.header);
            last_header = Some(&h.header);
            offset = 0;
        }
        let first = i64::from(if h.old_count == 0 { h.old_start + 1 } else { h.old_start }) + offset;
        let new_start = if h.new_count == 0 { first - 1 } else { first };
        out.push_str(&format!("@@ -{},{} +{new_start},{} @@\n", h.old_start, h.old_count, h.new_count));
        out.push_str(&h.changes);
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
        let hunks = parse_hunks(diff).unwrap();
        let header = "diff --git a/f b/f\n--- a/f\n+++ b/f\n";
        assert_eq!(patch(hunks.iter()), format!("{header}@@ -1,0 +2,1 @@\n+X\n@@ -3,0 +5,1 @@\n+Y\n@@ -6,0 +9,1 @@\n+Z\n@@ -8,1 +10,0 @@\n-gone\n"));
        let skip_y = [&hunks[0], &hunks[2], &hunks[3]];
        assert_eq!(patch(skip_y.into_iter()), format!("{header}@@ -1,0 +2,1 @@\n+X\n@@ -6,0 +8,1 @@\n+Z\n@@ -8,1 +9,0 @@\n-gone\n"));
        assert_eq!(patch(std::iter::once(&hunks[1])), format!("{header}@@ -3,0 +4,1 @@\n+Y\n"));
    }
}
