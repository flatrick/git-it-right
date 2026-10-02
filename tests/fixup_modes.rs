mod common;

use common::{Repo, stderr};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

const EDITOR: (&str, &str) = ("GIT_EDITOR", "true");
const INTERACTIVE: (&str, &str) = ("GIR_INTERACTIVE", "1");

fn topic_repo() -> Repo {
    let repo = Repo::new();
    repo.commit_file("base.txt", "base\n", "chore: base");
    repo.git(&["switch", "-q", "-c", "topic"]);
    repo.commit_file("a.txt", "one\ntwo\nthree\n", "feat: add a");
    repo
}

/// Topic branch with `a.txt` from "feat: add a" and `b.txt` from "feat: add b",
/// both changed and staged. Returns the two commits in the order gir lists them.
fn two_target_repo() -> (Repo, [(String, &'static str); 2]) {
    let repo = topic_repo();
    let a = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("b.txt", "b\n", "feat: add b");
    let b = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    stage(&repo, "b.txt", "B\n");
    let mut targets = [(a, "feat: add a"), (b, "feat: add b")];
    targets.sort();
    (repo, targets)
}

fn stage(repo: &Repo, path: &str, content: &str) {
    repo.write(path, content);
    repo.git(&["add", path]);
}

fn subjects(repo: &Repo, count: usize) -> Vec<String> {
    repo.git(&["log", &format!("-{count}"), "--format=%s"]).lines().map(str::to_string).collect()
}

fn files_in(repo: &Repo, rev: &str) -> String {
    repo.git(&["diff-tree", "--no-commit-id", "--name-only", "-r", rev])
}

#[test]
fn amend_creates_an_amend_commit_with_the_staged_changes() {
    let repo = topic_repo();
    let base = repo.git(&["rev-parse", "main"]);
    let target = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let out = repo.gir_with(&["amend"], &[EDITOR], "");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(
        stderr(&out),
        format!("gir: created amend! for {} feat: add a\n  fold: git rebase --autosquash {}\n", &target[..10], &base[..10])
    );
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "amend! feat: add a");
    assert_eq!(repo.git(&["show", "HEAD:a.txt"]), "one\nTWO\nthree");
}

#[test]
fn amend_with_nothing_staged_points_to_reword() {
    let repo = topic_repo();
    let head = repo.git(&["rev-parse", "HEAD"]);
    let out = repo.gir_with(&["amend", "HEAD"], &[EDITOR], "");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(
        stderr(&out),
        "gir: nothing staged; `git add` the fix first, or change only the message: gir reword (more: gir explain fixup)\n"
    );
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), head);
}

#[test]
fn reword_changes_no_content_and_leaves_the_index_alone() {
    let repo = topic_repo();
    let target = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let out = repo.gir_with(&["reword", &target], &[EDITOR], "");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(stderr(&out).starts_with(&format!("gir: created amend! for {} feat: add a\n", &target[..10])), "{}", stderr(&out));
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "amend! feat: add a");
    assert_eq!(repo.git(&["rev-parse", "HEAD^{tree}"]), repo.git(&["rev-parse", "HEAD~1^{tree}"]));
    assert_eq!(repo.git(&["diff", "--cached", "--name-only"]), "a.txt");
}

#[test]
fn reword_needs_nothing_staged() {
    let repo = topic_repo();
    let out = repo.gir_with(&["reword", "HEAD"], &[EDITOR], "");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "amend! feat: add a");
}

#[test]
fn reword_without_a_target_needs_one_when_not_interactive() {
    let repo = topic_repo();
    let head = repo.git(&["rev-parse", "HEAD"]);
    let out = repo.gir_with(&["reword"], &[EDITOR], "1\n");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: nothing to trace for a reword; pass one: gir reword <commit>\n");
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), head);
}

#[test]
fn squash_creates_a_squash_commit_for_the_traced_target() {
    let repo = topic_repo();
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let out = repo.gir_with(&["squash"], &[EDITOR], "");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "squash! feat: add a");
}

#[test]
fn explicit_target_checks_apply_to_every_subcommand() {
    let repo = topic_repo();
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    for sub in ["amend", "reword", "squash"] {
        let out = repo.gir_with(&[sub, "main"], &[EDITOR], "");
        assert_eq!(out.status.code(), Some(2), "{sub}");
        assert_eq!(stderr(&out), "gir: `main` is already on the base branch\n", "{sub}");
    }
}

#[test]
fn refusals_name_the_subcommand_that_was_run() {
    let repo = topic_repo();
    stage(&repo, "new.txt", "new\n");
    let out = repo.gir_with(&["squash"], &[EDITOR], "");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: new.txt is a new file, so it has no earlier commit; pass one: gir squash <commit>\n");
}

#[test]
fn dry_run_prints_the_target_for_every_subcommand() {
    let repo = topic_repo();
    let target = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    for args in [vec!["amend", "--dry-run"], vec!["squash", "--dry-run"], vec!["reword", "HEAD", "--dry-run"]] {
        let out = repo.gir_with(&args, &[EDITOR], "");
        assert_eq!(out.status.code(), Some(0), "{args:?}: {}", stderr(&out));
        assert_eq!(String::from_utf8_lossy(&out.stdout), format!("{} feat: add a\n", &target[..10]), "{args:?}");
    }
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), target);
}

#[test]
fn several_targets_without_a_terminal_refuse_and_suggest_split() {
    let (repo, _) = two_target_repo();
    let out = repo.gir_with(&["fixup"], &[], "1\n");
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(2));
    assert!(err.contains("staged changes belong to several commits:"), "{err}");
    assert!(err.contains("\n  or: gir fixup --split creates one fixup! per commit\n"), "{err}");
    assert!(!err.contains("pick ["), "no prompt without a terminal: {err}");
}

#[test]
fn split_creates_one_commit_per_target() {
    let (repo, targets) = two_target_repo();
    let goal = repo.git(&["write-tree"]);
    let out = repo.gir(&["fixup", "--split"]);
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(0), "{err}");
    for (sha, subject) in &targets {
        assert!(err.contains(&format!("gir: created fixup! for {} {subject}\n", &sha[..10])), "{err}");
    }
    assert_eq!(err.matches("  fold: ").count(), 1, "{err}");
    assert_eq!(subjects(&repo, 2), vec![format!("fixup! {}", targets[1].1), format!("fixup! {}", targets[0].1)]);
    assert_eq!(files_in(&repo, "HEAD~1"), if targets[0].1 == "feat: add a" { "a.txt" } else { "b.txt" });
    assert_eq!(files_in(&repo, "HEAD"), if targets[1].1 == "feat: add a" { "a.txt" } else { "b.txt" });
    assert_eq!(repo.git(&["rev-parse", "HEAD^{tree}"]), goal);
    assert_eq!(repo.git(&["status", "--porcelain"]), "");
}

#[test]
fn split_handles_alternating_targets_in_one_file() {
    // Insertions alternate B, A, B, so whichever target goes first, a skipped hunk
    // that shifts later lines comes before one that is applied.
    let repo = topic_repo();
    let a = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("a.txt", "b1\nb2\none\ntwo\nthree\nb3\nb4\n", "feat: wrap a");
    let b = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "b1\nX\nb2\none\nY\ntwo\nthree\nb3\nZ\nb4\n");
    let goal = repo.git(&["write-tree"]);
    let out = repo.gir(&["fixup", "--split"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let (first, second) = if a < b { (("feat: add a", "1"), ("feat: wrap a", "2")) } else { (("feat: wrap a", "2"), ("feat: add a", "1")) };
    assert_eq!(subjects(&repo, 2), vec![format!("fixup! {}", second.0), format!("fixup! {}", first.0)]);
    assert_eq!(repo.git(&["diff", "--numstat", "HEAD~2", "HEAD~1"]), format!("{}\t0\ta.txt", first.1));
    assert_eq!(repo.git(&["diff", "--numstat", "HEAD~1", "HEAD"]), format!("{}\t0\ta.txt", second.1));
    assert_eq!(repo.git(&["rev-parse", "HEAD^{tree}"]), goal);
    assert_eq!(repo.git(&["status", "--porcelain"]), "");
}

#[test]
fn split_refuses_a_hunk_spanning_several_commits_before_committing() {
    let repo = topic_repo();
    repo.commit_file("a.txt", "one\ntwo\nthree\nfour\n", "feat: extend a");
    let head = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\ntwo\nTHREE\nFOUR\n");
    let goal = repo.git(&["write-tree"]);
    let out = repo.gir(&["fixup", "--split"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: a.txt:3 spans several commits; split it with git add -p\n");
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), head);
    assert_eq!(repo.git(&["write-tree"]), goal);
}

#[test]
fn split_restores_head_and_index_when_a_commit_fails() {
    let (repo, targets) = two_target_repo();
    let head = repo.git(&["rev-parse", "HEAD"]);
    let goal = repo.git(&["write-tree"]);
    let hook = repo.dir.join(".git/hooks/commit-msg");
    let second = targets[1].1;
    std::fs::write(&hook, format!("#!/bin/sh\n! grep -q '{second}' \"$1\"\n")).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    let out = repo.gir(&["fixup", "--split"]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(stderr(&out).contains("restored HEAD and the index"), "{}", stderr(&out));
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), head);
    assert_eq!(repo.git(&["write-tree"]), goal);
}

fn sparse_checkout(repo: &Repo) {
    repo.commit_file("out/c.txt", "c\n", "feat: add c");
    repo.git(&["sparse-checkout", "set", "--no-cone", "/*", "!/out/"]);
}

fn skip_worktree_with_local_edit(repo: &Repo) {
    repo.commit_file("cfg.txt", "c\n", "feat: add cfg");
    repo.git(&["update-index", "--skip-worktree", "cfg.txt"]);
    repo.write("cfg.txt", "private local edit\n");
}

fn sparse_index(repo: &Repo) {
    repo.commit_file("out/c.txt", "c\n", "feat: add c");
    repo.git(&["sparse-checkout", "set", "--cone", "--sparse-index", "in"]);
}

fn intent_to_add(repo: &Repo) {
    repo.write("ita.txt", "not staged yet\n");
    repo.git(&["add", "-N", "ita.txt"]);
}

/// `git status --short` without the lines for `skip`, and `git ls-files --sparse -v`, which shows `S` for
/// skip-worktree and a sparse index's directory entries.
fn index_view(repo: &Repo, skip: &[&str]) -> (Vec<String>, String) {
    let status = repo.git(&["status", "--short"]).lines().filter(|l| !skip.contains(&&l[3..])).map(str::to_string).collect();
    (status, repo.git(&["ls-files", "--sparse", "-v"]))
}

/// `gir-split-index-*` files left in `dir`'s git directory or its common git directory.
fn leftover_temp_indexes(repo: &Repo, dir: &Path) -> Vec<String> {
    let mut found = Vec::new();
    for which in ["--absolute-git-dir", "--git-common-dir"] {
        let out = repo.cmd("git").current_dir(dir).args(["rev-parse", "--path-format=absolute", which]).output().unwrap();
        let git_dir = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
        for entry in std::fs::read_dir(&git_dir).unwrap() {
            let name = entry.unwrap().file_name().to_string_lossy().to_string();
            if name.starts_with("gir-split-index-") {
                found.push(name);
            }
        }
    }
    found
}

fn split_repo_with(setup: Setup) -> (Repo, [(String, &'static str); 2]) {
    let repo = topic_repo();
    let a = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("b.txt", "b\n", "feat: add b");
    let b = repo.git(&["rev-parse", "HEAD"]);
    setup(&repo);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    stage(&repo, "b.txt", "B\n");
    let mut targets = [(a, "feat: add a"), (b, "feat: add b")];
    targets.sort();
    (repo, targets)
}

type Setup = fn(&Repo);

const INDEX_SETUPS: [(&str, Setup); 4] = [
    ("sparse checkout", sparse_checkout),
    ("sparse index", sparse_index),
    ("skip-worktree", skip_worktree_with_local_edit),
    ("intent-to-add", intent_to_add),
];

#[test]
fn split_leaves_index_entries_it_does_not_commit_alone() {
    for (name, setup) in INDEX_SETUPS {
        let (repo, _) = split_repo_with(setup);
        let before = index_view(&repo, &["a.txt", "b.txt"]);
        let out = repo.gir(&["fixup", "--split"]);
        assert_eq!(out.status.code(), Some(0), "{name}: {}", stderr(&out));
        assert_eq!(index_view(&repo, &["a.txt", "b.txt"]), before, "{name}");
        assert_eq!(repo.git(&["diff", "--cached", "--name-only"]), "", "{name}");
        assert_eq!(leftover_temp_indexes(&repo, &repo.dir), Vec::<String>::new(), "{name}");
    }
}

#[test]
fn failed_split_leaves_index_entries_alone() {
    for (name, setup) in INDEX_SETUPS {
        let (repo, targets) = split_repo_with(setup);
        let head = repo.git(&["rev-parse", "HEAD"]);
        let hook = repo.dir.join(".git/hooks/commit-msg");
        std::fs::write(&hook, format!("#!/bin/sh\n! grep -q '{}' \"$1\"\n", targets[1].1)).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        let before = index_view(&repo, &[]);
        let out = repo.gir(&["fixup", "--split"]);
        assert_eq!(out.status.code(), Some(2), "{name}: {}", stderr(&out));
        assert!(stderr(&out).contains("restored HEAD and the index"), "{name}: {}", stderr(&out));
        assert_eq!(repo.git(&["rev-parse", "HEAD"]), head, "{name}");
        assert_eq!(index_view(&repo, &[]), before, "{name}");
        assert_eq!(leftover_temp_indexes(&repo, &repo.dir), Vec::<String>::new(), "{name}");
    }
}

#[test]
fn add_all_and_commit_all_after_split_keep_hidden_and_private_files() {
    let (repo, _) = split_repo_with(sparse_checkout);
    let out = repo.gir(&["fixup", "--split"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    repo.git(&["add", "-A"]);
    let _ = repo.git_out(&["commit", "-q", "-a", "-m", "chore: everything"]);
    assert_eq!(repo.git(&["show", "HEAD:out/c.txt"]), "c");

    let (repo, _) = split_repo_with(skip_worktree_with_local_edit);
    let out = repo.gir(&["fixup", "--split"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    repo.git(&["add", "-A"]);
    let _ = repo.git_out(&["commit", "-q", "-a", "-m", "chore: everything"]);
    assert_eq!(repo.git(&["show", "HEAD:cfg.txt"]), "c");
    assert_eq!(std::fs::read_to_string(repo.dir.join("cfg.txt")).unwrap(), "private local edit\n");
}

#[test]
fn split_in_a_linked_worktree_keeps_its_index_entries() {
    let repo = topic_repo();
    repo.commit_file("b.txt", "b\n", "feat: add b");
    repo.commit_file("cfg.txt", "c\n", "feat: add cfg");
    let wt = repo.dir.parent().unwrap().join("linked");
    repo.git(&["worktree", "add", "-q", "-b", "linked", wt.to_str().unwrap(), "topic"]);
    let git_in = |args: &[&str]| {
        let out = repo.cmd("git").current_dir(&wt).args(args).output().unwrap();
        assert!(out.status.success(), "git {args:?}: {}", stderr(&out));
        String::from_utf8_lossy(&out.stdout).trim_end().to_string()
    };
    git_in(&["update-index", "--skip-worktree", "cfg.txt"]);
    std::fs::write(wt.join("cfg.txt"), "private local edit\n").unwrap();
    std::fs::write(wt.join("a.txt"), "one\nTWO\nthree\n").unwrap();
    std::fs::write(wt.join("b.txt"), "B\n").unwrap();
    git_in(&["add", "a.txt", "b.txt"]);
    let flags = git_in(&["ls-files", "-v"]);
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(&wt).args(["fixup", "--split"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(git_in(&["status", "--short"]), "");
    assert_eq!(git_in(&["ls-files", "-v"]), flags);
    assert_eq!(leftover_temp_indexes(&repo, &wt), Vec::<String>::new());
    assert_eq!(repo.git(&["status", "--short"]), "");
}

#[test]
fn split_from_a_subdirectory_keeps_index_entries() {
    let (repo, _) = split_repo_with(skip_worktree_with_local_edit);
    let sub = repo.dir.join("sub");
    std::fs::create_dir(&sub).unwrap();
    let before = index_view(&repo, &["a.txt", "b.txt"]);
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(&sub).args(["fixup", "--split"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(index_view(&repo, &["a.txt", "b.txt"]), before);
    assert_eq!(repo.git(&["diff", "--cached", "--name-only"]), "");
}

#[test]
fn picker_number_puts_all_staged_changes_in_that_commit() {
    let (repo, targets) = two_target_repo();
    let goal = repo.git(&["write-tree"]);
    let out = repo.gir_with(&["fixup"], &[INTERACTIVE], "9\n2\n");
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(0), "{err}");
    assert!(err.contains(&format!("  1) {} {}", &targets[0].0[..10], targets[0].1)), "{err}");
    assert!(err.contains(&format!("  2) {} {}", &targets[1].0[..10], targets[1].1)), "{err}");
    assert!(err.contains("  s) split: one fixup! per commit"), "{err}");
    assert_eq!(err.matches("pick [1-2, s, q]: ").count(), 2, "an invalid answer asks again: {err}");
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), format!("fixup! {}", targets[1].1));
    assert_eq!(repo.git(&["rev-parse", "HEAD^{tree}"]), goal);
}

#[test]
fn picker_split_creates_one_commit_per_target() {
    let (repo, targets) = two_target_repo();
    let out = repo.gir_with(&["fixup"], &[INTERACTIVE], "s\n");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(subjects(&repo, 2), vec![format!("fixup! {}", targets[1].1), format!("fixup! {}", targets[0].1)]);
}

#[test]
fn picker_cancel_creates_no_commit() {
    for input in ["\n", "q\n", ""] {
        let (repo, _) = two_target_repo();
        let head = repo.git(&["rev-parse", "HEAD"]);
        let out = repo.gir_with(&["fixup"], &[INTERACTIVE], input);
        assert_eq!(out.status.code(), Some(1), "{input:?}: {}", stderr(&out));
        assert!(stderr(&out).ends_with("gir: cancelled; nothing committed\n"), "{input:?}: {}", stderr(&out));
        assert_eq!(repo.git(&["rev-parse", "HEAD"]), head);
    }
}

#[test]
fn picker_can_be_disabled_and_never_runs_for_dry_run() {
    let (repo, _) = two_target_repo();
    for (args, env) in [(vec!["fixup"], ("GIR_INTERACTIVE", "0")), (vec!["fixup", "--dry-run"], INTERACTIVE)] {
        let out = repo.gir_with(&args, &[env], "1\n");
        assert_eq!(out.status.code(), Some(2), "{args:?}");
        assert!(!stderr(&out).contains("pick ["), "{args:?}: {}", stderr(&out));
    }
}

#[test]
fn picker_offers_branch_commits_for_a_new_file() {
    let repo = topic_repo();
    let target = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "new.txt", "new\n");
    let out = repo.gir_with(&["fixup"], &[INTERACTIVE], "1\n");
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(0), "{err}");
    assert!(err.contains("new.txt is a new file"), "{err}");
    assert!(err.contains(&format!("  1) {} feat: add a", &target[..10])), "{err}");
    assert!(!err.contains("s) split"), "{err}");
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "fixup! feat: add a");
}

#[test]
fn picker_offers_branch_commits_for_reword() {
    let repo = topic_repo();
    let out = repo.gir_with(&["reword"], &[INTERACTIVE, EDITOR], "1\n");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "amend! feat: add a");
}

#[test]
fn split_is_refused_with_a_commit_argument_and_for_reword() {
    let (repo, _) = two_target_repo();
    let out = repo.gir(&["fixup", "--split", "HEAD"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: --split finds each commit itself; drop the commit argument\n");
    let out = repo.gir(&["reword", "--split"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).starts_with("gir: unknown option --split for `gir reword`"), "{}", stderr(&out));
}

#[test]
fn amend_and_squash_accept_an_explicit_target() {
    for (sub, prefix) in [("amend", "amend!"), ("squash", "squash!")] {
        let repo = topic_repo();
        let target = repo.git(&["rev-parse", "HEAD"]);
        repo.commit_file("b.txt", "b\n", "feat: add b");
        stage(&repo, "b.txt", "B\n");
        let out = repo.gir_with(&[sub, &target], &[EDITOR], "");
        assert_eq!(out.status.code(), Some(0), "{sub}: {}", stderr(&out));
        assert_eq!(repo.git(&["log", "-1", "--format=%s"]), format!("{prefix} feat: add a"), "{sub}");
        assert_eq!(repo.git(&["show", "HEAD:b.txt"]), "B", "{sub}");
    }
}

/// The file contents of one byte-level split case: two commits, the staged version, and
/// what each commit must hold after `rebase --autosquash`.
struct Bytes<'a> {
    first: &'a [u8],
    second: &'a [u8],
    staged: &'a [u8],
    want_first: &'a [u8],
    want_second: &'a [u8],
}

fn split_and_fold_keeps_bytes(name: &OsStr, case: Bytes, base: &[(&str, &[u8])], config: &[(&str, &str)]) {
    let repo = Repo::new();
    for (key, value) in config {
        repo.git(&["config", key, value]);
    }
    std::fs::write(repo.dir.join("base.txt"), "base\n").unwrap();
    for (path, data) in base {
        std::fs::write(repo.dir.join(path), data).unwrap();
    }
    repo.git(&["add", "-A"]);
    repo.git(&["commit", "-q", "-m", "chore: base"]);
    repo.git(&["switch", "-q", "-c", "topic"]);
    for (data, msg) in [(case.first, "feat: first"), (case.second, "feat: second")] {
        std::fs::write(repo.dir.join(name), data).unwrap();
        repo.git(&["add", "-A"]);
        repo.git(&["commit", "-q", "-m", msg]);
    }
    std::fs::write(repo.dir.join(name), case.staged).unwrap();
    repo.git(&["add", "-A"]);
    let out = repo.gir(&["fixup", "--split"]);
    assert_eq!(out.status.code(), Some(0), "{name:?}: {}", stderr(&out));
    repo.git_with(&["rebase", "-q", "-i", "--autosquash", "main"], &[("GIT_SEQUENCE_EDITOR", "true")]);
    assert_eq!(repo.git(&["log", "--format=%s", "main..HEAD"]), "feat: second\nfeat: first", "{name:?}");
    for (rev, want) in [("HEAD~1:", case.want_first), ("HEAD:", case.want_second)] {
        let mut spec = OsString::from(rev);
        spec.push(name);
        let got = repo.cmd("git").arg("show").arg(&spec).output().unwrap().stdout;
        assert_eq!(got, want, "{name:?} at {rev}");
    }
}

const FOUR_LINES: Bytes = Bytes {
    first: b"one\ntwo\nthree\nfour\n",
    second: b"one\ntwo\nthree\nFOUR\n",
    staged: b"ONE  \ntwo\nthree\nFOURx   \n",
    want_first: b"ONE  \ntwo\nthree\nfour\n",
    want_second: b"ONE  \ntwo\nthree\nFOURx   \n",
};

#[test]
fn split_keeps_crlf_line_endings() {
    let case = Bytes {
        first: b"one\r\ntwo\r\nthree\r\nfour\r\n",
        second: b"one\r\ntwo\r\nthree\r\nFOUR\r\n",
        staged: b"ONE\r\ntwo\r\nthree\r\nFOURx\r\n",
        want_first: b"ONE\r\ntwo\r\nthree\r\nfour\r\n",
        want_second: b"ONE\r\ntwo\r\nthree\r\nFOURx\r\n",
    };
    split_and_fold_keeps_bytes(OsStr::new("f.txt"), case, &[(".gitattributes", b"* -text\n")], &[]);
}

#[test]
fn split_keeps_non_utf8_content() {
    let case = Bytes {
        first: b"caf\xe9 1\nl2\nl3\nl4\n",
        second: b"caf\xe9 1\nl2\nl3\nL4 \xe9\n",
        staged: b"CAF\xc9 1\nl2\nl3\nL4x \xe9\n",
        want_first: b"CAF\xc9 1\nl2\nl3\nl4\n",
        want_second: b"CAF\xc9 1\nl2\nl3\nL4x \xe9\n",
    };
    split_and_fold_keeps_bytes(OsStr::new("f.txt"), case, &[], &[]);
}

#[test]
fn split_keeps_trailing_whitespace() {
    split_and_fold_keeps_bytes(OsStr::new("f.txt"), FOUR_LINES, &[], &[]);
    let no_final_newline = Bytes {
        staged: b"ONE\ntwo\nthree\nFOURx   ",
        want_first: b"ONE\ntwo\nthree\nfour\n",
        want_second: b"ONE\ntwo\nthree\nFOURx   ",
        ..FOUR_LINES
    };
    split_and_fold_keeps_bytes(OsStr::new("f.txt"), no_final_newline, &[], &[]);
}

#[test]
fn split_ignores_apply_whitespace_and_textconv_settings() {
    for setting in ["error", "fix"] {
        split_and_fold_keeps_bytes(OsStr::new("f.txt"), FOUR_LINES, &[], &[("apply.whitespace", setting)]);
    }
    split_and_fold_keeps_bytes(
        OsStr::new("f.txt"),
        FOUR_LINES,
        &[(".gitattributes", b"*.txt diff=shout\n")],
        &[("diff.shout.textconv", "sed s/^/SHOUT:/")],
    );
}

// Windows file names are UTF-16 and cannot hold bytes that are not valid UTF-8,
// and macOS (APFS) refuses to create such names in the working tree.
#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn split_traces_non_utf8_file_names() {
    use std::os::unix::ffi::OsStrExt;
    split_and_fold_keeps_bytes(OsStr::from_bytes(b"caf\xe9.txt"), FOUR_LINES, &[], &[]);
}

/// `topic_repo` plus `feat: add b`; the caller stages changes.
fn a_and_b_repo() -> Repo {
    let repo = topic_repo();
    repo.commit_file("b.txt", "b\n", "feat: add b");
    repo
}

fn stage_a_and_b(repo: &Repo) {
    stage(repo, "a.txt", "one\nTWO\nthree\n");
    stage(repo, "b.txt", "B\n");
}

/// Stages a change gir cannot trace and returns its path.
fn stage_untraceable(repo: &Repo, kind: &str) -> &'static str {
    match kind {
        "new file" => {
            stage(repo, "new.txt", "new\n");
            "new.txt"
        }
        "binary file" => {
            std::fs::write(repo.dir.join("bin.dat"), b"\x00\x01old").unwrap();
            repo.git(&["add", "bin.dat"]);
            repo.git(&["commit", "-q", "-m", "feat: add bin"]);
            std::fs::write(repo.dir.join("bin.dat"), b"\x00\x01new").unwrap();
            repo.git(&["add", "bin.dat"]);
            "bin.dat"
        }
        _ => {
            repo.git(&["update-index", "--chmod=+x", "b.txt"]);
            "b.txt"
        }
    }
}

#[test]
fn split_without_a_terminal_refuses_an_untraceable_file_with_split_advice() {
    for kind in ["new file", "binary file", "mode-only change"] {
        let repo = a_and_b_repo();
        let path = stage_untraceable(&repo, kind);
        stage(&repo, "a.txt", "one\nTWO\nthree\n");
        let head = repo.git(&["rev-parse", "HEAD"]);
        let out = repo.gir(&["fixup", "--split"]);
        let err = stderr(&out);
        assert_eq!(out.status.code(), Some(2), "{kind}: {err}");
        assert!(err.ends_with(&format!(
            "; --split cannot place it: commit it on its own or unstage it (git restore --staged -- {path}), then run gir fixup --split again\n"
        )), "{kind}: {err}");
        assert!(!err.contains("pass one"), "{kind}: {err}");
        assert_eq!(repo.git(&["rev-parse", "HEAD"]), head, "{kind}");
    }
}

#[test]
fn split_at_a_terminal_asks_where_an_untraceable_file_goes() {
    let repo = a_and_b_repo();
    stage_untraceable(&repo, "new file");
    stage_a_and_b(&repo);
    // newest first: 1) feat: add b, 2) feat: add a
    let out = repo.gir_with(&["fixup", "--split"], &[INTERACTIVE], "2\n");
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(0), "{err}");
    assert!(err.contains("new.txt is a new file, so it has no earlier commit; pick the commit it belongs to:"), "{err}");
    assert_eq!(err.matches("pick [").count(), 1, "{err}");
    repo.git_with(&["rebase", "-q", "-i", "--autosquash", "main"], &[("GIT_SEQUENCE_EDITOR", "true")]);
    assert_eq!(repo.git(&["log", "--format=%s", "main..HEAD"]), "feat: add b\nfeat: add a");
    assert_eq!(repo.git(&["show", "HEAD~1:new.txt"]), "new");
    assert_eq!(repo.git(&["show", "HEAD~1:a.txt"]), "one\nTWO\nthree");
    assert_eq!(repo.git(&["show", "HEAD:b.txt"]), "B");
}

#[test]
fn split_at_a_terminal_places_a_binary_file_whole() {
    let repo = a_and_b_repo();
    stage_untraceable(&repo, "binary file");
    stage_a_and_b(&repo);
    // newest first: 1) feat: add bin
    let out = repo.gir_with(&["fixup", "--split"], &[INTERACTIVE], "1\n");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    repo.git_with(&["rebase", "-q", "-i", "--autosquash", "main"], &[("GIT_SEQUENCE_EDITOR", "true")]);
    assert_eq!(repo.git(&["log", "--format=%s", "main..HEAD"]), "feat: add bin\nfeat: add b\nfeat: add a");
    assert_eq!(repo.git_out(&["show", "HEAD:bin.dat"]).stdout, b"\x00\x01new");
    assert_eq!(repo.git(&["show", "HEAD~2:a.txt"]), "one\nTWO\nthree");
    assert_eq!(repo.git(&["show", "HEAD~1:b.txt"]), "B");
}

/// `f.txt` gets `one` from `feat: one` and `two` from `feat: two`; `mid` is staged between them,
/// along with a change to `a.txt`.
fn insertion_repo() -> Repo {
    let repo = topic_repo();
    repo.commit_file("f.txt", "one\n", "feat: one");
    repo.commit_file("f.txt", "one\ntwo\n", "feat: two");
    stage(&repo, "f.txt", "one\nmid\ntwo\n");
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    repo
}

#[test]
fn split_without_a_terminal_refuses_an_ambiguous_insertion_naming_both_commits() {
    let repo = insertion_repo();
    let head = repo.git(&["rev-parse", "HEAD"]);
    let out = repo.gir(&["fixup", "--split"]);
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(2), "{err}");
    assert!(err.starts_with("gir: f.txt:1 is an insertion between lines of "), "{err}");
    assert!(err.contains("feat: one") && err.contains("feat: two"), "{err}");
    assert!(err.ends_with("; run gir fixup --split in a terminal to choose, or stage it on its own and run gir fixup <commit>\n"), "{err}");
    assert!(!err.contains("add -p"), "{err}");
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), head);
}

#[test]
fn split_at_a_terminal_asks_where_an_ambiguous_insertion_goes() {
    for args in [vec!["fixup", "--split"], vec!["fixup"]] {
        let repo = insertion_repo();
        let input = if args.len() == 1 { "s\n2\n" } else { "2\n" };
        let out = repo.gir_with(&args, &[INTERACTIVE], input);
        let err = stderr(&out);
        assert_eq!(out.status.code(), Some(0), "{args:?}: {err}");
        assert!(err.contains("gir: f.txt:1 is an insertion between lines of two commits; pick the one it belongs to:"), "{args:?}: {err}");
        repo.git_with(&["rebase", "-q", "-i", "--autosquash", "main"], &[("GIT_SEQUENCE_EDITOR", "true")]);
        assert_eq!(repo.git(&["log", "--format=%s", "main..HEAD"]), "feat: two\nfeat: one\nfeat: add a", "{args:?}");
        assert_eq!(repo.git(&["show", "HEAD:f.txt"]), "one\nmid\ntwo", "{args:?}");
        assert_eq!(repo.git(&["show", "HEAD~2:a.txt"]), "one\nTWO\nthree", "{args:?}");
    }
}

#[test]
fn split_is_not_offered_when_a_hunk_changes_lines_of_several_commits() {
    let repo = topic_repo();
    repo.commit_file("a.txt", "one\ntwo\nthree\nfour\n", "feat: extend a");
    repo.commit_file("b.txt", "b\n", "feat: add b");
    stage(&repo, "a.txt", "one\ntwo\nTHREE\nFOUR\n");
    stage(&repo, "b.txt", "B\n");
    let out = repo.gir_with(&["fixup"], &[INTERACTIVE], "q\n");
    let err = stderr(&out);
    assert!(err.contains("pick ["), "{err}");
    assert!(!err.contains("s) split"), "{err}");
    let out = repo.gir(&["fixup"]);
    let err = stderr(&out);
    assert!(err.contains("staged changes belong to several commits:"), "{err}");
    assert!(!err.contains("--split"), "{err}");
}

#[test]
fn split_places_a_file_in_a_commit_with_no_hunks_of_its_own() {
    let repo = a_and_b_repo();
    stage_untraceable(&repo, "new file");
    // newest first: 1) feat: add b
    let out = repo.gir_with(&["fixup", "--split"], &[INTERACTIVE], "1\n");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "fixup! feat: add b");
    assert_eq!(repo.git(&["show", "HEAD:new.txt"]), "new");
}
