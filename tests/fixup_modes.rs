mod common;

use common::{Repo, stderr};

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
    assert_eq!(stderr(&out), format!("gir: created amend! for {} feat: add a\n  fold: git rebase --autosquash {}\n", &target[..10], &base[..10]));
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "amend! feat: add a");
    assert_eq!(repo.git(&["show", "HEAD:a.txt"]), "one\nTWO\nthree");
}

#[test]
fn amend_with_nothing_staged_points_to_reword() {
    let repo = topic_repo();
    let head = repo.git(&["rev-parse", "HEAD"]);
    let out = repo.gir_with(&["amend", "HEAD"], &[EDITOR], "");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: nothing staged; `git add` the fix first, or change only the message: gir reword (more: gir explain fixup)\n");
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
