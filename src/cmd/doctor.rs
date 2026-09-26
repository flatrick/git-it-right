use std::collections::BTreeMap;
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
    Chmod(Vec<String>),
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

    let files = git::run(&["ls-files", "--cached", "--others", "--exclude-standard"]).unwrap_or_default();
    let mut rules: Vec<(&str, &str)> = templates::IGNORE_RULES
        .iter()
        .filter(|(marker, _, _)| {
            *marker == "*"
                || (marker.starts_with('.') && files.lines().any(|f| f.ends_with(marker)))
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
    let tracked = git::run(&["ls-files"]).unwrap_or_default();
    let missing: Vec<&str> = rules
        .iter()
        .filter(|(_, probe)| !tracked.lines().any(|t| t == *probe))
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

pub fn windows_unsafe(path: &str) -> bool {
    path.split('/').any(|seg| {
        let stem = seg.split('.').next().unwrap_or(seg).to_ascii_uppercase();
        RESERVED.contains(&stem.as_str())
            || seg.ends_with('.')
            || seg.ends_with(' ')
            || seg.contains(['<', '>', ':', '"', '\\', '|', '?', '*'])
    })
}

fn index_checks(out: &mut Vec<Check>) -> Result<(), String> {
    let staged = git::run(&["-c", "core.quotePath=false", "ls-files", "-s"])?;
    let mut by_lower: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    let mut not_exec = Vec::new();
    let mut unsafe_names = Vec::new();
    let mut previous = None;
    for line in staged.lines() {
        let Some((meta, path)) = line.split_once('\t') else { continue };
        if previous.replace(path) == Some(path) {
            continue;
        }
        by_lower.entry(path.to_lowercase()).or_default().push(path);
        if windows_unsafe(path) {
            unsafe_names.push(path);
        }
        if meta.starts_with("100644") && (path.ends_with(".sh") || path.starts_with(".githooks/")) {
            not_exec.push(path.to_string());
        }
    }
    let collisions: Vec<String> = by_lower.values().filter(|v| v.len() > 1).map(|v| v.join(" = ")).collect();
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
        out.push(check(Level::Warn, "exec-bit", format!("scripts not executable in git: {}", not_exec.join(" ")), Some(Fix::Chmod(not_exec))));
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
            let mut args = vec!["update-index", "--chmod=+x", "--"];
            args.extend(paths.iter().map(String::as_str));
            git::run(&args).map(drop)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_reserved_names() {
        for bad in ["con", "docs/aux.txt", "COM1.log", "a/b./c", "x:y", "trailing "] {
            assert!(windows_unsafe(bad), "{bad}");
        }
        for good in ["console.rs", "auxiliary/x", "src/nul_check.rs", "a.b.c"] {
            assert!(!windows_unsafe(good), "{good}");
        }
    }
}
