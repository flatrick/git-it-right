mod common;

use std::io::Write;
use std::process::{Output, Stdio};

use common::{Repo, stderr};

fn lint_stdin(repo: &Repo, args: &[&str], message: &str) -> Output {
    let mut child =
        repo.cmd(env!("CARGO_BIN_EXE_gir")).args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(message.as_bytes()).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn autosquash_subjects_pass_file_and_stdin_without_checks() {
    let repo = Repo::new();
    for prefix in ["fixup!", "squash!", "amend!"] {
        let message = format!("{prefix} not a Conventional Commit\n\ninvalid body");
        repo.write("MSG", &message);
        let file = repo.gir(&["lint", "MSG"]);
        assert_eq!(file.status.code(), Some(0), "{prefix}: {}", stderr(&file));
        assert!(stderr(&file).is_empty(), "{prefix}: {}", stderr(&file));
        let stdin = lint_stdin(&repo, &["lint", "-"], &message);
        assert_eq!(stdin.status.code(), Some(0), "{prefix}: {}", stderr(&stdin));
        assert!(stderr(&stdin).is_empty(), "{prefix}: {}", stderr(&stdin));
    }
}

#[test]
fn lint_file_rejects_without_changing_file() {
    let repo = Repo::new();
    repo.write("MSG", "not a Conventional Commit\n");
    let out = repo.gir(&["lint", "MSG"]);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    assert!(stderr(&out).contains("gir: rejected [type-missing]"), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(repo.dir.join("MSG")).unwrap(), "not a Conventional Commit\n");
}

#[test]
fn lint_file_fix_rewrites_message_and_preserves_comments() {
    let repo = Repo::new();
    repo.write("MSG", "Feat: add x.\n# note\n");
    let out = repo.gir(&["lint", "MSG", "--fix"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(repo.dir.join("MSG")).unwrap(), "feat: add x\n\n# note\n");
    assert!(stderr(&out).contains("gir: fixed [type-case] Feat -> feat"), "{}", stderr(&out));
    assert!(stderr(&out).contains("gir: fixed [desc-period] add x. -> add x"), "{}", stderr(&out));
}

#[test]
fn lint_reads_stdin_with_or_without_dash_and_prints_fixed_message() {
    let repo = Repo::new();
    for args in [["lint", "--fix"].as_slice(), ["lint", "-", "--fix"].as_slice()] {
        let out = lint_stdin(&repo, args, "Feat: add x.\n");
        assert_eq!(out.status.code(), Some(0), "{args:?}: {}", stderr(&out));
        assert_eq!(String::from_utf8(out.stdout).unwrap(), "feat: add x\n", "{args:?}");
    }
    let out = lint_stdin(&repo, &["lint", "--fix", "--json"], "Feat: add x.\n");
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(String::from_utf8_lossy(&out.stdout).starts_with("[{\"commit\":null,"));
    assert!(!out.stdout.starts_with(b"feat: add x\n"));
}

#[test]
fn lint_range_rejects_each_unsquashed_autosquash_subject() {
    let repo = Repo::new();
    repo.commit_file("base.txt", "base\n", "chore: base");
    for prefix in ["fixup!", "squash!", "amend!"] {
        repo.commit_file("change.txt", prefix, &format!("{prefix} feat: add x"));
        let sha = repo.git(&["rev-parse", "HEAD"]);
        let out = repo.gir(&["lint", "--range", "HEAD~1..HEAD"]);
        assert_eq!(out.status.code(), Some(1), "{prefix}: {}", stderr(&out));
        assert!(stderr(&out).contains(&format!("gir: {} rejected [fixup-unsquashed]", &sha[..10])), "{}", stderr(&out));
        assert!(stderr(&out).contains("  try: git rebase --autosquash <base>"), "{}", stderr(&out));
    }
}

#[test]
fn lint_range_rejects_every_pending_safe_fix_and_shows_fixed_subject() {
    let repo = Repo::new();
    repo.commit_file("base.txt", "base\n", "chore: base");
    repo.commit_file("change.txt", "change\n", "Feature(api) :Add retry.");
    let out = repo.gir(&["lint", "--range", "HEAD~1..HEAD"]);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    let err = stderr(&out);
    assert!(err.contains("[fix-pending]"), "{err}");
    for rule in ["header-spacing", "type-case", "type-alias", "desc-period"] {
        assert!(err.contains(rule), "missing {rule}: {err}");
    }
    assert!(err.contains("  try: feat(api): Add retry"), "{err}");
}

#[test]
fn lint_range_refuses_fix() {
    let repo = Repo::new();
    repo.commit_file("a.txt", "a\n", "Feat: add a.");
    let out = repo.gir(&["lint", "--range", "HEAD", "--fix"]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert_eq!(stderr(&out), "gir: gir lint --range cannot --fix recorded commits\n");
}

#[test]
fn lint_range_accepts_any_revision_git_log_takes() {
    let repo = Repo::new();
    repo.commit_file("a.txt", "a\n", "not conventional");
    repo.commit_file("b.txt", "b\n", "feat: add b");
    let out = repo.gir(&["lint", "--range", "HEAD"]);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    assert!(stderr(&out).contains("rejected [type-missing]"), "a bare revision must lint its whole history: {}", stderr(&out));
}

#[test]
fn lint_uses_last_range_value() {
    let repo = Repo::new();
    repo.commit_file("bad.txt", "bad\n", "not conventional");
    repo.commit_file("good.txt", "good\n", "feat: good");
    let out = repo.gir(&["lint", "--range", "HEAD", "--range", "HEAD~1..HEAD"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(out.stdout.is_empty());
    assert_eq!(stderr(&out), "");
}
