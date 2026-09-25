mod common;

use common::{Repo, stderr};
use std::fs;

fn content(repo: &Repo, path: &str) -> String {
    fs::read_to_string(repo.dir.join(path)).unwrap()
}

fn index(repo: &Repo) -> String {
    repo.git(&["ls-files", "--stage", "--", ".githooks/commit-msg", ".githooks/pre-push"])
}

#[test]
fn init_requires_a_repository() {
    let repo = Repo::new();
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(repo.dir.parent().unwrap()).arg("init").output().unwrap();
    assert_eq!(out.status.code(), Some(2), "init outside a repository exits 2");
    assert!(stderr(&out).contains("gir: not inside a git repository"), "init explains the missing repository");
}

#[test]
fn init_creates_files_and_reports_each_write() {
    let repo = Repo::new();
    let out = repo.gir(&["init"]);
    assert!(out.status.success(), "fresh init succeeds");
    assert!(out.stdout.is_empty(), "successful init prints no stdout");
    assert!(stderr(&out).contains("gir: next: gir doctor"), "successful init prints the next step");
    for path in [".githooks/commit-msg", ".githooks/pre-push", ".girconfig", "cliff.toml"] {
        assert!(stderr(&out).contains(&format!("gir: wrote {path}")), "init reports writing {path}");
    }
    for path in [".githooks/commit-msg", ".githooks/pre-push"] {
        assert!(content(&repo, path).starts_with("#!/bin/sh\n"), "{path} has a shell header");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_ne!(fs::metadata(repo.dir.join(path)).unwrap().permissions().mode() & 0o111, 0, "{path} is executable");
        }
    }
    let cfg = content(&repo, ".girconfig");
    for item in ["[gir]", "types = feat fix docs style refactor perf test build ci chore revert", "subjectMax = 72", "[gir \"alias\"]", "feature = feat", "bugfix = fix", "# scopes =", "# scopeRequired =", "# descCase =", "# hookMissing ="] {
        assert!(cfg.contains(item), ".girconfig contains {item}");
    }
    assert_eq!(repo.git(&["config", "--local", "--get", "core.hooksPath"]), ".githooks", "init sets the local hook path");
    assert!(stderr(&out).contains("gir: set core.hooksPath=.githooks (this clone)"), "init reports setting the hook path");
}

#[test]
fn init_generates_cliff_format_and_default_parsers() {
    let repo = Repo::new();
    assert!(repo.gir(&["init"]).status.success());
    let cliff = content(&repo, "cliff.toml");
    for item in ["[changelog]", "[git]", "conventional_commits = true", "filter_unconventional = true", "protect_breaking_commits = true", "sort_commits = \"oldest\"", "trim = true", "## {{ version }}", "## Unreleased", "group_by(attribute=\"group\")", "commit.scope", "commit.breaking"] {
        assert!(cliff.contains(item), "cliff.toml contains {item}");
    }
    assert!(cliff.contains("^(fixup|squash|amend)!\", skip = true"), "cliff.toml skips autosquash messages");
    for (ty, group) in [("feat", "Features"), ("fix", "Bug Fixes"), ("docs", "Documentation"), ("style", "Styling"), ("refactor", "Refactor"), ("perf", "Performance"), ("test", "Testing"), ("build", "Build"), ("ci", "CI"), ("chore", "Miscellaneous"), ("revert", "Reverts")] {
        assert!(cliff.contains(&format!("message = \"^{ty}(\\\\(|!|:)\", group = \"{group}\"")), "{ty} parser uses {group}");
    }
}

#[test]
fn init_uses_kept_config_for_cliff_parsers() {
    let repo = Repo::new();
    repo.write(".girconfig", "[gir]\n types = feat custom\n");
    let out = repo.gir(&["init"]);
    assert_eq!(out.status.code(), Some(1), "kept custom config gives conflict status");
    assert_eq!(content(&repo, ".girconfig"), "[gir]\n types = feat custom\n", "custom config remains unchanged");
    let cliff = content(&repo, "cliff.toml");
    assert!(cliff.contains("message = \"^custom("), "custom type gets a parser");
    assert!(cliff.contains("group = \"Custom\""), "custom type gets a capitalized group");
    assert!(!cliff.contains("message = \"^fix("), "omitted default type gets no parser");
}

#[test]
fn init_keeps_differing_files_and_processes_the_rest() {
    let repo = Repo::new();
    repo.write(".githooks/commit-msg", "custom hook\n");
    repo.write(".girconfig", "[gir]\n types = feat\n");
    let out = repo.gir(&["init"]);
    assert_eq!(out.status.code(), Some(1), "kept differences exit 1");
    assert_eq!(content(&repo, ".githooks/commit-msg"), "custom hook\n", "init keeps a changed hook");
    assert_eq!(content(&repo, ".girconfig"), "[gir]\n types = feat\n", "init keeps a changed config");
    for path in [".githooks/commit-msg", ".girconfig"] {
        assert!(stderr(&out).contains(&format!("gir: kept {path} (differs from the template; --force overwrites)")), "init reports keeping {path}");
    }
    assert!(repo.dir.join("cliff.toml").exists(), "init continues to generate cliff.toml");
    assert_eq!(repo.git(&["config", "--local", "--get", "core.hooksPath"]), ".githooks", "init continues Git setup");
}

#[test]
fn force_replaces_differing_files() {
    let repo = Repo::new();
    assert!(repo.gir(&["init"]).status.success());
    repo.write(".girconfig", "changed\n");
    repo.write("cliff.toml", "changed\n");
    repo.write(".githooks/pre-push", "changed\n");
    let out = repo.gir(&["init", "--force"]);
    assert_eq!(out.status.code(), Some(0), "--force is accepted and succeeds");
    assert!(content(&repo, ".girconfig").contains("subjectMax = 72"), "--force restores the config template");
    assert!(content(&repo, "cliff.toml").contains("[changelog]"), "--force restores the cliff template");
    assert!(content(&repo, ".githooks/pre-push").starts_with("#!/bin/sh\n"), "--force restores the hook template");
    for path in [".girconfig", "cliff.toml", ".githooks/pre-push"] {
        assert!(stderr(&out).contains(&format!("gir: wrote {path}")), "--force reports replacing {path}");
    }
}

#[test]
fn identical_crlf_files_are_left_unchanged() {
    let repo = Repo::new();
    assert!(repo.gir(&["init"]).status.success());
    let files: Vec<_> = [".githooks/commit-msg", ".githooks/pre-push", ".girconfig", "cliff.toml"]
        .into_iter()
        .map(|path| (path, content(&repo, path).replace('\n', "\r\n")))
        .collect();
    for (path, crlf) in &files {
        repo.write(path, crlf);
    }
    let out = repo.gir(&["init"]);
    assert!(out.status.success(), "equivalent CRLF files are not conflicts");
    for (path, crlf) in files {
        assert_eq!(content(&repo, path), crlf, "init preserves equivalent CRLF bytes in {path}");
    }
    assert!(!stderr(&out).contains("gir: wrote"), "clean rerun reports no writes");
}

#[test]
fn init_stages_written_hooks_as_executable() {
    let repo = Repo::new();
    let out = repo.gir(&["init"]);
    assert!(out.status.success());
    let staged = index(&repo);
    for path in [".githooks/commit-msg", ".githooks/pre-push"] {
        assert!(staged.lines().any(|line| line.starts_with("100755 ") && line.ends_with(path)), "{path} is staged executable");
    }
    assert!(stderr(&out).contains("gir: staged .githooks/commit-msg .githooks/pre-push as executable"), "init lists both staged paths");
}

#[test]
fn init_restages_only_hooks_with_missing_or_wrong_index_modes() {
    let repo = Repo::new();
    assert!(repo.gir(&["init"]).status.success());
    repo.git(&["update-index", "--chmod=-x", ".githooks/commit-msg"]);
    repo.git(&["rm", "--cached", "-q", ".githooks/pre-push"]);
    let out = repo.gir(&["init"]);
    assert!(out.status.success());
    for path in [".githooks/commit-msg", ".githooks/pre-push"] {
        assert!(index(&repo).lines().any(|line| line.starts_with("100755 ") && line.ends_with(path)), "{path} is repaired in the index");
    }
    assert!(stderr(&out).contains("gir: staged .githooks/commit-msg .githooks/pre-push as executable"), "init reports repaired paths");
}

#[test]
fn init_reports_only_the_hook_it_stages() {
    let repo = Repo::new();
    assert!(repo.gir(&["init"]).status.success());
    repo.git(&["update-index", "--chmod=-x", ".githooks/commit-msg"]);
    let out = repo.gir(&["init"]);
    assert!(out.status.success());
    assert!(stderr(&out).contains("gir: staged .githooks/commit-msg as executable"), "init reports the repaired hook");
    assert!(!stderr(&out).contains("gir: staged .githooks/commit-msg .githooks/pre-push"), "init does not report the untouched hook");
}

#[test]
fn clean_rerun_does_not_restage_or_repeat_hook_path_message() {
    let repo = Repo::new();
    assert!(repo.gir(&["init"]).status.success());
    let before = fs::metadata(repo.dir.join(".git/index")).unwrap().modified().unwrap();
    let staged = index(&repo);
    repo.write(".git/index.lock", "locked");
    let out = repo.gir(&["init"]);
    assert!(out.status.success(), "a clean rerun succeeds with a locked index because it never stages");
    assert_eq!(index(&repo), staged, "a clean rerun preserves index entries");
    assert_eq!(fs::metadata(repo.dir.join(".git/index")).unwrap().modified().unwrap(), before, "a clean rerun leaves the index file untouched");
    assert!(!stderr(&out).contains("gir: staged"), "a clean rerun reports no staging");
    assert!(!stderr(&out).contains("gir: set core.hooksPath=.githooks (this clone)"), "a clean rerun does not report setting hooksPath");
}

#[test]
fn invalid_config_stops_before_cliff_and_git_setup() {
    for config in ["[gir]\n unknown = yes\n", "[gir]\n subjectMax = many\n"] {
        let repo = Repo::new();
        repo.write(".girconfig", config);
        let out = repo.gir(&["init"]);
        assert_eq!(out.status.code(), Some(2), "invalid config exits 2");
        assert!(stderr(&out).contains("gir: .girconfig:"), "invalid config names .girconfig");
        assert!(!repo.dir.join("cliff.toml").exists(), "invalid config prevents cliff generation");
        assert!(repo.git_out(&["config", "--local", "--get", "core.hooksPath"]).stdout.is_empty(), "invalid config prevents hook-path setup");
    }
}

#[test]
fn git_staging_failure_is_reported() {
    let repo = Repo::new();
    repo.write(".git/index.lock", "locked");
    let out = repo.gir(&["init"]);
    assert_eq!(out.status.code(), Some(2), "Git staging failure exits 2");
    assert!(stderr(&out).contains("gir: fatal:"), "Git staging failure reports Git's error");
}

#[test]
fn git_hook_path_failure_is_reported() {
    let repo = Repo::new();
    repo.write(".git/config.lock", "locked");
    let out = repo.gir(&["init"]);
    assert_eq!(out.status.code(), Some(2), "Git hook path failure exits 2");
    assert!(stderr(&out).contains("gir: error:"), "Git hook path failure reports Git's error");
}

#[test]
fn init_writes_files_at_the_repository_root_from_a_subdirectory() {
    let repo = Repo::new();
    let sub = repo.dir.join("nested");
    fs::create_dir(&sub).unwrap();
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(&sub).arg("init").output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    for path in [".githooks/commit-msg", ".githooks/pre-push", ".girconfig", "cliff.toml"] {
        assert!(repo.dir.join(path).is_file(), "{path} is at the repository root");
        assert!(!sub.join(path).exists(), "{path} is absent from the current subdirectory");
    }
}

#[test]
fn init_keeps_invalid_utf8_cliff_toml_without_force() {
    let repo = Repo::new();
    let original = b"custom = \xff\n";
    fs::write(repo.dir.join("cliff.toml"), original).unwrap();
    let out = repo.gir(&["init"]);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    assert_eq!(fs::read(repo.dir.join("cliff.toml")).unwrap(), original);
}

#[test]
fn init_kept_file_prints_next_step_last() {
    let repo = Repo::new();
    repo.write("cliff.toml", "custom = true\n");
    let out = repo.gir(&["init"]);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    assert!(out.stdout.is_empty());
    assert_eq!(stderr(&out).lines().last(), Some("gir: next: gir doctor"));
}
