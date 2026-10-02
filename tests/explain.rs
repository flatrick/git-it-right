mod common;

use common::{Repo, stderr};
use std::collections::BTreeMap;
use std::path::Path;

fn stdout(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}

#[test]
fn explain_lists_topics_on_one_line() {
    let repo = Repo::new();
    let out = repo.gir(&["explain"]);
    let text = stdout(&out);
    assert_eq!(out.status.code(), Some(0), "explain should succeed: {}", stderr(&out));
    assert!(text.starts_with("topics: "), "missing topic prefix: {text}");
    assert_eq!(text.lines().count(), 1, "topic list must occupy one line: {text}");
    assert!(text.ends_with('\n'), "topic list must end with a newline: {text}");
    assert!(!text["topics: ".len()..].trim_end().contains("  "), "topic names must have single spaces: {text}");
    assert!(!text.trim_end().ends_with(' '), "topic list must not have a trailing separator: {text}");
}

#[test]
fn config_page_shows_the_init_aliases() {
    let repo = Repo::new();
    let init = repo.gir(&["init"]);
    assert_eq!(init.status.code(), Some(0), "init should succeed: {}", stderr(&init));
    let config = std::fs::read_to_string(repo.dir.join(".girconfig")).unwrap();
    let out = repo.gir(&["explain", "config"]);
    let page = stdout(&out);
    assert_eq!(out.status.code(), Some(0), "config page should succeed: {}", stderr(&out));
    for alias in ["feature = feat", "bugfix = fix"] {
        assert!(config.contains(alias), "init config lacks {alias}");
        assert!(page.contains(alias), "config page lacks {alias}");
    }
}

#[test]
fn topic_pages_contain_only_their_section_and_one_final_newline() {
    let repo = Repo::new();
    let cheatsheet = include_str!("../CHEATSHEET.md");
    let rules = include_str!("../src/explain.md");
    let topics = [cheatsheet, rules].into_iter().flat_map(|source| source.lines().filter_map(|line| line.strip_prefix("## ")));
    for topic in topics {
        let heading = format!("## {topic}\n");
        let source = [cheatsheet, rules].into_iter().find(|source| source.contains(&heading)).unwrap();
        let body = source.split_once(&heading).unwrap().1;
        let expected = format!("{}\n", body.split("\n## ").next().unwrap().trim_end());
        let out = repo.gir(&["explain", topic]);
        assert_eq!(out.status.code(), Some(0), "{topic} should succeed: {}", stderr(&out));
        assert_eq!(stdout(&out), expected, "{topic} must print only its section with one final newline");
        assert!(!stdout(&out).contains("\n## "), "{topic} must omit subsequent headings");
    }
}

#[test]
fn topic_names_are_case_sensitive() {
    let repo = Repo::new();
    let exact = repo.gir(&["explain", "feat"]);
    assert_eq!(exact.status.code(), Some(0), "lowercase topic should exist");
    let changed = repo.gir(&["explain", "Feat"]);
    assert_eq!(changed.status.code(), Some(2), "changed case must not match");
    assert!(stderr(&changed).contains("no topic `Feat`"), "changed case must be reported: {}", stderr(&changed));
}

#[test]
fn unknown_topic_reports_name_and_available_topics() {
    let repo = Repo::new();
    let out = repo.gir(&["explain", "does-not-exist"]);
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(2), "unknown topic should exit 2");
    assert!(err.contains("gir: no topic `does-not-exist`"), "unknown topic should be named: {err}");
    assert!(err.contains("topics: "), "available topics should be shown: {err}");
    assert!(out.stdout.is_empty(), "unknown topic must not print a page");
}

#[test]
fn types_page_uses_configured_order_and_defaults_without_usable_config() {
    let repo = Repo::new();
    repo.write(".girconfig", "[gir]\n\ttypes = widget fix feat\n");
    let configured = stdout(&repo.gir(&["explain", "types"]));
    assert!(configured.starts_with("Allowed types (gir.types in .girconfig):\n"), "types heading is missing: {configured}");
    let lines: Vec<_> = configured.lines().filter(|line| line.starts_with("  ")).collect();
    assert_eq!(
        lines.iter().map(|line| line.split_whitespace().next().unwrap()).collect::<Vec<_>>(),
        ["widget", "fix", "feat"],
        "configured type order must be kept"
    );
    repo.write(".girconfig", "[gir]\n\tsubjectMax = invalid\n");
    let fallback = stdout(&repo.gir(&["explain", "types"]));
    let fallback_types: Vec<_> =
        fallback.lines().filter(|line| line.starts_with("  ")).map(|line| line.split_whitespace().next().unwrap()).collect();
    assert_eq!(
        fallback_types,
        ["feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert"],
        "invalid config must use default type order"
    );
}

#[test]
fn types_page_uses_cheatsheet_summaries_and_group_fallback() {
    let repo = Repo::new();
    repo.write(".girconfig", "[gir]\n\ttypes = feat fix docs style refactor perf test build ci chore revert widget\n");
    let page = stdout(&repo.gir(&["explain", "types"]));
    let cheatsheet = include_str!("../CHEATSHEET.md");
    for ty in ["feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert"] {
        let heading = format!("## {ty}\n");
        let summary = cheatsheet.split_once(&heading).unwrap().1.lines().next().unwrap();
        let line = page.lines().find(|line| line.trim_start().starts_with(&format!("{ty} "))).unwrap();
        assert_eq!(line.trim_start().strip_prefix(ty).unwrap().trim_start(), summary, "{ty} summary must be the cheatsheet first line");
    }
    assert!(page.contains("  widget    changelog group: Widget\n"), "custom type must use its group title: {page}");
}

#[test]
fn types_page_prints_alias_mappings_and_footer() {
    let repo = Repo::new();
    let out = repo.gir(&["explain", "types"]);
    let page = stdout(&out);
    assert_eq!(out.status.code(), Some(0), "types page should succeed: {}", stderr(&out));
    assert!(
        page.contains(
            "Auto-mapped aliases: feature->feat bugfix->fix hotfix->fix doc->docs tests->test refactoring->refactor chores->chore\n"
        ),
        "aliases must be listed as mappings: {page}"
    );
    assert!(
        page.ends_with("When to use each, with examples: gir explain <type>. Also: gir explain breaking, scopes, fixup.\n"),
        "types page must end with its navigation footer: {page}"
    );
    repo.write(".girconfig", "[gir \"alias\"]\n\trelease = feat\n");
    let configured = stdout(&repo.gir(&["explain", "types"]));
    assert!(configured.contains("release->feat"), "configured alias must appear in the types page: {configured}");
}

fn files_under(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn visit(root: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                visit(root, &path, files);
            } else {
                files.insert(path.strip_prefix(root).unwrap().to_string_lossy().to_string(), std::fs::read(path).unwrap());
            }
        }
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

#[test]
fn explain_does_not_change_repository_files_or_git_config() {
    let repo = Repo::new();
    repo.commit_file("tracked.txt", "original\n", "docs: add tracked file");
    repo.write(".girconfig", "[gir]\n\ttypes = feat widget\n");
    let before_files = files_under(&repo.dir);
    let before_global = std::fs::read(&repo.global).unwrap();
    let before_index = repo.git(&["ls-files", "--stage"]);
    let before_config = repo.git(&["config", "--list", "--show-origin"]);
    for args in [&["explain"][..], &["explain", "feat"], &["explain", "types"], &["explain", "missing"]] {
        repo.gir(args);
    }
    assert_eq!(files_under(&repo.dir), before_files, "explain must not create or modify repository files");
    assert_eq!(std::fs::read(&repo.global).unwrap(), before_global, "explain must not change global git config");
    assert_eq!(repo.git(&["ls-files", "--stage"]), before_index, "explain must not change the index");
    assert_eq!(repo.git(&["config", "--list", "--show-origin"]), before_config, "explain must not change git config");
}
