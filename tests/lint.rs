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
    let base = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("x.txt", "x\n", "feat: add x");
    for prefix in ["fixup!", "squash!", "amend!"] {
        repo.commit_file("change.txt", prefix, &format!("{prefix} feat: add x"));
        let sha = repo.git(&["rev-parse", "HEAD"]);
        let out = repo.gir(&["lint", "--range", &format!("{base}..HEAD")]);
        assert_eq!(out.status.code(), Some(1), "{prefix}: {}", stderr(&out));
        assert!(stderr(&out).contains(&format!("gir: {} rejected [fixup-unsquashed]", &sha[..10])), "{}", stderr(&out));
        assert!(stderr(&out).contains(&format!("  try: git rebase --autosquash {}\n", &base[..10])), "{}", stderr(&out));
    }
}

/// A branch off `chore: base` with `feat: add a` (a.txt line 2) and `feat: add b`; returns the
/// base and `feat: add a`.
fn branch_with_two_commits(repo: &Repo) -> (String, String) {
    repo.commit_file("a.txt", "1\n2\n3\n", "chore: base");
    let base = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("a.txt", "1\nA\n3\n", "feat: add a");
    let a = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("b.txt", "b\n", "feat: add b");
    (base, a)
}

fn range_lint(repo: &Repo, base: &str) -> String {
    let out = repo.gir(&["lint", "--range", &format!("{base}..HEAD")]);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    stderr(&out)
}

#[test]
fn lint_range_folds_every_specifier_form_git_matches() {
    let repo = Repo::new();
    let (base, a) = branch_with_two_commits(&repo);
    for subject in [
        "fixup! feat: add a".to_string(),
        "fixup! feat: add".to_string(),
        format!("fixup! {a}"),
        format!("fixup! {}", &a[..7]),
        "squash! fixup! feat: add a".to_string(),
    ] {
        repo.commit_file("c.txt", &subject, &subject);
        let err = range_lint(&repo, &base);
        assert!(err.contains("[fixup-unsquashed]") && !err.contains("[fixup-unmatched]"), "{subject}: {err}");
        repo.git(&["reset", "-q", "--hard", "HEAD~1"]);
    }
}

#[test]
fn lint_range_unmatched_fixup_names_the_target_blame_finds() {
    let repo = Repo::new();
    let (base, a) = branch_with_two_commits(&repo);
    for (prefix, message, action) in [
        ("fixup!", "fixup! wip", "fixup"),
        ("squash!", "squash! wip", "squash"),
        ("amend!", "amend! wip\n\nfeat: add alpha", "fixup -C"),
        ("amend!", "amend! wip", "fixup"),
    ] {
        repo.commit_file("a.txt", "1\nAA\n3\n", message);
        let sha = repo.git(&["rev-parse", "HEAD"]);
        let err = range_lint(&repo, &base);
        assert!(
            err.contains(&format!(
                "gir: {} rejected [fixup-unmatched] `{prefix}` commit matches no earlier commit, so git rebase --autosquash leaves it\n",
                &sha[..10]
            )),
            "{err}"
        );
        assert!(
            err.contains(&format!(
                "  try: git rebase -i {}: move it below {} feat: add a, change pick to {action}\n",
                &base[..10],
                &a[..10]
            )),
            "{err}"
        );
        assert!(err.contains("  more: gir explain fixup-unmatched\n"), "{err}");
        assert!(!err.contains("[fixup-unsquashed]"), "{err}");
        repo.git(&["reset", "-q", "--hard", "HEAD~1"]);
    }
}

#[test]
fn lint_range_unmatched_when_target_is_reworded_older_than_it_or_head() {
    let repo = Repo::new();
    let (base, _) = branch_with_two_commits(&repo);
    repo.commit_file("a.txt", "1\nAA\n3\n", "fixup! feat: introduce a");
    assert!(range_lint(&repo, &base).contains("[fixup-unmatched]"));
    repo.git(&["reset", "-q", "--hard", "HEAD~1"]);
    repo.commit_file("c.txt", "c\n", "fixup! HEAD");
    assert!(range_lint(&repo, &base).contains("[fixup-unmatched]"));
    repo.git(&["reset", "-q", "--hard", "HEAD~1"]);
    repo.commit_file("c.txt", "c\n", "fixup! feat: later");
    repo.commit_file("d.txt", "d\n", "feat: later");
    assert!(range_lint(&repo, &base).contains("[fixup-unmatched]"));
}

#[test]
fn lint_range_unmatched_fixup_of_a_published_target_says_to_reword_it() {
    let repo = Repo::new();
    let (_, a) = branch_with_two_commits(&repo);
    repo.commit_file("a.txt", "1\nAA\n3\n", "fixup! feat: add a");
    let err = range_lint(&repo, &a);
    assert!(err.contains(&format!("[fixup-unmatched] `fixup!` commit's target {} feat: add a is already published\n", &a[..10])), "{err}");
    assert!(err.contains(&format!("  try: git rebase -i {}: reword it into a normal commit\n", &a[..10])), "{err}");
    assert!(!err.contains("--no-verify"), "{err}");
}

#[test]
fn lint_range_unmatched_fixup_without_one_blamed_target_gets_generic_advice() {
    let repo = Repo::new();
    let (base, _) = branch_with_two_commits(&repo);
    let generic = format!(
        "  try: git rebase -i {}: move it below the commit it belongs to and change pick to fixup, or reword it into a normal commit\n",
        &base[..10]
    );
    repo.commit_file("new.txt", "new\n", "fixup! wip");
    let err = range_lint(&repo, &base);
    assert!(err.contains("[fixup-unmatched]") && err.contains(&generic), "{err}");
    repo.git(&["reset", "-q", "--hard", "HEAD~1"]);
    repo.commit_file("a.txt", "1\nA\n3\nB\n", "feat: add b line");
    repo.commit_file("a.txt", "1\nAX\n3\nBX\n", "fixup! wip");
    let err = range_lint(&repo, &base);
    assert!(err.contains("[fixup-unmatched]") && err.contains(&generic), "{err}");
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
