use std::path::{Path, PathBuf};

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

/// One entry of a SourceGit type definition file.
#[derive(Debug, Clone)]
pub struct TypeDef {
    pub ty: String,
    /// The human-friendly label a type picker shows.
    pub name: String,
    pub description: String,
}

/// The type definition file `gir.typesFile` names, and where that setting came from.
#[derive(Debug, Clone)]
pub struct TypesFile {
    pub path: PathBuf,
    pub origin: String,
    pub defs: Vec<TypeDef>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub types: Vec<String>,
    pub types_file: Option<TypesFile>,
    pub scopes: Vec<String>,
    pub scope_required: bool,
    pub subject_max: usize,
    pub aliases: Vec<(String, String)>,
    pub allow_merge: bool,
    pub allow_revert: bool,
    pub desc_case: DescCase,
    pub warnings: Vec<String>,
}

/// A `gir.typesFile` value and the directory a relative value resolves against.
struct Setting {
    value: String,
    base: PathBuf,
    origin: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            types: DEFAULT_TYPES.iter().map(|(t, _)| t.to_string()).collect(),
            types_file: None,
            scopes: Vec::new(),
            scope_required: false,
            subject_max: 72,
            aliases: DEFAULT_ALIASES.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect(),
            allow_merge: true,
            allow_revert: true,
            desc_case: DescCase::Any,
            warnings: Vec::new(),
        }
    }
}

impl Config {
    /// Loads `.girconfig` from the repository root; defaults outside a repo or when absent.
    pub fn load() -> Result<Config, String> {
        match git::toplevel() {
            Some(root) => Config::load_at(&root),
            None => Ok(Config::default()),
        }
    }

    /// `.girconfig` at `root`, with `gir.typesFile` from git config when `.girconfig` does not set it.
    pub fn load_at(root: &Path) -> Result<Config, String> {
        let (cfg, setting, types_set) = Config::read(&root.join(FILE))?;
        let setting = match setting {
            Some(s) => Some(s),
            None => git_config_setting(root)?,
        };
        cfg.with_types_file(setting, types_set)
    }

    /// `.girconfig` alone, including a types file it names; git config is not read.
    pub fn load_from(path: &Path) -> Result<Config, String> {
        let (cfg, setting, types_set) = Config::read(path)?;
        cfg.with_types_file(setting, types_set)
    }

    fn read(path: &Path) -> Result<(Config, Option<Setting>, bool), String> {
        let mut cfg = Config::default();
        let (mut setting, mut types_set) = (None, false);
        if !path.is_file() {
            return Ok((cfg, setting, types_set));
        }
        let path_str = path.to_string_lossy();
        let out = match git::run(&["config", "--file", &path_str, "--get-regexp", r"^gir\."]) {
            Ok(out) => out,
            Err(e) if e.is_empty() => String::new(),
            Err(e) => return Err(format!("{FILE}: {e}")),
        };
        for line in out.lines() {
            let (key, value) = line.split_once(' ').unwrap_or((line, ""));
            if let Some(from) = key.strip_prefix("gir.alias.") {
                cfg.aliases.retain(|(a, _)| a != from);
                cfg.aliases.push((from.to_string(), value.to_string()));
                continue;
            }
            match key {
                "gir.types" => {
                    cfg.types = list(value);
                    types_set = true;
                }
                "gir.typesfile" => {
                    let value = git::run(&["config", "--file", &path_str, "--type=path", "--get", "gir.typesFile"])
                        .map_err(|e| format!("{FILE}: {e}"))?;
                    let base = path.parent().unwrap_or(Path::new("")).to_path_buf();
                    setting = Some(Setting { value, base, origin: FILE.to_string() });
                }
                "gir.scopes" => cfg.scopes = list(value),
                "gir.scoperequired" => cfg.scope_required = boolean(key, value)?,
                "gir.subjectmax" => cfg.subject_max = value.parse().map_err(|_| bad(key, value, "a number"))?,
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
        Ok((cfg, setting, types_set))
    }

    /// Replaces the type list with the file's types; an empty value means no types file.
    fn with_types_file(mut self, setting: Option<Setting>, types_set: bool) -> Result<Config, String> {
        let Some(s) = setting.filter(|s| !s.value.is_empty()) else {
            return Ok(self);
        };
        let path = s.base.join(&s.value);
        let defs = read_types_file(&path).map_err(|e| format!("types file {} (gir.typesFile in {}): {e}", path.display(), s.origin))?;
        if types_set {
            self.warnings.push(format!(
                "gir.types in {FILE} is ignored; types come from {} (gir.typesFile in {})",
                path.display(),
                s.origin
            ));
        }
        self.types = defs.iter().map(|d| d.ty.clone()).collect();
        self.types_file = Some(TypesFile { path, origin: s.origin, defs });
        Ok(self)
    }

    /// An alias applies only while its target is an allowed type.
    pub fn alias_for(&self, ty: &str) -> Option<&str> {
        self.aliases.iter().find(|(a, b)| a == ty && self.types.contains(b)).map(|(_, b)| b.as_str())
    }
}

/// `gir.typesFile` from git config (system, global, repository, `-c`); `None` when unset.
fn git_config_setting(root: &Path) -> Result<Option<Setting>, String> {
    let args = [
        "-C".as_ref(),
        root.as_os_str(),
        "config".as_ref(),
        "--show-origin".as_ref(),
        "-z".as_ref(),
        "--type=path".as_ref(),
        "--get".as_ref(),
        "gir.typesFile".as_ref(),
    ];
    let out = match git::run_raw::<&std::ffi::OsStr>(&args) {
        Ok(out) => out,
        Err(e) if e.is_empty() => return Ok(None),
        Err(e) => return Err(format!("gir.typesFile: {e}")),
    };
    let out = String::from_utf8_lossy(&out);
    let mut fields = out.split('\0');
    let (origin, value) = (fields.next().unwrap_or(""), fields.next().unwrap_or(""));
    let (base, origin) = match origin.strip_prefix("file:") {
        Some(file) => {
            let file = root.join(file);
            (file.parent().unwrap_or(root).to_path_buf(), file.display().to_string())
        }
        None => (root.to_path_buf(), origin.trim_end_matches(':').to_string()),
    };
    Ok(Some(Setting { value: value.to_string(), base, origin }))
}

/// Parses a SourceGit type definition file: an array of objects with string `Type`, `Name` and `Description`.
fn read_types_file(path: &Path) -> Result<Vec<TypeDef>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(text.trim_start_matches('\u{feff}')).map_err(|e| format!("not valid JSON: {e}"))?;
    let entries = json.as_array().ok_or("expected a JSON array of types")?;
    if entries.is_empty() {
        return Err("defines no types".into());
    }
    let mut defs: Vec<TypeDef> = Vec::new();
    for (i, entry) in entries.iter().enumerate() {
        let mut at = format!("entry {}", i + 1);
        let obj = entry.as_object().ok_or(format!("{at}: expected an object"))?;
        let field = |at: &str, name: &str| match obj.get(name) {
            Some(serde_json::Value::String(s)) => Ok(s.clone()),
            Some(_) => Err(format!("{at}: `{name}` must be a string")),
            None => Err(format!("{at}: `{name}` is missing")),
        };
        let ty = field(&at, "Type")?;
        if ty.is_empty() || !ty.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(format!("{at}: `Type` is `{ty}`, expected ASCII letters, digits and `-`"));
        }
        at = format!("{at} (`{ty}`)");
        if defs.iter().any(|d| d.ty == ty) {
            return Err(format!("{at}: `Type` is already defined by an earlier entry"));
        }
        defs.push(TypeDef { name: field(&at, "Name")?, description: field(&at, "Description")?, ty });
    }
    Ok(defs)
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

/// Whether git config, outside `.girconfig`, names a types file.
pub fn types_file_in_git_config(root: &Path) -> bool {
    matches!(git_config_setting(root), Ok(Some(s)) if !s.value.is_empty())
}

/// With `types_file_set`, the `types` line is commented out, since a types file replaces it.
pub fn default_file(types_file_set: bool) -> String {
    let types: Vec<&str> = DEFAULT_TYPES.iter().map(|(t, _)| *t).collect();
    format!(
        "# git-it-right settings (git-config syntax). Details: gir explain config\n\
         [gir]\n\
         \t{}types = {}\n\
         \tsubjectMax = 72\n\
         \t# scopes = api, cli, docs\n\
         \t# scopeRequired = false\n\
         \t# descCase = lower\n\
         \t# hookMissing = fail\n\
         [gir \"alias\"]\n\
         \tfeature = feat\n\
         \tbugfix = fix\n",
        if types_file_set { "# " } else { "" },
        types.join(" ")
    )
}
