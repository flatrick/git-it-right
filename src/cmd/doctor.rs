use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::templates;
use crate::config::{self, Config};
use crate::git;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Level {
    Ok,
    Info,
    Warn,
}

enum Fix {
    LocalConfig(&'static str, &'static str),
    WriteFile(&'static str, &'static str),
    AppendIgnore(Vec<&'static str>),
    Chmod(Vec<ChmodPath>),
}

struct ChmodPath {
    path: Vec<u8>,
    object: Vec<u8>,
    skip_worktree: bool,
}

struct Check {
    level: Level,
    id: String,
    msg: String,
    fix: Option<Fix>,
}

fn check(level: Level, id: impl Into<String>, msg: impl Into<String>, fix: Option<Fix>) -> Check {
    Check { level, id: id.into(), msg: msg.into(), fix }
}

/// (key, recommended value, why). Applied with `git config --local` only.
const LOCAL_CONFIG: &[(&str, &str, &str)] = &[
    ("pull.ff", "only", "pull never creates surprise merge commits"),
    ("fetch.prune", "true", "deleted remote branches disappear locally"),
    ("push.autoSetupRemote", "true", "first push of a branch sets its upstream"),
    ("rerere.enabled", "true", "git remembers conflict resolutions"),
    ("merge.conflictStyle", "zdiff3", "conflict markers show the common ancestor"),
    ("diff.algorithm", "histogram", "more readable diffs"),
    ("rebase.autoSquash", "true", "fixup!/squash! commits fold in on rebase"),
    ("rebase.autoStash", "true", "rebase works with a dirty tree"),
    ("rebase.updateRefs", "true", "stacked branches follow a rebase"),
];

pub fn doctor(fix: bool) -> Result<i32, String> {
    let root = git::enter_toplevel()?;
    let mut checks = Vec::new();
    config_checks(&root, &mut checks);
    file_checks(&root, &mut checks)?;
    index_checks(&mut checks)?;

    let mut remaining = 0;
    let mut fixable = 0;
    for c in &checks {
        if c.level == Level::Ok {
            continue;
        }
        let mut status = if c.level == Level::Warn { "warn" } else { "info" };
        if let (true, Some(f)) = (fix, &c.fix) {
            apply(&root, f)?;
            status = "fixed";
        } else if c.level == Level::Warn {
            remaining += 1;
        }
        if c.fix.is_some() && !fix {
            fixable += 1;
        }
        crate::outln!("{status:<5} {}: {}", c.id, c.msg);
    }
    let ok = checks.iter().filter(|c| c.level == Level::Ok).count();
    let fix_hint = if fixable > 0 { format!(", {fixable} fixable with: gir doctor --fix") } else { String::new() };
    crate::outln!("gir doctor: {ok} ok, {remaining} warnings{fix_hint}   more: gir explain doctor");
    Ok(i32::from(remaining > 0))
}

fn config_checks(root: &Path, out: &mut Vec<Check>) {
    if root.join(".githooks").is_dir() {
        match git::get_config("core.hooksPath") {
            Some(p) if p == ".githooks" => out.push(check(Level::Ok, "core.hooksPath", "", None)),
            _ => out.push(check(Level::Warn, "core.hooksPath", "hooks in .githooks/ are not active in this clone", Some(Fix::LocalConfig("core.hooksPath", ".githooks")))),
        }
    } else {
        out.push(check(Level::Warn, "hooks", "no .githooks/; run: gir init", None));
    }

    for (key, want, why) in LOCAL_CONFIG {
        if *key == "pull.ff" && git::get_config("pull.rebase").is_some() {
            out.push(check(Level::Ok, *key, "", None));
            continue;
        }
        match git::get_config(key) {
            Some(v) if v.eq_ignore_ascii_case(want) => out.push(check(Level::Ok, *key, "", None)),
            Some(v) => out.push(check(Level::Info, *key, format!("is `{v}`; `{want}` recommended ({why})"), None)),
            None => out.push(check(Level::Warn, *key, format!("unset; `{want}` recommended ({why})"), Some(Fix::LocalConfig(key, want)))),
        }
    }
    if cfg!(windows) && git::get_config("core.longpaths").is_none() {
        out.push(check(Level::Warn, "core.longpaths", "unset; paths over 260 chars fail on Windows", Some(Fix::LocalConfig("core.longpaths", "true"))));
    }

    for key in ["user.name", "user.email"] {
        if git::get_config(key).is_none() {
            out.push(check(Level::Warn, key, format!("unset; run: git config --global {key} <value>"), None));
        }
    }
    if git::get_config("init.defaultBranch").is_none() {
        out.push(check(Level::Info, "init.defaultBranch", "unset; suggest: git config --global init.defaultBranch main", None));
    }
}

fn file_checks(root: &Path, out: &mut Vec<Check>) -> Result<(), String> {
    match std::fs::read(root.join(".gitattributes")).map(|b| String::from_utf8_lossy(&b).into_owned()) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
            out.push(check(Level::Warn, ".gitattributes", format!("cannot read: {e}"), None));
        }
        Err(_) => {
            let crlf = git::get_config("core.autocrlf").filter(|v| v != "false");
            let extra = crlf.map(|v| format!(" and core.autocrlf={v}, so line endings depend on each clone")).unwrap_or_default();
            out.push(check(Level::Warn, ".gitattributes", format!("missing{extra}"), Some(Fix::WriteFile(".gitattributes", templates::GITATTRIBUTES))));
        }
        Ok(text) if !text.lines().any(|l| l.trim_start().starts_with("* text=auto")) => {
            out.push(check(Level::Info, ".gitattributes", "no `* text=auto` line; line endings are not normalised", None));
        }
        Ok(_) => out.push(check(Level::Ok, ".gitattributes", "", None)),
    }

    if !root.join(".editorconfig").exists() {
        out.push(check(Level::Info, ".editorconfig", "missing; editors will not agree on indentation and newlines", Some(Fix::WriteFile(".editorconfig", templates::EDITORCONFIG))));
    }

    if let Err(e) = Config::load_from(&root.join(config::FILE)) {
        out.push(check(Level::Warn, config::FILE, e, None));
    }

    let files = git::run_raw(&["ls-files", "--cached", "--others", "--exclude-standard", "-z"]).unwrap_or_default();
    let files: Vec<&[u8]> = files.split(|&b| b == 0).collect();
    let mut rules: Vec<(&str, &str)> = templates::IGNORE_RULES
        .iter()
        .filter(|(marker, _, _)| {
            *marker == "*"
                || (marker.starts_with('.') && files.iter().any(|f| f.ends_with(marker.as_bytes())))
                || root.join(marker).exists()
        })
        .map(|(_, pattern, probe)| (*pattern, *probe))
        .collect();
    if root.join(".claude").is_dir() {
        rules.push((".claude/settings.local.json", ".claude/settings.local.json"));
    }
    if root.join(".scratch").is_dir() {
        rules.push((".scratch/", ".scratch/gir-probe"));
    }
    let mut seen = std::collections::HashSet::new();
    rules.retain(|r| seen.insert(*r));
    let tracked = git::run_raw(&["ls-files", "-z"]).unwrap_or_default();
    let missing: Vec<&str> = rules
        .iter()
        .filter(|(_, probe)| !tracked.split(|&b| b == 0).any(|t| t == probe.as_bytes()))
        .filter(|(_, probe)| git::run(&["check-ignore", "-q", "--no-index", probe]).is_err())
        .map(|(pattern, _)| *pattern)
        .collect();
    if missing.is_empty() {
        out.push(check(Level::Ok, ".gitignore", "", None));
    } else {
        out.push(check(Level::Warn, ".gitignore", format!("does not ignore: {}", missing.join(" ")), Some(Fix::AppendIgnore(missing))));
    }

    if !git::on_path("git-cliff") {
        out.push(check(Level::Info, "git-cliff", "not installed (optional: changelog + next version from commits)", None));
    }
    Ok(())
}

const RESERVED: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Whether Windows cannot hold `path`: bytes that are not UTF-8, a control character, a
/// reserved device name, a trailing dot or space, or a reserved character.
pub fn windows_unsafe(path: &[u8]) -> bool {
    let Ok(path) = std::str::from_utf8(path) else { return true };
    path.bytes().any(|b| (1..=31).contains(&b)) || path.split('/').any(|seg| {
        let stem = seg.split('.').next().unwrap_or(seg).to_ascii_uppercase();
        RESERVED.contains(&stem.as_str())
            || seg.ends_with('.')
            || seg.ends_with(' ')
            || seg.contains(['<', '>', ':', '"', '\\', '|', '?', '*'])
    })
}

/// A path for a report line: as stored, or C-quoted the way git quotes it when it has a
/// control character or bytes that are not UTF-8.
fn display_path(path: &[u8]) -> String {
    let needs_quoting = |b: &u8| *b < 0x20 || *b == 0x7f;
    if let Ok(text) = std::str::from_utf8(path)
        && !path.iter().any(needs_quoting)
    {
        return text.to_string();
    }
    let mut out = String::from("\"");
    for chunk in path.utf8_chunks() {
        for c in chunk.valid().chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\u{7}' => out.push_str("\\a"),
                '\u{8}' => out.push_str("\\b"),
                '\t' => out.push_str("\\t"),
                '\n' => out.push_str("\\n"),
                '\u{b}' => out.push_str("\\v"),
                '\u{c}' => out.push_str("\\f"),
                '\r' => out.push_str("\\r"),
                c if c.is_ascii_control() => out.push_str(&format!("\\{:03o}", u32::from(c))),
                c => out.push(c),
            }
        }
        for b in chunk.invalid() {
            out.push_str(&format!("\\{b:03o}"));
        }
    }
    out.push('"');
    out
}

fn index_checks(out: &mut Vec<Check>) -> Result<(), String> {
    let staged = git::run_raw(&["ls-files", "-s", "-v", "-z"])?;
    let unmerged: BTreeSet<&[u8]> = staged
        .split(|&b| b == 0)
        .filter_map(|record| {
            let tab = record.iter().position(|&b| b == b'\t')?;
            let stage = record[..tab].split(|&b| b == b' ').nth(3)?;
            (stage != b"0").then_some(&record[tab + 1..])
        })
        .collect();
    let mut by_lower: BTreeMap<Vec<u8>, Vec<&[u8]>> = BTreeMap::new();
    let mut not_exec = Vec::new();
    let mut unsafe_names = Vec::new();
    let mut previous = None;
    for record in staged.split(|&b| b == 0) {
        let Some(tab) = record.iter().position(|&b| b == b'\t') else { continue };
        let (meta, path) = (&record[..tab], &record[tab + 1..]);
        let mut fields = meta.split(|&b| b == b' ');
        let tag = fields.next().unwrap_or_default();
        let mode = fields.next().unwrap_or_default();
        let object = fields.next().unwrap_or_default();
        if previous.replace(path) == Some(path) {
            continue;
        }
        let key = std::str::from_utf8(path).map_or_else(|_| path.to_ascii_lowercase(), |p| p.to_lowercase().into_bytes());
        by_lower.entry(key).or_default().push(path);
        if windows_unsafe(path) {
            unsafe_names.push(display_path(path));
        }
        if mode == b"100644" && !unmerged.contains(path) && (path.ends_with(b".sh") || path.starts_with(b".githooks/")) {
            not_exec.push(ChmodPath { path: path.to_vec(), object: object.to_vec(), skip_worktree: tag == b"S" });
        }
    }
    let collisions: Vec<String> = by_lower
        .values()
        .filter(|v| v.len() > 1)
        .map(|v| v.iter().map(|p| display_path(p)).collect::<Vec<_>>().join(" = "))
        .collect();
    let mut index_ok = true;
    if !collisions.is_empty() {
        index_ok = false;
        out.push(check(Level::Warn, "case-collision", format!("paths differ only in case, which breaks Windows/macOS checkouts: {}", collisions.join(", ")), None));
    }
    if !unsafe_names.is_empty() {
        index_ok = false;
        out.push(check(Level::Warn, "windows-names", format!("cannot be checked out on Windows: {}", unsafe_names.join(" ")), None));
    }
    if !not_exec.is_empty() {
        index_ok = false;
        let names: Vec<String> = not_exec.iter().map(|p| display_path(&p.path)).collect();
        out.push(check(Level::Warn, "exec-bit", format!("scripts not executable in git: {}", names.join(" ")), Some(Fix::Chmod(not_exec))));
    }
    if index_ok {
        out.push(check(Level::Ok, "index", "", None));
    }
    Ok(())
}

fn apply(root: &Path, fix: &Fix) -> Result<(), String> {
    match fix {
        Fix::LocalConfig(key, value) => git::run(&["config", "--local", key, value]).map(drop),
        Fix::WriteFile(rel, content) => std::fs::write(root.join(rel), content).map_err(|e| format!("cannot write {rel}: {e}")),
        Fix::AppendIgnore(patterns) => {
            let path = root.join(".gitignore");
            let mut text = match std::fs::read(&path) {
                Ok(bytes) => bytes,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
                Err(e) => return Err(format!("cannot read .gitignore: {e}")),
            };
            if !text.is_empty() && !text.ends_with(b"\n") {
                text.push(b'\n');
            }
            for p in patterns {
                text.extend_from_slice(p.as_bytes());
                text.push(b'\n');
            }
            std::fs::write(&path, text).map_err(|e| format!("cannot write .gitignore: {e}"))
        }
        Fix::Chmod(paths) => {
            let mut input = Vec::new();
            let mut skipped = Vec::new();
            for entry in paths {
                input.extend_from_slice(b"100755 ");
                input.extend_from_slice(&entry.object);
                input.push(b'\t');
                input.extend_from_slice(&entry.path);
                input.push(0);
                if entry.skip_worktree {
                    skipped.extend_from_slice(&entry.path);
                    skipped.push(0);
                }
            }
            git::run_with_stdin_bytes(&["update-index", "-z", "--index-info"], &input)?;
            if !skipped.is_empty() {
                git::run_with_stdin_bytes(&["update-index", "-z", "--skip-worktree", "--stdin"], &skipped)?;
            }
            make_executable_on_disk(root, paths)
        }
    }
}

/// Gives each fixed script that is a regular file in the working tree an execute bit
/// wherever it has a read bit, so the file matches its new `100755` index entry.
#[cfg(unix)]
fn make_executable_on_disk(root: &Path, paths: &[ChmodPath]) -> Result<(), String> {
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::PermissionsExt;
    for entry in paths {
        let file = root.join(std::ffi::OsStr::from_bytes(&entry.path));
        let Ok(meta) = std::fs::symlink_metadata(&file) else { continue };
        if !meta.file_type().is_file() {
            continue;
        }
        let mode = meta.permissions().mode();
        let wanted = mode | ((mode & 0o444) >> 2);
        if wanted != mode {
            std::fs::set_permissions(&file, std::fs::Permissions::from_mode(wanted))
                .map_err(|e| format!("cannot make {} executable: {e}", display_path(&entry.path)))?;
        }
    }
    Ok(())
}

/// Windows has no executable bit: git keeps the mode in the index only.
#[cfg(not(unix))]
fn make_executable_on_disk(_root: &Path, _paths: &[ChmodPath]) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_reserved_names() {
        for bad in ["con", "docs/aux.txt", "COM1.log", "a/b./c", "x:y", "trailing "] {
            assert!(windows_unsafe(bad.as_bytes()), "{bad}");
        }
        for good in ["console.rs", "auxiliary/x", "src/nul_check.rs", "a.b.c"] {
            assert!(!windows_unsafe(good.as_bytes()), "{good}");
        }
    }
}
