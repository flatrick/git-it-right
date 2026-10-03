mod common;

use common::{Repo, stderr};

#[test]
fn commit_msg_hook_fixes_and_rejects_through_real_git_commit() {
    let repo = Repo::new();
    assert!(repo.gir(&["init"]).status.success());
    assert_eq!(repo.git(&["config", "core.hooksPath"]), ".githooks");

    repo.write("a.txt", "a\n");
    repo.git(&["add", "a.txt"]);
    let ok = repo.git_out(&["commit", "-q", "-m", "Feature(api) :Add a.", "-m", "breaking change: renamed"]);
    assert!(ok.status.success(), "{}", stderr(&ok));
    assert_eq!(repo.head_message(), "feat(api): Add a\n\nBREAKING CHANGE: renamed");
    let err = stderr(&ok);
    assert!(err.contains("gir: fixed [type-alias]"), "{err}");

    repo.write("b.txt", "b\n");
    repo.git(&["add", "b.txt"]);
    let bad = repo.git_out(&["commit", "-q", "-m", "update stuff"]);
    assert!(!bad.status.success());
    let err = stderr(&bad);
    assert!(err.contains("gir: commit rejected [type-missing]"), "{err}");
    assert!(err.contains("  try: <type>: update stuff"), "{err}");
    assert!(err.contains("  more: gir explain type-missing"), "{err}");
    assert!(err.lines().filter(|l| l.starts_with("gir:") || l.starts_with("  ")).count() <= 3, "{err}");
}

#[test]
fn commit_msg_hook_keeps_comments_and_verbose_diff_out_of_the_way() {
    let repo = Repo::new();
    let raw = "Feat: x\nbody\n# Please enter the commit message\n# ------------------------ >8 ------------------------\ndiff --git a/x b/x\n+feat: not a header\n";
    repo.write("MSG", raw);
    let out = repo.gir(&["hook", "commit-msg", "MSG"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let fixed = std::fs::read_to_string(repo.dir.join("MSG")).unwrap();
    assert!(fixed.starts_with("feat: x\n\nbody\n\n# Please enter"), "{fixed}");
    assert!(fixed.ends_with("diff --git a/x b/x\n+feat: not a header\n"), "{fixed}");
}

#[test]
fn pre_push_rejects_unsquashed_fixups_and_no_verify_commits() {
    let repo = Repo::new();
    let remote = repo.dir.parent().unwrap().join("remote.git");
    repo.git(&["init", "-q", "--bare", remote.to_str().unwrap()]);
    repo.git(&["remote", "add", "origin", remote.to_str().unwrap()]);
    assert!(repo.gir(&["init"]).status.success());
    repo.git(&["commit", "-q", "-m", "chore: add gir setup"]);
    repo.git(&["push", "-q", "origin", "main"]);

    repo.commit_file("a.txt", "a\n", "feat: add a");
    repo.commit_file("a.txt", "a2\n", "fixup! feat: add a");
    let push = repo.git_out(&["push", "-q", "origin", "main"]);
    assert!(!push.status.success());
    assert!(stderr(&push).contains("[fixup-unsquashed]"), "{}", stderr(&push));

    repo.git(&["reset", "-q", "--hard", "HEAD~1"]);
    repo.commit_file("b.txt", "b\n", "wip");
    let push = repo.git_out(&["push", "-q", "origin", "main"]);
    assert!(!push.status.success());
    assert!(stderr(&push).contains("push rejected [type-missing]"), "{}", stderr(&push));
}

#[test]
fn pre_push_rejects_a_fixup_of_a_pushed_commit_and_names_no_verify() {
    let repo = Repo::new();
    let remote = repo.dir.parent().unwrap().join("remote.git");
    repo.git(&["init", "-q", "--bare", remote.to_str().unwrap()]);
    repo.git(&["remote", "add", "origin", remote.to_str().unwrap()]);
    assert!(repo.gir(&["init"]).status.success());
    repo.git(&["commit", "-q", "-m", "chore: add gir setup"]);
    repo.commit_file("a.txt", "1\nA\n3\n", "feat: add a");
    repo.git(&["push", "-q", "origin", "main"]);
    let pushed = repo.git(&["rev-parse", "HEAD"]);

    repo.commit_file("a.txt", "1\nAA\n3\n", "fixup! feat: add a");
    let push = repo.git_out(&["push", "-q", "origin", "main"]);
    assert!(!push.status.success());
    let err = stderr(&push);
    assert!(
        err.contains(&format!(
            "push rejected [fixup-unmatched] `fixup!` commit's target {} feat: add a is already published",
            &pushed[..10]
        )),
        "{err}"
    );
    assert!(
        err.contains(&format!(
            "  try: git rebase -i {}: reword it into a normal commit, or push it as is: git push --no-verify",
            &pushed[..10]
        )),
        "{err}"
    );
}

#[test]
fn fixup_finds_the_target_from_staged_lines_and_autosquash_folds_it() {
    let repo = Repo::new();
    repo.commit_file("base.txt", "base\n", "chore: base");
    repo.git(&["switch", "-q", "-c", "topic"]);
    repo.commit_file("a.txt", "one\ntwo\nthree\n", "feat: add a");
    repo.commit_file("b.txt", "b\n", "feat: add b");

    repo.write("a.txt", "one\nTWO\nthree\n");
    repo.git(&["add", "a.txt"]);
    let out = repo.gir(&["fixup"]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "fixup! feat: add a");

    repo.git(&["-c", "sequence.editor=:", "rebase", "-q", "--autosquash", "main"]);
    assert_eq!(repo.git(&["log", "--format=%s", "main..topic"]), "feat: add b\nfeat: add a");
    assert_eq!(repo.git(&["show", "HEAD~1:a.txt"]), "one\nTWO\nthree");
}

/// Checks every row of the fixup table in CHEATSHEET.md against real git.
#[test]
fn cheatsheet_fixup_table_matches_git() {
    let repo = Repo::new();
    repo.commit_file("base.txt", "base\n", "chore: base");
    repo.git(&["switch", "-q", "-c", "topic"]);
    for name in ["a", "b", "c", "d"] {
        repo.commit_file(&format!("{name}.txt"), "1\n", &format!("feat: add {name}"));
    }
    let sha = |subject: &str| repo.git(&["log", "--format=%H", "--grep", &format!("^{subject}$"), "topic"]);
    let (a, b, c, d) = (sha("feat: add a"), sha("feat: add b"), sha("feat: add c"), sha("feat: add d"));

    let log = repo.dir.parent().unwrap().join("editor.log");
    let log_path = log.to_string_lossy().replace('\\', "/");
    let editor = r#"f() { echo called >> "$EDLOG"; if [ -n "$NEWMSG" ]; then printf '%s\n' "$NEWMSG" > "$1"; fi; }; f"#;
    let with_msg = |msg: &'static str| [("GIT_EDITOR", editor), ("EDLOG", log_path.as_str()), ("NEWMSG", msg)];

    // Row 1, add forgotten changes and keep the message: fixup!
    repo.write("a.txt", "2\n");
    repo.git(&["add", "a.txt"]);
    repo.git(&["commit", "-q", &format!("--fixup={a}")]);
    // Row 2, add changes and write a new message: amend!
    repo.write("b.txt", "2\n");
    repo.git(&["add", "b.txt"]);
    repo.git_with(&["commit", "-q", &format!("--fixup=amend:{b}")], &with_msg("amend! feat: add b\n\nfeat: add b with retries"));
    // Row 3, only change the message: reword
    repo.git_with(&["commit", "-q", &format!("--fixup=reword:{c}")], &with_msg("amend! feat: add c\n\nfeat: add c, reworded"));
    // Row 4, add changes and merge both messages: squash!
    repo.write("d.txt", "2\n");
    repo.git(&["add", "d.txt"]);
    repo.git(&["commit", "-q", &format!("--squash={d}"), "-m", "extra detail"]);

    let _ = std::fs::remove_file(&log);
    repo.git_with(&["rebase", "-q", "--autosquash", "main"], &with_msg("feat: add d\n\nextra detail"));

    assert_eq!(
        repo.git(&["log", "--format=%s", "main..topic"]),
        "feat: add d\nfeat: add c, reworded\nfeat: add b with retries\nfeat: add a",
        "fixup keeps, amend and reword replace, squash takes the edited message"
    );
    for (file, want) in [("a.txt", "2"), ("b.txt", "2"), ("c.txt", "1"), ("d.txt", "2")] {
        assert_eq!(repo.git(&["show", &format!("HEAD:{file}")]), want, "{file}");
    }
    let calls = std::fs::read_to_string(&log).unwrap_or_default();
    assert_eq!(calls.lines().count(), 1, "the rebase should open the editor only for squash!, got:\n{calls}");
}

#[test]
fn fixup_refuses_ambiguous_and_base_branch_targets() {
    let repo = Repo::new();
    repo.commit_file("base.txt", "base\n", "chore: base");
    repo.git(&["switch", "-q", "-c", "topic"]);
    repo.commit_file("a.txt", "a\n", "feat: add a");
    repo.commit_file("b.txt", "b\n", "feat: add b");

    repo.write("a.txt", "A\n");
    repo.write("b.txt", "B\n");
    repo.git(&["add", "a.txt", "b.txt"]);
    let out = repo.gir(&["fixup"]);
    assert!(!out.status.success());
    assert!(stderr(&out).contains("several commits"), "{}", stderr(&out));

    repo.git(&["reset", "-q", "--hard"]);
    repo.write("base.txt", "BASE\n");
    repo.git(&["add", "base.txt"]);
    let out = repo.gir(&["fixup"]);
    assert!(!out.status.success());
    assert!(stderr(&out).contains("already on the base branch"), "{}", stderr(&out));
}

#[test]
fn lint_range_reports_json_per_commit() {
    let repo = Repo::new();
    repo.commit_file("a.txt", "a\n", "chore: base");
    repo.commit_file("b.txt", "b\n", "Added b");
    let out = repo.gir(&["lint", "--range", "HEAD~1..HEAD", "--json"]);
    assert_eq!(out.status.code(), Some(1));
    let json = String::from_utf8_lossy(&out.stdout);
    assert!(json.starts_with("[{\"commit\":\""), "{json}");
    assert!(json.contains("\"rule\":\"type-missing\""), "{json}");
}

#[test]
fn doctor_fix_converges() {
    let repo = Repo::new();
    repo.write("Cargo.toml", "[package]\n");
    repo.commit_file("run.sh", "#!/bin/sh\n", "chore: base");
    let first = repo.gir(&["doctor"]);
    assert_eq!(first.status.code(), Some(1));
    let report = String::from_utf8_lossy(&first.stdout).to_string();
    for id in [".gitattributes", ".gitignore: does not ignore: /target/", "exec-bit", "rebase.autoSquash"] {
        assert!(report.contains(id), "{id} missing from:\n{report}");
    }

    assert!(repo.gir(&["init"]).status.success());
    repo.gir(&["doctor", "--fix"]);
    let second = repo.gir(&["doctor"]);
    let report = String::from_utf8_lossy(&second.stdout).to_string();
    assert!(!report.lines().any(|l| l.starts_with("warn ")), "{report}");
    assert_eq!(second.status.code(), Some(0), "{report}");
    assert!(repo.dir.join(".gitattributes").exists());
    assert_eq!(repo.git(&["config", "--local", "rebase.autoSquash"]), "true");
}

#[test]
fn init_is_idempotent_and_keeps_user_edits() {
    let repo = Repo::new();
    assert!(repo.gir(&["init"]).status.success());
    let again = repo.gir(&["init"]);
    assert!(again.status.success());
    assert!(!stderr(&again).contains("wrote"), "{}", stderr(&again));

    repo.write(".girconfig", "[gir]\n\ttypes = feat fix\n");
    let kept = repo.gir(&["init"]);
    assert_eq!(kept.status.code(), Some(1));
    assert!(stderr(&kept).contains("kept .girconfig"));
}

#[test]
fn explain_every_listed_topic() {
    let repo = Repo::new();
    let list = String::from_utf8_lossy(&repo.gir(&["explain"]).stdout).to_string();
    let topics: Vec<&str> = list.trim().trim_start_matches("topics: ").split(' ').collect();
    assert!(topics.len() > 15);
    for t in topics {
        let out = repo.gir(&["explain", t]);
        assert!(out.status.success() && !out.stdout.is_empty(), "{t}");
    }
}
