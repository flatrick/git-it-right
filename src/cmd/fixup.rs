use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::git;
use crate::pick::{self, Choice};

mod split;

/// The autosquash commit to create; each maps to one `git commit` option.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Fixup,
    Amend,
    Reword,
    Squash,
}

impl Mode {
    pub fn name(self) -> &'static str {
        match self {
            Mode::Fixup => "fixup",
            Mode::Amend => "amend",
            Mode::Reword => "reword",
            Mode::Squash => "squash",
        }
    }

    fn prefix(self) -> &'static str {
        match self {
            Mode::Fixup => "fixup!",
            Mode::Amend | Mode::Reword => "amend!",
            Mode::Squash => "squash!",
        }
    }

    fn commit_arg(self, sha: &str) -> String {
        match self {
            Mode::Fixup => format!("--fixup={sha}"),
            Mode::Amend => format!("--fixup=amend:{sha}"),
            Mode::Reword => format!("--fixup=reword:{sha}"),
            Mode::Squash => format!("--squash={sha}"),
        }
    }

    fn pass_one(self) -> String {
        format!("pass one: gir {} <commit>", self.name())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub dry_run: bool,
    pub split: bool,
}

pub fn run(mode: Mode, target: Option<String>, opts: Options) -> Result<i32, String> {
    git::enter_toplevel()?;
    if mode != Mode::Reword && git::run(&["diff", "--cached", "--name-only"])?.is_empty() {
        return Err(match mode {
            Mode::Amend => "nothing staged; `git add` the fix first, or change only the message: gir reword (more: gir explain fixup)",
            _ => "nothing staged; `git add` the fix first (more: gir explain fixup)",
        }
        .into());
    }
    if opts.split && target.is_some() {
        return Err("--split finds each commit itself; drop the commit argument".into());
    }
    let base = find_base();
    let ask = !opts.dry_run && pick::interactive();
    let sha = match target {
        Some(t) => explicit_target(&t, base.as_deref())?,
        None if mode == Mode::Reword => {
            let reason = "nothing to trace for a reword";
            if !ask {
                return Err(format!("{reason}; {}", mode.pass_one()));
            }
            match pick_branch_commit(&format!("{reason}; pick the commit to reword:"), base.as_deref())? {
                Some(sha) => sha,
                None => return Ok(cancelled()),
            }
        }
        None => match trace(base.as_deref())? {
            Trace::Unattributed(reason) if ask => match pick_branch_commit(&format!("{reason}; pick the commit it belongs to:"), base.as_deref())? {
                Some(sha) => sha,
                None => return Ok(cancelled()),
            },
            Trace::Unattributed(reason) => return Err(format!("{reason}; {}", mode.pass_one())),
            Trace::Hunks(hunks) if opts.split => return split::split(mode, &hunks, base.as_deref(), opts.dry_run),
            Trace::Hunks(hunks) => {
                let targets = targets(&hunks);
                if targets.len() == 1 {
                    targets.into_keys().next().unwrap()
                } else if !ask {
                    return Err(several_targets(mode, &targets));
                } else {
                    let items: Vec<String> = targets
                        .iter()
                        .map(|(sha, places)| format!("{} {}  <- {}", &sha[..10], subject(sha), places.join(" ")))
                        .collect();
                    eprintln!("gir: staged changes belong to several commits:");
                    let split = format!("one {} per commit", mode.prefix());
                    match pick::choose(&items, Some(&split), std::io::stdin().lock(), std::io::stderr()) {
                        Choice::Item(i) => {
                            let sha = targets.into_keys().nth(i).unwrap();
                            eprintln!("gir: all staged changes go into one {} for {}", mode.prefix(), &sha[..10]);
                            sha
                        }
                        Choice::Split => return split::split(mode, &hunks, base.as_deref(), false),
                        Choice::Cancel => return Ok(cancelled()),
                    }
                }
            }
        },
    };
    let subject = subject(&sha);
    if opts.dry_run {
        crate::outln!("{} {subject}", &sha[..10]);
        return Ok(0);
    }
    commit(mode, &sha)?;
    eprintln!("gir: created {} for {} {subject}", mode.prefix(), &sha[..10]);
    eprintln!("  fold: git rebase --autosquash {}", base_hint(base.as_deref()));
    Ok(0)
}

fn explicit_target(t: &str, base: Option<&str>) -> Result<String, String> {
    let sha = git::run(&["rev-parse", "--verify", "--quiet", &format!("{t}^{{commit}}")])
        .map_err(|_| format!("`{t}` is not a commit"))?;
    if git::run(&["merge-base", "--is-ancestor", &sha, "HEAD"]).is_err() {
        return Err(format!("`{t}` is not in the current branch's history"));
    }
    if base.is_some_and(|b| git::run(&["merge-base", "--is-ancestor", &sha, b]).is_ok()) {
        return Err(format!("`{t}` is already on the base branch"));
    }
    Ok(sha)
}

fn commit(mode: Mode, sha: &str) -> Result<(), String> {
    git::passthrough(&["commit", "--quiet", &mode.commit_arg(sha)])
}

fn subject(sha: &str) -> String {
    git::run(&["log", "-1", "--format=%s", sha]).unwrap_or_default()
}

fn base_hint(base: Option<&str>) -> String {
    base.map_or("<base>".to_string(), |b| b[..10].to_string())
}

fn cancelled() -> i32 {
    eprintln!("gir: cancelled; nothing committed");
    1
}

/// Asks for one of the newest commits after the base, or on `HEAD` when no base is found.
/// `None` means the user cancelled.
fn pick_branch_commit(question: &str, base: Option<&str>) -> Result<Option<String>, String> {
    let range = base.map_or("HEAD".to_string(), |b| format!("{b}..HEAD"));
    let shas: Vec<String> = git::run(&["rev-list", "--max-count=20", &range])?.lines().map(str::to_string).collect();
    if shas.is_empty() {
        return Err("this branch has no commits after its base to pick from".into());
    }
    let items: Vec<String> = shas.iter().map(|sha| format!("{} {}", &sha[..10], subject(sha))).collect();
    eprintln!("gir: {question}");
    Ok(match pick::choose(&items, None, std::io::stdin().lock(), std::io::stderr()) {
        Choice::Item(i) => Some(shas[i].clone()),
        Choice::Split | Choice::Cancel => None,
    })
}

fn several_targets(mode: Mode, targets: &BTreeMap<String, Vec<String>>) -> String {
    let mut msg = String::from("staged changes belong to several commits:");
    for (sha, places) in targets {
        msg.push_str(&format!("\n  {} {}  <- {}", &sha[..10], subject(sha), places.join(" ")));
    }
    msg.push_str(&format!("\n  split: git restore --staged . && git add -p, then one gir {} per commit", mode.name()));
    msg.push_str(&format!("\n  or: gir {} --split creates one {} per commit", mode.name(), mode.prefix()));
    if mode == Mode::Fixup && git::on_path("git-absorb") {
        msg.push_str("\n  or: git absorb (installed) creates one fixup per commit");
    }
    msg
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
    insertion: bool,
    /// The file's `diff --git` header lines, for rebuilding a patch.
    header: String,
    /// The `@@` ranges as git writes them for `-U0`: a zero count's start is the line before.
    old_start: u32,
    old_count: u32,
    new_count: u32,
    /// The hunk's `-`, `+` and `\` lines.
    changes: String,
}

impl Hunk {
    fn place(&self) -> String {
        format!("{}:{}", self.path, self.lines.first().copied().unwrap_or(1))
    }
}

/// A staged hunk and the eligible commits that last changed its lines.
#[derive(Debug)]
struct Traced {
    hunk: Hunk,
    shas: BTreeSet<String>,
}

enum Trace {
    Hunks(Vec<Traced>),
    /// A staged file that no commit can be named for; the reason reads as a clause.
    Unattributed(String),
}

fn targets(hunks: &[Traced]) -> BTreeMap<String, Vec<String>> {
    let mut targets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for traced in hunks {
        for sha in &traced.shas {
            targets.entry(sha.clone()).or_default().push(traced.hunk.place());
        }
    }
    targets
}

fn trace(base: Option<&str>) -> Result<Trace, String> {
    let diff = git::run(&[
        "-c", "core.quotePath=false", "diff", "--cached", "-U0", "--no-color", "--no-ext-diff",
        "--no-renames", "--src-prefix=a/", "--dst-prefix=b/",
    ])?;
    let hunks = match parse_hunks(&diff) {
        Ok(hunks) => hunks,
        Err(reason) => return Ok(Trace::Unattributed(reason)),
    };
    let traced: BTreeSet<&str> = hunks.iter().map(|h| h.path.as_str()).collect();
    let staged = git::run(&["-c", "core.quotePath=false", "diff", "--cached", "--name-status", "--no-renames"])?;
    if let Some((status, path)) = staged.lines().filter_map(|l| l.split_once('\t')).find(|(_, p)| !traced.contains(p)) {
        return Ok(Trace::Unattributed(if status == "A" {
            format!("{path} is a new file, so it has no earlier commit")
        } else {
            format!("cannot tell which commit {path} belongs to")
        }));
    }
    let allowed: Option<HashSet<String>> = match base {
        Some(b) => Some(git::run(&["rev-list", &format!("{b}..HEAD")])?.lines().map(str::to_string).collect()),
        None => None,
    };

    let mut out = Vec::new();
    for hunk in hunks {
        let blamed = blame(&hunk.path, &hunk.lines);
        let place = hunk.place();
        if blamed.is_empty() {
            return Ok(Trace::Unattributed(format!("cannot tell which commit {place} belongs to")));
        }
        let in_range: BTreeSet<String> = blamed.iter().filter(|s| allowed.as_ref().is_none_or(|a| a.contains(*s))).cloned().collect();
        let on_base = blamed.iter().find(|s| !in_range.contains(*s));
        if let Some(outside) = on_base.filter(|_| in_range.is_empty() || !hunk.insertion) {
            return Err(format!(
                "{place} was last changed by {} which is already on the base branch; commit it normally instead",
                &outside[..10]
            ));
        }
        out.push(Traced { hunk, shas: in_range });
    }
    Ok(Trace::Hunks(out))
}

/// Lines in the HEAD version each hunk touches. A pure insertion has no old
/// lines, so the lines around it stand in for it. A new file is an error.
fn parse_hunks(diff: &str) -> Result<Vec<Hunk>, String> {
    let mut hunks: Vec<Hunk> = Vec::new();
    let mut path: Option<String> = None;
    let mut header = String::new();
    let mut in_header = false;
    for line in diff.lines() {
        if line.starts_with("diff --git ") {
            header.clear();
            in_header = true;
        }
        if in_header && !line.starts_with("@@ ") {
            header.push_str(line);
            header.push('\n');
        }
        if let Some(p) = line.strip_prefix("--- ") {
            path = match p {
                "/dev/null" => None,
                p => Some(p.trim_matches('"').strip_prefix("a/").unwrap_or(p).to_string()),
            };
        } else if line.starts_with("+++ b/") && path.is_none() {
            let new = line.trim_start_matches("+++ b/");
            return Err(format!("{new} is a new file, so it has no earlier commit"));
        } else if let Some(h) = line.strip_prefix("@@ -") {
            in_header = false;
            let mut ranges = h.split_whitespace();
            let (start, count) = range(ranges.next().unwrap_or(""));
            let (_, new_count) = range(ranges.next().unwrap_or("").trim_start_matches('+'));
            let lines = if count == 0 {
                [start, start + 1].into_iter().filter(|l| *l >= 1).collect()
            } else {
                (start..start + count).collect()
            };
            if let Some(p) = &path {
                hunks.push(Hunk {
                    path: p.clone(),
                    lines,
                    insertion: count == 0,
                    header: header.clone(),
                    old_start: start,
                    old_count: count,
                    new_count,
                    changes: String::new(),
                });
            }
        } else if !in_header && (line.starts_with('+') || line.starts_with('-') || line.starts_with('\\'))
            && let Some(last) = hunks.last_mut()
        {
            last.changes.push_str(line);
            last.changes.push('\n');
        }
    }
    Ok(hunks)
}

/// `start,count` or `start` (count 1) from an `@@` line.
fn range(r: &str) -> (u32, u32) {
    match r.split_once(',') {
        Some((s, c)) => (s.parse().unwrap_or(0), c.parse().unwrap_or(0)),
        None => (r.parse().unwrap_or(0), 1),
    }
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
    fn keeps_each_hunks_header_and_ranges_for_rebuilding_a_patch() {
        let diff = "diff --git a/x b/x\nindex 1..2 100644\n--- a/x\n+++ b/x\n@@ -1 +1 @@ ctx\n-a\n+b\n@@ -5,0 +6 @@\n+c\n\\ No newline at end of file\n";
        let hunks = parse_hunks(diff).unwrap();
        assert_eq!(hunks[0].header, "diff --git a/x b/x\nindex 1..2 100644\n--- a/x\n+++ b/x\n");
        assert_eq!((hunks[0].old_start, hunks[0].old_count, hunks[0].new_count), (1, 1, 1));
        assert_eq!(hunks[0].changes, "-a\n+b\n");
        assert_eq!((hunks[1].old_start, hunks[1].old_count, hunks[1].new_count), (5, 0, 1));
        assert_eq!(hunks[1].changes, "+c\n\\ No newline at end of file\n");
    }

    #[test]
    fn new_files_need_an_explicit_target() {
        let diff = "--- /dev/null\n+++ b/new.rs\n@@ -0,0 +1 @@\n+x\n";
        assert!(parse_hunks(diff).unwrap_err().contains("new.rs is a new file"));
    }
}
