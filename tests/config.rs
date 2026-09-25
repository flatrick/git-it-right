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
