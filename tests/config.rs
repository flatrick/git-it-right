mod common;

use common::{Repo, stderr};
use gir::config::Config;
use std::process::Output;

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn load(repo: &Repo, contents: &str) -> Config {
    repo.write(".girconfig", contents);
    Config::load_from(&repo.dir.join(".girconfig")).unwrap()
}

#[test]
fn config_comes_from_repo_root_and_only_gir_keys() {
    let repo = Repo::new();
    repo.write(".girconfig", "[other]\ntypes = ignored\n[gir]\ntypes = custom\n");
    repo.write("nested/.girconfig", "[gir]\ntypes = wrong\n");
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(repo.dir.join("nested")).args(["explain", "types"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(stdout(&out).contains("  custom"), "{}", stdout(&out));
    assert!(!stdout(&out).contains("  wrong"), "{}", stdout(&out));
    assert!(!stdout(&out).contains("  ignored"), "{}", stdout(&out));
}

#[test]
fn missing_config_and_non_repo_use_defaults() {
    let repo = Repo::new();
    let in_repo = repo.gir(&["explain", "types"]);
    let outside = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(repo.dir.parent().unwrap()).args(["explain", "types"]).output().unwrap();
    assert_eq!(in_repo.status.code(), Some(0), "{}", stderr(&in_repo));
    assert_eq!(outside.status.code(), Some(0), "{}", stderr(&outside));
    assert_eq!(stdout(&in_repo), stdout(&outside), "outside a repository must use the same defaults");
    assert!(stdout(&outside).contains("  feat"), "{}", stdout(&outside));
}

#[test]
fn config_key_names_are_case_insensitive() {
    let repo = Repo::new();
    let cfg = load(&repo, "[gir]\nSuBjEcTmAx = 19\nScOpErEqUiReD = yes\n");
    assert_eq!(cfg.subject_max, 19);
    assert!(cfg.scope_required);
}

#[test]
fn types_default_and_custom_lists_accept_spaces_and_commas() {
    let repo = Repo::new();
    let defaults = Config::load_from(&repo.dir.join(".girconfig")).unwrap();
    assert_eq!(defaults.types, ["feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert"]);
    let custom = load(&repo, "[gir]\ntypes = alpha, beta gamma\n");
    assert_eq!(custom.types, ["alpha", "beta", "gamma"]);
}

#[test]
fn subject_max_defaults_to_72_and_counts_characters() {
    let repo = Repo::new();
    let defaults = Config::load_from(&repo.dir.join(".girconfig")).unwrap();
    assert_eq!(defaults.subject_max, 72);
    repo.write(".girconfig", "[gir]\nsubjectMax = 9\n");
    repo.write("MSG", "feat: ééé\n");
    let allowed = repo.gir(&["lint", "MSG"]);
    assert_eq!(allowed.status.code(), Some(0), "{}", stderr(&allowed));
    repo.write("MSG", "feat: éééé\n");
    let long = repo.gir(&["lint", "MSG"]);
    assert_eq!(long.status.code(), Some(1), "{}", stderr(&long));
    assert!(stderr(&long).contains("[header-length] header is 10 chars (max 9)"), "{}", stderr(&long));
}

#[test]
fn merge_and_revert_are_allowed_by_default_and_can_be_disabled() {
    let repo = Repo::new();
    for (name, message, rule) in [("merge", "Merge branch 'topic'", "merge-commit"), ("revert", "Revert \"feat: x\"", "revert-commit")] {
        repo.write("MSG", message);
        let default = repo.gir(&["lint", "MSG"]);
        assert_eq!(default.status.code(), Some(0), "{name}: {}", stderr(&default));
        repo.write(".girconfig", &format!("[gir]\nallow{name} = false\n"));
        let disabled = repo.gir(&["lint", "MSG"]);
        assert_eq!(disabled.status.code(), Some(1), "{name}: {}", stderr(&disabled));
        assert!(stderr(&disabled).contains(&format!("[{rule}]")), "{name}: {}", stderr(&disabled));
        std::fs::remove_file(repo.dir.join(".girconfig")).unwrap();
    }
}

#[test]
fn boolean_keys_accept_all_documented_spellings() {
    let repo = Repo::new();
    for (value, expected) in [("true", true), ("yes", true), ("on", true), ("1", true), ("false", false), ("no", false), ("off", false), ("0", false)] {
        let cfg = load(&repo, &format!("[gir]\nscopeRequired = {value}\nallowMerge = {value}\nallowRevert = {value}\n"));
        assert_eq!(cfg.scope_required, expected, "scopeRequired={value}");
        assert_eq!(cfg.allow_merge, expected, "allowMerge={value}");
        assert_eq!(cfg.allow_revert, expected, "allowRevert={value}");
    }
}

#[test]
fn default_aliases_and_configured_override_map_to_types() {
    let repo = Repo::new();
    let defaults = Config::load_from(&repo.dir.join(".girconfig")).unwrap();
    for (alias, target) in [("feature", "feat"), ("bugfix", "fix"), ("hotfix", "fix"), ("doc", "docs"), ("tests", "test"), ("refactoring", "refactor"), ("chores", "chore")] {
        assert_eq!(defaults.alias_for(alias), Some(target), "default alias {alias}");
    }
    let cfg = load(&repo, "[gir]\ntypes = custom fix\n[gir \"alias\"]\nfeature = custom\nshortcut = fix\n");
    assert_eq!(cfg.alias_for("feature"), Some("custom"));
    assert_eq!(cfg.alias_for("shortcut"), Some("fix"));
    assert_eq!(cfg.alias_for("bugfix"), Some("fix"));
}

#[test]
fn hook_missing_is_accepted_without_changing_config_behavior() {
    let repo = Repo::new();
    let before = repo.gir(&["explain", "types"]);
    repo.write(".girconfig", "[gir]\nhookMissing = fail\n");
    let after = repo.gir(&["explain", "types"]);
    assert_eq!(after.status.code(), Some(0), "{}", stderr(&after));
    assert_eq!(stdout(&before), stdout(&after), "hookMissing must not change regular config behavior");
}

#[test]
fn invalid_values_report_key_value_expected_form_and_exit_two() {
    let repo = Repo::new();
    for (key, value, expected) in [("scopeRequired", "maybe", "true/false"), ("allowMerge", "maybe", "true/false"), ("allowRevert", "maybe", "true/false"), ("subjectMax", "many", "a number"), ("descCase", "upper", "`any` or `lower`")] {
        repo.write(".girconfig", &format!("[gir]\n{key} = {value}\n"));
        let out = repo.gir(&["lint", "MSG"]);
        assert_eq!(out.status.code(), Some(2), "{key}: {}", stderr(&out));
        let err = stderr(&out);
        assert!(err.contains(".girconfig: "), "{key}: {err}");
        assert!(err.contains(&key.to_ascii_lowercase()), "{key}: {err}");
        assert!(err.contains(value), "{key}: {err}");
        assert!(err.contains(expected), "{key}: {err}");
    }
}

#[test]
fn unknown_gir_key_reports_error_while_other_sections_are_ignored() {
    let repo = Repo::new();
    repo.write(".girconfig", "[other]\nunknown = value\n[gir]\nunknown = value\n");
    let out = repo.gir(&["lint", "MSG"]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(stderr(&out).contains(".girconfig: unknown key `gir.unknown`"), "{}", stderr(&out));
}

#[test]
fn explain_uses_defaults_when_config_is_invalid() {
    let repo = Repo::new();
    let defaults = repo.gir(&["explain", "types"]);
    repo.write(".girconfig", "[gir]\nsubjectMax = many\n");
    let invalid = repo.gir(&["explain", "types"]);
    assert_eq!(invalid.status.code(), Some(0), "{}", stderr(&invalid));
    assert_eq!(stdout(&invalid), stdout(&defaults), "invalid config must not change explain types");
    assert!(!stderr(&invalid).contains(".girconfig:"), "{}", stderr(&invalid));
}

#[test]
fn malformed_config_fails_and_config_without_gir_keys_uses_defaults() {
    let repo = Repo::new();
    repo.write("MSG", "feat: add x\n");
    repo.write(".girconfig", "[gir\n\ttypes = x\n");
    let malformed = repo.gir(&["lint", "MSG"]);
    assert_eq!(malformed.status.code(), Some(2), "{}", stderr(&malformed));
    assert!(stderr(&malformed).starts_with("gir: .girconfig: "), "{}", stderr(&malformed));
    repo.write(".girconfig", "[other]\n\tkey = value\n");
    let unrelated = repo.gir(&["lint", "MSG"]);
    assert_eq!(unrelated.status.code(), Some(0), "{}", stderr(&unrelated));
}

const TYPES_JSON: &str = r#"[
  { "Name": "New Feature", "Type": "feat", "Description": "Adding a new feature", "PrefillShortDesc": "" },
  { "Name": "Work In Progress", "Type": "wip", "Description": "Still being developed", "Extra": 1 }
]"#;

fn lint_type(repo: &Repo, ty: &str) -> Output {
    repo.write("MSG", &format!("{ty}: add x\n"));
    repo.gir(&["lint", "MSG"])
}

#[test]
fn types_file_replaces_types_keeps_names_and_drops_aliases_to_left_out_types() {
    let repo = Repo::new();
    repo.write("types.json", TYPES_JSON);
    let cfg = load(&repo, "[gir]\ntypesFile = types.json\n");
    assert_eq!(cfg.types, ["feat", "wip"]);
    let file = cfg.types_file.unwrap();
    assert_eq!(file.path, repo.dir.join("types.json"));
    let names: Vec<&str> = file.defs.iter().map(|d| d.name.as_str()).collect();
    assert_eq!(names, ["New Feature", "Work In Progress"]);

    for ty in ["feat", "wip"] {
        let out = lint_type(&repo, ty);
        assert_eq!(out.status.code(), Some(0), "{ty}: {}", stderr(&out));
    }
    for ty in ["fix", "hotfix"] {
        let out = lint_type(&repo, ty);
        assert_eq!(out.status.code(), Some(1), "{ty}: {}", stderr(&out));
        assert!(stderr(&out).contains(&format!("`{ty}` is not an allowed type")), "{ty}: {}", stderr(&out));
    }
    let out = lint_type(&repo, "feature");
    assert_eq!(out.status.code(), Some(0), "an alias to a type the file keeps still applies: {}", stderr(&out));

    let explain = repo.gir(&["explain", "types"]);
    let text = stdout(&explain);
    assert!(text.contains("Allowed types (from "), "{text}");
    assert!(text.contains("  wip       Still being developed"), "{text}");
    assert!(text.contains("feature->feat"), "{text}");
    assert!(!text.contains("bugfix->fix"), "{text}");
}

#[test]
fn types_file_from_git_config_resolves_against_its_file_and_girconfig_overrides_it() {
    let repo = Repo::new();
    let home = repo.global.parent().unwrap();
    std::fs::write(home.join("global-types.json"), r#"[{ "Name": "G", "Type": "global", "Description": "from global" }]"#).unwrap();
    repo.git(&["config", "--global", "gir.typesFile", "global-types.json"]);
    assert_eq!(lint_type(&repo, "global").status.code(), Some(0), "global value resolves next to the global config");
    assert_eq!(lint_type(&repo, "fix").status.code(), Some(1), "global types file replaces the defaults");

    std::fs::write(repo.dir.join(".git").join("local-types.json"), r#"[{ "Name": "L", "Type": "local", "Description": "from .git/config" }]"#).unwrap();
    repo.git(&["config", "--local", "gir.typesFile", "local-types.json"]);
    repo.write("nested/MSG", "local: add x\n");
    let from_subdir = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(repo.dir.join("nested")).args(["lint", "MSG"]).output().unwrap();
    assert_eq!(from_subdir.status.code(), Some(0), "repository value resolves next to .git/config: {}", stderr(&from_subdir));

    repo.write("girconfig-types.json", r#"[{ "Name": "R", "Type": "repo", "Description": "from .girconfig" }]"#);
    repo.write(".girconfig", "[gir]\ntypesFile = girconfig-types.json\n");
    assert_eq!(lint_type(&repo, "repo").status.code(), Some(0), ".girconfig wins over git config");
    assert_eq!(lint_type(&repo, "local").status.code(), Some(1), ".girconfig wins over git config");

    repo.write(".girconfig", "[gir]\ntypesFile =\n");
    assert_eq!(lint_type(&repo, "fix").status.code(), Some(0), "an empty .girconfig value turns the types file off");
    assert_eq!(lint_type(&repo, "local").status.code(), Some(1), "an empty .girconfig value turns the types file off");
}

#[test]
fn types_file_from_command_line_config_resolves_against_repo_root() {
    let repo = Repo::new();
    repo.write("cli-types.json", r#"[{ "Name": "C", "Type": "cli", "Description": "from -c" }]"#);
    repo.write("nested/MSG", "cli: add x\n");
    let envs = [("GIT_CONFIG_COUNT", "1"), ("GIT_CONFIG_KEY_0", "gir.typesFile"), ("GIT_CONFIG_VALUE_0", "cli-types.json")];
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(repo.dir.join("nested")).envs(envs).args(["lint", "MSG"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
}

#[test]
fn invalid_types_file_refuses_naming_file_origin_and_reason() {
    let repo = Repo::new();
    repo.write(".girconfig", "[gir]\ntypesFile = types.json\n");
    let missing = lint_type(&repo, "feat");
    assert_eq!(missing.status.code(), Some(2), "{}", stderr(&missing));
    assert!(stderr(&missing).contains("types.json (gir.typesFile in .girconfig): "), "{}", stderr(&missing));

    let entry = |fields: &str| format!("[{{ {fields} }}]");
    let cases = [
        ("[\n  { \"Type\": \"feat\", }\n]".to_string(), "not valid JSON: trailing comma at line 2"),
        ("{}".to_string(), "expected a JSON array of types"),
        ("[]".to_string(), "defines no types"),
        ("[1]".to_string(), "entry 1: expected an object"),
        (entry(r#""Name": "N", "Description": "D""#), "entry 1: `Type` is missing"),
        (entry(r#""Type": 3, "Name": "N", "Description": "D""#), "entry 1: `Type` must be a string"),
        (entry(r#""Type": "fe at", "Name": "N", "Description": "D""#), "entry 1: `Type` is `fe at`, expected ASCII letters, digits and `-`"),
        (entry(r#""Type": "", "Name": "N", "Description": "D""#), "entry 1: `Type` is ``"),
        (entry(r#""Type": "feat", "Description": "D""#), "entry 1 (`feat`): `Name` is missing"),
        (entry(r#""Type": "feat", "Name": "N", "Description": null"#), "entry 1 (`feat`): `Description` must be a string"),
        (r#"[{ "Type": "feat", "Name": "N", "Description": "D" }, { "Type": "feat", "Name": "N", "Description": "D" }]"#.to_string(), "entry 2 (`feat`): `Type` is already defined"),
    ];
    for (json, reason) in cases {
        repo.write("types.json", &json);
        for args in [&["lint", "MSG"][..], &["hook", "commit-msg", "MSG"]] {
            let out = repo.gir(args);
            assert_eq!(out.status.code(), Some(2), "{args:?} {json}: {}", stderr(&out));
            let err = stderr(&out);
            assert!(err.starts_with("gir: types file "), "{json}: {err}");
            assert!(err.contains("types.json (gir.typesFile in .girconfig): "), "{json}: {err}");
            assert!(err.contains(reason), "{json}: {err}");
        }
        let explain = repo.gir(&["explain", "types"]);
        assert_eq!(explain.status.code(), Some(0), "{json}: {}", stderr(&explain));
        assert!(stdout(&explain).contains("Allowed types (gir.types in .girconfig):"), "{json}: {}", stdout(&explain));
        let doctor = repo.gir(&["doctor"]);
        assert!(stdout(&doctor).contains(reason), "{json}: {}", stdout(&doctor));
    }
}

#[test]
fn types_file_overrides_girconfig_types_with_a_warning() {
    let repo = Repo::new();
    repo.write("types.json", TYPES_JSON);
    repo.write(".girconfig", "[gir]\ntypes = alpha beta\ntypesFile = types.json\n");
    let out = lint_type(&repo, "wip");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(stderr(&out).contains("gir: warning: gir.types in .girconfig is ignored; types come from "), "{}", stderr(&out));
    assert_eq!(lint_type(&repo, "alpha").status.code(), Some(1));
}

#[test]
fn init_comments_out_types_when_git_config_names_a_types_file() {
    let repo = Repo::new();
    let home = repo.global.parent().unwrap();
    std::fs::write(home.join("types.json"), TYPES_JSON).unwrap();
    repo.git(&["config", "--global", "gir.typesFile", "types.json"]);
    let out = repo.gir(&["init"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let girconfig = std::fs::read_to_string(repo.dir.join(".girconfig")).unwrap();
    assert!(girconfig.contains("\t# types = feat fix"), "{girconfig}");
    assert!(!stderr(&out).contains("warning"), "{}", stderr(&out));
    let cliff = std::fs::read_to_string(repo.dir.join("cliff.toml")).unwrap();
    assert!(cliff.contains("^wip"), "{cliff}");
    assert!(!cliff.contains("^fix"), "{cliff}");
}
