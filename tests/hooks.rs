mod common;

use std::io::Write;
use std::process::{Output, Stdio};

use common::{Repo, stderr};

fn pre_push(repo: &Repo, local_sha: &str, remote_sha: &str) -> Output {
    let input = format!("refs/heads/topic {local_sha} refs/heads/topic {remote_sha}\n");
    let mut child = repo.cmd(env!("CARGO_BIN_EXE_gir"))
        .args(["hook", "pre-push", "origin", "unused-url"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    child.wait_with_output().unwrap()
}

fn missing_gir_script(repo: &Repo, name: &str) -> Output {
    let gir = if cfg!(windows) { "gir.exe" } else { "gir" };
    let dirs = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()).filter(|dir| !dir.join(gir).exists()).collect::<Vec<_>>();
    repo.cmd("sh")
        .arg(repo.dir.join(".githooks").join(name))
        .arg("unused-argument")
        .env("PATH", std::env::join_paths(dirs).unwrap())
        .output()
        .unwrap()
}

#[test]
fn commit_msg_accepts_all_autosquash_prefixes_unchanged() {
    let repo = Repo::new();
    for prefix in ["fixup! ", "squash! ", "amend! "] {
        let message = format!("{prefix}feat: original\n");
        repo.write("MSG", &message);
        let out = repo.gir(&["hook", "commit-msg", "MSG"]);
        assert_eq!(out.status.code(), Some(0), "{prefix}: {}", stderr(&out));
        assert_eq!(std::fs::read_to_string(repo.dir.join("MSG")).unwrap(), message, "{prefix} message changed");
        assert!(!stderr(&out).contains("commit rejected"), "{prefix}: {}", stderr(&out));
    }
}

#[test]
fn pre_push_lints_only_commits_missing_from_remote() {
    let repo = Repo::new();
    let remote = repo.dir.parent().unwrap().join("remote.git");
    repo.git(&["init", "-q", "--bare", remote.to_str().unwrap()]);
    repo.git(&["remote", "add", "origin", remote.to_str().unwrap()]);
    repo.commit_file("old.txt", "old\n", "old invalid message");
    let remote_sha = repo.git(&["rev-parse", "HEAD"]);
    repo.git(&["push", "-q", "origin", "main"]);
    repo.commit_file("new.txt", "new\n", "feat: new work");
    let clean_sha = repo.git(&["rev-parse", "HEAD"]);

    let existing = pre_push(&repo, &clean_sha, &remote_sha);
    assert_eq!(existing.status.code(), Some(0), "existing branch: {}", stderr(&existing));
    assert!(!stderr(&existing).contains("old invalid message"), "remote commit was linted: {}", stderr(&existing));

    let zeros = "0".repeat(40);
    let new_branch = pre_push(&repo, &clean_sha, &zeros);
    assert_eq!(new_branch.status.code(), Some(0), "new branch: {}", stderr(&new_branch));

    repo.commit_file("bad.txt", "bad\n", "another invalid message");
    let bad_sha = repo.git(&["rev-parse", "HEAD"]);
    let existing = pre_push(&repo, &bad_sha, &remote_sha);
    assert_eq!(existing.status.code(), Some(1), "existing branch must lint its new commit");
    assert!(stderr(&existing).contains("push rejected [type-missing]"), "{}", stderr(&existing));
    let new_branch = pre_push(&repo, &bad_sha, &zeros);
    assert_eq!(new_branch.status.code(), Some(1), "new branch must lint its new commit");
    assert!(stderr(&new_branch).contains("push rejected [type-missing]"), "{}", stderr(&new_branch));
    assert!(stderr(&new_branch).contains(&bad_sha[..10]), "{}", stderr(&new_branch));

    let fallback = pre_push(&repo, &bad_sha, "deadbeef");
    assert_eq!(fallback.status.code(), Some(1), "failed range must use remote-tracking refs");
    assert!(stderr(&fallback).contains("push rejected [type-missing]"), "{}", stderr(&fallback));
}

#[test]
fn pre_push_skips_branch_deletions() {
    let repo = Repo::new();
    let zeros = "0".repeat(40);
    let out = pre_push(&repo, &zeros, "not-a-commit");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(out.stdout.is_empty(), "deletion printed stdout: {}", String::from_utf8_lossy(&out.stdout));
    assert!(out.stderr.is_empty(), "deletion printed stderr: {}", stderr(&out));
}

#[test]
fn installed_hooks_warn_when_gir_is_missing() {
    let repo = Repo::new();
    assert_eq!(repo.gir(&["init"]).status.code(), Some(0));
    for name in ["commit-msg", "pre-push"] {
        let out = missing_gir_script(&repo, name);
        assert_eq!(out.status.code(), Some(0), "{name}: {}", stderr(&out));
        assert_eq!(stderr(&out), format!("gir: not installed, {name} check skipped\n"));
        assert!(out.stdout.is_empty(), "{name} printed stdout");
    }
}

#[test]
fn installed_hooks_block_when_gir_is_missing_and_configured_to_fail() {
    let repo = Repo::new();
    assert_eq!(repo.gir(&["init"]).status.code(), Some(0));
    repo.write(".girconfig", "[gir]\n\thookMissing = fail\n");
    for name in ["commit-msg", "pre-push"] {
        let out = missing_gir_script(&repo, name);
        assert_eq!(out.status.code(), Some(1), "{name}: {}", stderr(&out));
        assert_eq!(stderr(&out), format!("gir: not installed, {name} check blocked (gir.hookMissing=fail)\n"));
        assert!(out.stdout.is_empty(), "{name} printed stdout");
    }
}

#[test]
fn pre_push_rejects_commits_that_skipped_safe_fixes() {
    let repo = Repo::new();
    repo.commit_file("old.txt", "old\n", "chore: base");
    let remote_sha = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("new.txt", "new\n", "Feature(api) :Add retry.");
    let sha = repo.git(&["rev-parse", "HEAD"]);
    let out = pre_push(&repo, &sha, &remote_sha);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    assert!(stderr(&out).contains(&format!("gir: {} push rejected [fix-pending]", &sha[..10])), "{}", stderr(&out));
    assert!(stderr(&out).contains("  try: feat(api): Add retry"), "{}", stderr(&out));
    assert_eq!(repo.head_message(), "Feature(api) :Add retry.", "pre-push must not modify the commit");
}
