use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ffi::OsStr;

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
        None => match trace(base.as_deref(), opts.split)? {
            trace if opts.split => return split::split(mode, trace, base.as_deref(), opts.dry_run, ask),
            Trace { untraced, .. } if !untraced.is_empty() => {
                let reason = &untraced[0].reason;
                if !ask {
                    return Err(format!("{reason}; {}", mode.pass_one()));
                }
                match pick_branch_commit(&format!("{reason}; pick the commit it belongs to:"), base.as_deref())? {
                    Some(sha) => sha,
                    None => return Ok(cancelled()),
                }
            }
            Trace { hunks, .. } => {
                let targets = targets(&hunks);
                let can_split = split::can_split(&hunks);
                if targets.len() == 1 {
                    targets.into_keys().next().unwrap()
                } else if !ask {
                    return Err(several_targets(mode, &targets, can_split));
                } else {
                    let items: Vec<String> = targets
                        .iter()
                        .map(|(sha, places)| format!("{} {}  <- {}", &sha[..10], subject(sha), places.join(" ")))
                        .collect();
                    eprintln!("gir: staged changes belong to several commits:");
                    let split = format!("one {} per commit", mode.prefix());
                    match pick::choose(&items, can_split.then_some(split.as_str()), std::io::stdin().lock(), std::io::stderr()) {
                        Choice::Item(i) => {
                            let sha = targets.into_keys().nth(i).unwrap();
                            eprintln!("gir: all staged changes go into one {} for {}", mode.prefix(), &sha[..10]);
                            sha
                        }
                        Choice::Split => return split::split(mode, Trace { hunks, untraced: Vec::new() }, base.as_deref(), false, true),
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

fn several_targets(mode: Mode, targets: &BTreeMap<String, Vec<String>>, can_split: bool) -> String {
    let mut msg = String::from("staged changes belong to several commits:");
    for (sha, places) in targets {
        msg.push_str(&format!("\n  {} {}  <- {}", &sha[..10], subject(sha), places.join(" ")));
    }
    msg.push_str(&format!("\n  split: git restore --staged . && git add -p, then one gir {} per commit", mode.name()));
    if can_split {
        msg.push_str(&format!("\n  or: gir {} --split creates one {} per commit", mode.name(), mode.prefix()));
    }
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
    path: Vec<u8>,
    lines: Vec<u32>,
    insertion: bool,
    /// The file's `diff --git` header lines, for rebuilding a patch.
    header: Vec<u8>,
    /// The `@@` ranges as git writes them for `-U0`: a zero count's start is the line before.
    old_start: u32,
    old_count: u32,
    new_count: u32,
    /// The hunk's `-`, `+` and `\` lines, byte for byte.
    changes: Vec<u8>,
}

impl Hunk {
    fn place(&self) -> String {
        format!("{}:{}", String::from_utf8_lossy(&self.path), self.lines.first().copied().unwrap_or(1))
    }
}

/// A staged hunk and the eligible commits that last changed its lines.
#[derive(Debug)]
struct Traced {
    hunk: Hunk,
    shas: BTreeSet<String>,
}

/// A staged file that no commit can be named for; the reason reads as a clause.
struct Untraced {
    path: Vec<u8>,
    reason: String,
}

/// The traced hunks, and the staged files that could not be traced: new files first, then
/// files without lines to trace, then files with a hunk `git blame` cannot name.
struct Trace {
    hunks: Vec<Traced>,
    untraced: Vec<Untraced>,
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

/// With `all`, every staged file is traced or listed as untraced; without it, tracing stops
/// at the first file it cannot trace, as only that one is reported.
fn trace(base: Option<&str>, all: bool) -> Result<Trace, String> {
    let diff = git::run_raw(&[
        "-c", "core.quotePath=false", "diff", "--cached", "-U0", "--no-color", "--no-ext-diff", "--no-textconv",
        "--no-renames", "--src-prefix=a/", "--dst-prefix=b/",
    ])?;
    let hunks = parse_hunks(&diff);
    let traced: BTreeSet<&[u8]> = hunks.iter().map(|h| h.path.as_slice()).collect();
    let staged = git::run_raw(&["diff", "--cached", "--name-status", "-z", "--no-renames"])?;
    let fields: Vec<&[u8]> = staged.split(|&b| b == 0).collect();
    let entries = fields.chunks(2).filter_map(|c| match c {
        [status, path] => Some((*status, *path)),
        _ => None,
    });
    let (mut untraced, others): (Vec<Untraced>, Vec<Untraced>) = entries
        .filter(|(_, p)| !traced.contains(p))
        .map(|(status, path)| {
            let name = String::from_utf8_lossy(path);
            let reason = if status == b"A" {
                format!("{name} is a new file, so it has no earlier commit")
            } else {
                format!("cannot tell which commit {name} belongs to")
            };
            (status == b"A", Untraced { path: path.to_vec(), reason })
        })
        .fold((Vec::new(), Vec::new()), |(mut new, mut other), (is_new, u)| {
            if is_new { new.push(u) } else { other.push(u) }
            (new, other)
        });
    untraced.extend(others);
    if !all && !untraced.is_empty() {
        return Ok(Trace { hunks: Vec::new(), untraced });
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
            untraced.push(Untraced { path: hunk.path.clone(), reason: format!("cannot tell which commit {place} belongs to") });
            if !all {
                return Ok(Trace { hunks: Vec::new(), untraced });
            }
            continue;
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
    let unblamed: BTreeSet<Vec<u8>> = untraced.iter().map(|u| u.path.clone()).collect();
    out.retain(|t| !unblamed.contains(&t.hunk.path));
    Ok(Trace { hunks: out, untraced })
}

/// Lines in the HEAD version each hunk touches. A pure insertion has no old
/// lines, so the lines around it stand in for it. A new file has no such lines and no hunks.
fn parse_hunks(diff: &[u8]) -> Vec<Hunk> {
    let mut hunks: Vec<Hunk> = Vec::new();
    let mut path: Option<Vec<u8>> = None;
    let mut header = Vec::new();
    let mut in_header = true;
    for raw in diff.split_inclusive(|&b| b == b'\n') {
        let line = raw.strip_suffix(b"\n").unwrap_or(raw);
        if line.starts_with(b"diff --git ") {
            header.clear();
            in_header = true;
        }
        if in_header && !line.starts_with(b"@@ ") {
            header.extend_from_slice(line);
            header.push(b'\n');
        }
        if in_header && let Some(p) = line.strip_prefix(b"--- ") {
            path = header_path(p, b"a/");
        } else if let Some(h) = line.strip_prefix(b"@@ -") {
            in_header = false;
            let h = String::from_utf8_lossy(h);
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
                    changes: Vec::new(),
                });
            }
        } else if !in_header
            && matches!(line.first(), Some(b'+' | b'-' | b'\\'))
            && let Some(last) = hunks.last_mut()
        {
            last.changes.extend_from_slice(line);
            last.changes.push(b'\n');
        }
    }
    hunks
}

/// The path in a `---`/`+++` header line without its `a/` or `b/` prefix, or `None` for
/// `/dev/null`. git ends the line with a tab when the name has a space, and C-quotes names
/// with special characters.
fn header_path(text: &[u8], prefix: &[u8]) -> Option<Vec<u8>> {
    let text = text.strip_suffix(b"\t").unwrap_or(text);
    if text == b"/dev/null" {
        return None;
    }
    let name = match text.strip_prefix(b"\"").and_then(|t| t.strip_suffix(b"\"")) {
        Some(quoted) => unquote(quoted),
        None => text.to_vec(),
    };
    Some(name.strip_prefix(prefix).map_or_else(|| name.clone(), <[u8]>::to_vec))
}

/// Decodes git's C-style quoting: single-letter escapes and `\ooo` octal bytes.
fn unquote(quoted: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut input = quoted.iter().copied().peekable();
    while let Some(b) = input.next() {
        if b != b'\\' {
            bytes.push(b);
            continue;
        }
        match input.next() {
            Some(b'a') => bytes.push(7),
            Some(b'b') => bytes.push(8),
            Some(b't') => bytes.push(b'\t'),
            Some(b'n') => bytes.push(b'\n'),
            Some(b'v') => bytes.push(11),
            Some(b'f') => bytes.push(12),
            Some(b'r') => bytes.push(b'\r'),
            Some(d @ b'0'..=b'7') => {
                let mut value = u16::from(d - b'0');
                for _ in 0..2 {
                    if let Some(&n @ b'0'..=b'7') = input.peek() {
                        value = value * 8 + u16::from(n - b'0');
                        input.next();
                    }
                }
                bytes.push(u8::try_from(value).unwrap_or(u8::MAX));
            }
            Some(other) => bytes.push(other),
            None => bytes.push(b'\\'),
        }
    }
    bytes
}

/// `start,count` or `start` (count 1) from an `@@` line.
fn range(r: &str) -> (u32, u32) {
    match r.split_once(',') {
        Some((s, c)) => (s.parse().unwrap_or(0), c.parse().unwrap_or(0)),
        None => (r.parse().unwrap_or(0), 1),
    }
}

fn blame(path: &[u8], lines: &[u32]) -> BTreeSet<String> {
    let path = git::os_path(path);
    let mut shas = BTreeSet::new();
    for line in lines {
        let range = format!("{line},{line}");
        let args = ["blame", "-l", "-s", "-L", &range, "HEAD", "--"].map(OsStr::new);
        if let Ok(out) = git::run_raw(&[&args[..], &[path.as_os_str()]].concat())
            && let Some(sha) = out.split(u8::is_ascii_whitespace).next().filter(|s| !s.is_empty())
        {
            shas.insert(String::from_utf8_lossy(sha).trim_start_matches('^').to_string());
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
        let hunks = parse_hunks(diff.as_bytes());
        assert_eq!(hunks.iter().map(|h| h.lines.clone()).collect::<Vec<_>>(), vec![vec![3, 4], vec![10, 11], vec![20]]);
        assert!(hunks.iter().all(|h| h.path == b"src/x.rs"));
    }

    #[test]
    fn keeps_each_hunks_header_and_ranges_for_rebuilding_a_patch() {
        let diff = "diff --git a/x b/x\nindex 1..2 100644\n--- a/x\n+++ b/x\n@@ -1 +1 @@ ctx\n-a\n+b\n@@ -5,0 +6 @@\n+c\n\\ No newline at end of file\n";
        let hunks = parse_hunks(diff.as_bytes());
        assert_eq!(hunks[0].header, b"diff --git a/x b/x\nindex 1..2 100644\n--- a/x\n+++ b/x\n");
        assert_eq!((hunks[0].old_start, hunks[0].old_count, hunks[0].new_count), (1, 1, 1));
        assert_eq!(hunks[0].changes, b"-a\n+b\n");
        assert_eq!((hunks[1].old_start, hunks[1].old_count, hunks[1].new_count), (5, 0, 1));
        assert_eq!(hunks[1].changes, b"+c\n\\ No newline at end of file\n");
    }

    #[test]
    fn hunk_lines_that_look_like_file_headers_are_content() {
        let diff = "diff --git a/x.lua b/x.lua\n--- a/x.lua\n+++ b/x.lua\n@@ -1,2 +0,0 @@\n--- /dev/null\n--- a/other\n@@ -5 +3 @@\n-d\n+D\n@@ -7,0 +6 @@\n+++ b/foo\n";
        let hunks = parse_hunks(diff.as_bytes());
        assert_eq!(
            hunks.iter().map(|h| (h.path.as_slice(), h.lines.clone())).collect::<Vec<_>>(),
            vec![(&b"x.lua"[..], vec![1, 2]), (b"x.lua", vec![5]), (b"x.lua", vec![7, 8])]
        );
        assert_eq!(hunks[0].changes, b"--- /dev/null\n--- a/other\n");
        assert_eq!(hunks[2].changes, b"+++ b/foo\n");
    }

    #[test]
    fn header_paths_drop_the_tab_terminator_and_decode_quoting() {
        let header_path = |text: &str, prefix: &str| super::header_path(text.as_bytes(), prefix.as_bytes()).map(|p| String::from_utf8(p).unwrap());
        assert_eq!(header_path("a/src/x.rs", "a/"), Some("src/x.rs".to_string()));
        assert_eq!(header_path("a/my file.txt\t", "a/"), Some("my file.txt".to_string()));
        assert_eq!(header_path("/dev/null", "a/"), None);
        assert_eq!(header_path("\"a/say \\\"hi\\\".txt\"\t", "a/"), Some("say \"hi\".txt".to_string()));
        assert_eq!(header_path("\"b/back\\\\slash.txt\"", "b/"), Some("back\\slash.txt".to_string()));
        assert_eq!(header_path("\"a/tab\\there.txt\"", "a/"), Some("tab\there.txt".to_string()));
        assert_eq!(header_path("\"a/caf\\303\\251.txt\"", "a/"), Some("caf\u{e9}.txt".to_string()));
    }

    #[test]
    fn new_files_with_quoted_names_have_no_lines_to_trace() {
        let diff = "diff --git \"a/n\\\"q.rs\" \"b/n\\\"q.rs\"\n--- /dev/null\n+++ \"b/n\\\"q.rs\"\n@@ -0,0 +1 @@\n+x\n";
        assert!(parse_hunks(diff.as_bytes()).is_empty());
    }

    #[test]
    fn new_files_have_no_lines_to_trace() {
        let diff = "--- /dev/null\n+++ b/new.rs\n@@ -0,0 +1 @@\n+x\n";
        assert!(parse_hunks(diff.as_bytes()).is_empty());
    }
}
