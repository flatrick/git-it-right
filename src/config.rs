use std::path::Path;

use crate::git;

pub const FILE: &str = ".girconfig";

pub const DEFAULT_TYPES: &[(&str, &str)] = &[
    ("feat", "Features"),
    ("fix", "Bug Fixes"),
    ("docs", "Documentation"),
    ("style", "Styling"),
    ("refactor", "Refactor"),
    ("perf", "Performance"),
    ("test", "Testing"),
    ("build", "Build"),
    ("ci", "CI"),
    ("chore", "Miscellaneous"),
    ("revert", "Reverts"),
];

const DEFAULT_ALIASES: &[(&str, &str)] = &[
    ("feature", "feat"),
    ("bugfix", "fix"),
    ("hotfix", "fix"),
    ("doc", "docs"),
    ("tests", "test"),
    ("refactoring", "refactor"),
    ("chores", "chore"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescCase {
    Any,
    Lower,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub types: Vec<String>,
    pub scopes: Vec<String>,
    pub scope_required: bool,
    pub subject_max: usize,
    pub aliases: Vec<(String, String)>,
    pub allow_merge: bool,
    pub allow_revert: bool,
    pub desc_case: DescCase,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            types: DEFAULT_TYPES.iter().map(|(t, _)| t.to_string()).collect(),
            scopes: Vec::new(),
            scope_required: false,
            subject_max: 72,
            aliases: DEFAULT_ALIASES.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect(),
            allow_merge: true,
            allow_revert: true,
            desc_case: DescCase::Any,
        }
    }
}

impl Config {
    /// Loads `.girconfig` from the repository root; defaults outside a repo or when absent.
    pub fn load() -> Result<Config, String> {
        match git::toplevel() {
            Some(root) => Config::load_from(&root.join(FILE)),
            None => Ok(Config::default()),
        }
    }

    pub fn load_from(path: &Path) -> Result<Config, String> {
        let mut cfg = Config::default();
        if !path.is_file() {
            return Ok(cfg);
        }
        let path_str = path.to_string_lossy();
        let out = git::run(&["config", "--file", &path_str, "--get-regexp", r"^gir\."])
            .unwrap_or_default();
        for line in out.lines() {
            let (key, value) = line.split_once(' ').unwrap_or((line, ""));
            if let Some(from) = key.strip_prefix("gir.alias.") {
                cfg.aliases.retain(|(a, _)| a != from);
                cfg.aliases.push((from.to_string(), value.to_string()));
                continue;
            }
            match key {
                "gir.types" => cfg.types = list(value),
                "gir.scopes" => cfg.scopes = list(value),
                "gir.scoperequired" => cfg.scope_required = boolean(key, value)?,
                "gir.subjectmax" => {
                    cfg.subject_max = value.parse().map_err(|_| bad(key, value, "a number"))?
                }
                "gir.allowmerge" => cfg.allow_merge = boolean(key, value)?,
                "gir.allowrevert" => cfg.allow_revert = boolean(key, value)?,
                "gir.desccase" => {
                    cfg.desc_case = match value {
                        "any" => DescCase::Any,
                        "lower" => DescCase::Lower,
                        _ => return Err(bad(key, value, "`any` or `lower`")),
                    }
                }
                "gir.hookmissing" => {}
                _ => return Err(format!("{FILE}: unknown key `{key}` (see: gir explain config)")),
            }
        }
        Ok(cfg)
    }

    pub fn alias_for(&self, ty: &str) -> Option<&str> {
        self.aliases.iter().find(|(a, _)| a == ty).map(|(_, b)| b.as_str())
    }
}

pub fn group_title(ty: &str) -> String {
    DEFAULT_TYPES.iter().find(|(t, _)| *t == ty).map(|(_, g)| g.to_string()).unwrap_or_else(|| {
        let mut c = ty.chars();
        c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
    })
}

fn list(value: &str) -> Vec<String> {
    value.split([',', ' ']).filter(|s| !s.is_empty()).map(str::to_string).collect()
}

fn boolean(key: &str, value: &str) -> Result<bool, String> {
    match value {
        "true" | "yes" | "on" | "1" => Ok(true),
        "false" | "no" | "off" | "0" => Ok(false),
        _ => Err(bad(key, value, "true/false")),
    }
}

fn bad(key: &str, value: &str, want: &str) -> String {
    format!("{FILE}: `{key}` is `{value}`, expected {want}")
}

pub fn default_file() -> String {
    let types: Vec<&str> = DEFAULT_TYPES.iter().map(|(t, _)| *t).collect();
    format!(
        "# git-it-right settings (git-config syntax). Details: gir explain config\n\
         [gir]\n\
         \ttypes = {}\n\
         \tsubjectMax = 72\n\
         \t# scopes = api, cli, docs\n\
         \t# scopeRequired = false\n\
         \t# descCase = lower\n\
         \t# hookMissing = fail\n\
         [gir \"alias\"]\n\
         \tfeature = feat\n\
         \tbugfix = fix\n",
        types.join(" ")
    )
}
