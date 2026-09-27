//! Keeps CLIENT-TESTING.md honest: its setup script builds the repository it
//! describes, and steps C7 and C8 print what the guide says they print.
#![cfg(unix)]

mod common;

use common::{Repo, stderr};

const GUIDE: &str = include_str!("../CLIENT-TESTING.md");

/// Runs the setup script and returns a `Repo` whose directory is the new `work` clone.
fn setup() -> Repo {
    let mut repo = Repo::new();
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/client-test-repo.sh");
    let target = repo.dir.join("gir-client-test");
    let out = repo.cmd("sh").arg(&script).arg(&target).output().unwrap();
    assert!(out.status.success(), "{}", stderr(&out));
    repo.dir = target.join("work");
    repo
}

fn stage_two(repo: &Repo) {
    repo.write("a.txt", "one\nTWO\nthree\n");
    repo.write("b.txt", "B\n");
    repo.git(&["add", "a.txt", "b.txt"]);
}

#[test]
fn setup_script_builds_the_described_repository() {
    let repo = setup();
    assert_eq!(repo.git(&["symbolic-ref", "--short", "HEAD"]), "topic");
    assert_eq!(repo.git(&["log", "--format=%s", "main..topic"]), "feat: add b\nfeat: add a");
    assert_eq!(repo.git(&["log", "--format=%s", "main"]), "chore: base");
    assert_eq!(repo.git(&["rev-parse", "client-test-start"]), repo.git(&["rev-parse", "HEAD"]));
    assert_eq!(repo.git(&["config", "core.hooksPath"]), ".githooks");
    assert_eq!(repo.git(&["rev-parse", "origin/main"]), repo.git(&["rev-parse", "main"]));
    assert_eq!(repo.git(&["status", "--porcelain"]), "");
}

#[test]
fn setup_script_refuses_an_existing_directory() {
    let repo = Repo::new();
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/client-test-repo.sh");
    let out = repo.cmd("sh").arg(&script).arg(&repo.dir).output().unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("already exists"), "{}", stderr(&out));
}

#[test]
fn c7_output_matches_the_guide() {
    let repo = setup();
    stage_two(&repo);
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(2));
    let err = stderr(&out);
    for line in err.lines().map(sha_placeholder) {
        assert!(GUIDE.contains(&line), "CLIENT-TESTING.md is missing {line:?}");
    }
}

/// `  0123456789 feat: add a` becomes `  <sha> feat: add a`, as the guide writes it.
fn sha_placeholder(line: &str) -> String {
    match line.strip_prefix("  ").and_then(|rest| rest.split_once(' ')) {
        Some((sha, rest)) if sha.len() == 10 && sha.bytes().all(|b| b.is_ascii_hexdigit()) => format!("  <sha> {rest}"),
        _ => line.to_string(),
    }
}

#[test]
fn c8_split_matches_the_guide() {
    let repo = setup();
    stage_two(&repo);
    let out = repo.gir(&["fixup", "--split"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(stderr(&out).lines().last().unwrap().starts_with("  fold: git rebase --autosquash "));
    assert_eq!(repo.git(&["diff", "--cached", "--name-only"]), "");
    for (subject, file) in [("fixup! feat: add a", "a.txt"), ("fixup! feat: add b", "b.txt")] {
        let sha = repo.git(&["log", "--format=%H", "--grep", &format!("^{subject}$")]);
        assert_eq!(repo.git(&["diff-tree", "--no-commit-id", "--name-only", "-r", &sha]), file, "{subject}");
    }
}
