mod common;

use common::{Repo, stderr};

fn topic_repo() -> Repo {
    let repo = Repo::new();
    repo.commit_file("base.txt", "base\n", "chore: base");
    repo.git(&["switch", "-q", "-c", "topic"]);
    repo.commit_file("a.txt", "one\ntwo\nthree\n", "feat: add a");
    repo
}

fn stage(repo: &Repo, path: &str, content: &str) {
    repo.write(path, content);
    repo.git(&["add", path]);
}

#[test]
fn accepts_optional_target_and_dry_run_in_either_order() {
    let repo = topic_repo();
    let sha = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    for args in [vec!["fixup", sha.as_str(), "--dry-run"], vec!["fixup", "--dry-run", sha.as_str()], vec!["fixup", "--dry-run"]] {
        let out = repo.gir(&args);
        assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
        assert_eq!(String::from_utf8_lossy(&out.stdout), format!("{} feat: add a\n", &sha[..10]));
    }
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), sha);
    assert_eq!(repo.git(&["diff", "--cached", "--name-only"]), "a.txt");
}

#[test]
fn requires_staged_changes_even_with_explicit_target() {
    let repo = topic_repo();
    let out = repo.gir(&["fixup", "HEAD"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: nothing staged; `git add` the fix first (more: gir explain fixup)\n");
}

#[test]
fn explicit_target_ignores_line_origin_and_accepts_new_files() {
    let repo = topic_repo();
    let target = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("b.txt", "b\n", "feat: add b");
    stage(&repo, "b.txt", "B\n");
    stage(&repo, "new.txt", "new\n");
    let out = repo.gir(&["fixup", &target]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "fixup! feat: add a");
    assert_eq!(repo.git(&["show", "HEAD:b.txt"]), "B");
    assert_eq!(repo.git(&["show", "HEAD:new.txt"]), "new");
}

#[test]
fn explicit_target_must_be_in_current_branch_history() {
    let repo = topic_repo();
    repo.git(&["switch", "-q", "-c", "other", "main"]);
    repo.commit_file("other.txt", "other\n", "feat: other");
    let other = repo.git(&["rev-parse", "HEAD"]);
    repo.git(&["switch", "-q", "topic"]);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let head = repo.git(&["rev-parse", "HEAD"]);
    let out = repo.gir(&["fixup", &other]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), format!("gir: `{other}` is not in the current branch's history\n"));
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), head, "refusal must create no commit");
    assert_eq!(repo.git(&["diff", "--cached", "--name-only"]), "a.txt");
}

#[test]
fn explicit_target_must_be_after_base() {
    let repo = topic_repo();
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let head = repo.git(&["rev-parse", "HEAD"]);
    let out = repo.gir(&["fixup", "main"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: `main` is already on the base branch\n");
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), head, "refusal must create no commit");
    assert_eq!(repo.git(&["diff", "--cached", "--name-only"]), "a.txt");
}

#[test]
fn rejects_invalid_explicit_commit() {
    let repo = topic_repo();
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let out = repo.gir(&["fixup", "no-such-commit"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: `no-such-commit` is not a commit\n");
}

#[test]
fn insertion_uses_adjacent_head_lines() {
    let repo = topic_repo();
    let sha = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\ninserted\ntwo\nthree\n");
    let out = repo.gir(&["fixup", "--dry-run"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(String::from_utf8_lossy(&out.stdout), format!("{} feat: add a\n", &sha[..10]));
}

#[test]
fn base_lookup_prefers_origin_head_over_main_and_limits_automatic_target() {
    let repo = topic_repo();
    let origin_base = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("later.txt", "later\n", "feat: later");
    repo.git(&["update-ref", "refs/remotes/origin/HEAD", &origin_base]);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("already on the base branch"), "{}", stderr(&out));
}

#[test]
fn base_lookup_falls_back_to_master() {
    let repo = topic_repo();
    let master = repo.git(&["rev-parse", "HEAD"]);
    repo.git(&["update-ref", "refs/heads/master", &master]);
    repo.git(&["update-ref", "-d", "refs/heads/main"]);
    repo.commit_file("later.txt", "later\n", "feat: later");
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("already on the base branch"), "{}", stderr(&out));
}

#[test]
fn base_lookup_falls_back_to_upstream() {
    let repo = topic_repo();
    repo.git(&["branch", "upstream", "HEAD"]);
    repo.git(&["branch", "--set-upstream-to=upstream", "topic"]);
    repo.git(&["update-ref", "-d", "refs/heads/main"]);
    repo.commit_file("later.txt", "later\n", "feat: later");
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("already on the base branch"), "{}", stderr(&out));
}

#[test]
fn automatic_target_has_no_base_limit_when_no_base_exists() {
    let repo = Repo::new();
    repo.commit_file("a.txt", "one\ntwo\n", "feat: add a");
    let sha = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\nTWO\n");
    let out = repo.gir(&["fixup", "--dry-run"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(String::from_utf8_lossy(&out.stdout), format!("{} feat: add a\n", &sha[..10]));
}

#[test]
fn automatic_target_refuses_a_new_file() {
    let repo = topic_repo();
    stage(&repo, "new.txt", "new\n");
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: new.txt is a new file, so it has no earlier commit; pass one: gir fixup <commit>\n");
}

#[test]
fn automatic_target_reports_unattributed_lines() {
    let repo = Repo::new();
    repo.commit_file("empty.txt", "", "feat: empty file");
    stage(&repo, "empty.txt", "first line\n");
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("cannot tell which commit"), "{}", stderr(&out));
    assert!(stderr(&out).contains("pass one: gir fixup <commit>"), "{}", stderr(&out));
}

#[test]
fn automatic_target_reports_multiple_targets_and_split_hint() {
    let repo = topic_repo();
    let a = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file("b.txt", "b\n", "feat: add b");
    let b = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    stage(&repo, "b.txt", "B\n");
    let out = repo.gir(&["fixup"]);
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(2));
    assert!(err.contains("staged changes belong to several commits:"), "{err}");
    assert!(err.contains(&format!("{} feat: add a  <- a.txt:2", &a[..10])), "{err}");
    assert!(err.contains(&format!("{} feat: add b  <- b.txt:1", &b[..10])), "{err}");
    assert!(err.contains("split: git restore --staged . && git add -p, then one gir fixup per commit"), "{err}");
}

#[test]
fn multiple_targets_suggest_installed_git_absorb() {
    let repo = topic_repo();
    repo.commit_file("b.txt", "b\n", "feat: add b");
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    stage(&repo, "b.txt", "B\n");
    let bin = repo.dir.join("bin");
    std::fs::create_dir(&bin).unwrap();
    let absorb = bin.join(format!("git-absorb{}", std::env::consts::EXE_SUFFIX));
    std::fs::copy(env!("CARGO_BIN_EXE_gir"), &absorb).unwrap();
    let mut paths = vec![bin];
    paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()));
    let path = std::env::join_paths(paths).unwrap();
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).args(["fixup"]).env("PATH", path).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("or: git absorb (installed) creates one fixup per commit"), "{}", stderr(&out));
}

#[test]
fn successful_fixup_prints_target_and_base_and_preserves_config() {
    let repo = topic_repo();
    let base = repo.git(&["rev-parse", "main"]);
    let target = repo.git(&["rev-parse", "HEAD"]);
    let config_before = std::fs::read(&repo.global).unwrap();
    let local_before = std::fs::read(repo.dir.join(".git/config")).unwrap();
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(stderr(&out), format!("gir: created fixup! for {} feat: add a\n  fold: git rebase --autosquash {}\n", &target[..10], &base[..10]));
    assert_eq!(repo.git(&["log", "-1", "--format=%s"]), "fixup! feat: add a");
    assert_eq!(std::fs::read(&repo.global).unwrap(), config_before);
    assert_eq!(std::fs::read(repo.dir.join(".git/config")).unwrap(), local_before);
}

#[test]
fn success_without_base_prints_base_placeholder() {
    let repo = Repo::new();
    repo.commit_file("a.txt", "one\ntwo\n", "feat: add a");
    stage(&repo, "a.txt", "one\nTWO\n");
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(stderr(&out).contains("  fold: git rebase --autosquash <base>\n"));
}

#[test]
fn commit_failure_reports_git_command_and_target() {
    let repo = topic_repo();
    let target = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "a.txt", "one\nTWO\nthree\n");
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).args(["fixup"]).env("GIT_COMMITTER_DATE", "not-a-date").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let err = stderr(&out);
    assert_eq!(err.lines().last(), Some(format!("gir: git commit --quiet --fixup={target} failed").as_str()), "{err}");
    assert!(err.lines().count() > 1, "git's own output must precede gir's message: {err}");
    assert_eq!(repo.git(&["rev-parse", "HEAD"]), target);
}
