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
    assert_eq!(
        stderr(&out),
        format!("gir: created fixup! for {} feat: add a\n  fold: git rebase --autosquash {}\n", &target[..10], &base[..10])
    );
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

#[test]
fn automatic_target_refuses_a_staged_file_without_line_changes() {
    let repo = topic_repo();
    repo.write("b.bin", "\0one");
    repo.git(&["add", "b.bin"]);
    repo.git(&["commit", "-q", "--no-verify", "-m", "feat: add b.bin"]);
    stage(&repo, "b.bin", "\0two");
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert_eq!(stderr(&out), "gir: cannot tell which commit b.bin belongs to; pass one: gir fixup <commit>\n");
}

#[test]
fn mixed_base_and_topic_replacement_is_refused() {
    let repo = Repo::new();
    repo.commit_file("lines.txt", "base\n", "chore: base");
    let base = repo.git(&["rev-parse", "HEAD"]);
    repo.git(&["switch", "-q", "-c", "topic"]);
    repo.commit_file("lines.txt", "base\ntopic\n", "feat: topic line");
    stage(&repo, "lines.txt", "BASE\nTOPIC\n");
    let out = repo.gir(&["fixup", "--dry-run"]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert_eq!(
        stderr(&out),
        format!("gir: lines.txt:1 was last changed by {} which is already on the base branch; commit it normally instead\n", &base[..10])
    );
    assert_eq!(repo.git(&["diff", "--cached", "--name-only"]), "lines.txt");
}

#[test]
fn automatic_target_refuses_a_new_binary_file() {
    let repo = topic_repo();
    std::fs::write(repo.dir.join("new.bin"), b"one\0two").unwrap();
    repo.git(&["add", "new.bin"]);
    let out = repo.gir(&["fixup"]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert_eq!(stderr(&out), "gir: new.bin is a new file, so it has no earlier commit; pass one: gir fixup <commit>\n");
    assert_eq!(repo.git(&["diff", "--cached", "--name-only"]), "new.bin");
}

#[test]
fn insertion_between_base_and_topic_lines_targets_topic() {
    let repo = Repo::new();
    repo.commit_file("lines.txt", "base\n", "chore: base");
    repo.git(&["switch", "-q", "-c", "topic"]);
    repo.commit_file("lines.txt", "base\ntopic\n", "feat: topic line");
    let topic = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "lines.txt", "base\ninserted\ntopic\n");
    let out = repo.gir(&["fixup", "--dry-run"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(String::from_utf8(out.stdout).unwrap(), format!("{} feat: topic line\n", &topic[..10]));
}

#[test]
fn deleted_line_starting_with_dashes_is_traced_like_any_other() {
    let repo = topic_repo();
    repo.commit_file("x.lua", "-- header\nlocal a = 1\nlocal b = 2\nlocal c = 3\nlocal d = 4\n", "feat: lua");
    let lua = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "x.lua", "local a = 1\nlocal b = 2\nlocal c = 3\nlocal d = 5\n");
    let out = repo.gir(&["fixup", "--dry-run"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(String::from_utf8(out.stdout).unwrap(), format!("{} feat: lua\n", &lua[..10]));
}

#[test]
fn deleted_dev_null_comment_does_not_hide_later_hunks() {
    let repo = topic_repo();
    repo.commit_file("x.lua", "-- /dev/null\nl1\nl2\nl3\nl4\n", "feat: lua a");
    repo.commit_file("x.lua", "-- /dev/null\nl1\nl2\nl3\nL4\n", "feat: lua b");
    stage(&repo, "x.lua", "l1\nl2\nl3\nL4x\n");
    let out = repo.gir(&["fixup", "--dry-run"]);
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(2), "{err}");
    assert!(err.contains("staged changes belong to several commits:"), "{err}");
    assert!(err.contains("feat: lua a  <- x.lua:1"), "{err}");
    assert!(err.contains("feat: lua b  <- x.lua:5"), "{err}");
}

#[test]
fn added_line_starting_with_plus_b_is_not_a_new_file() {
    let repo = topic_repo();
    repo.commit_file("z.lua", "-- /dev/null\nm1\nm2\nm3\nm4\n", "feat: z");
    let z = repo.git(&["rev-parse", "HEAD"]);
    stage(&repo, "z.lua", "m1\nm2\nm3\n++ b/foo\nm4\n");
    let out = repo.gir(&["fixup", "--dry-run"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert_eq!(String::from_utf8(out.stdout).unwrap(), format!("{} feat: z\n", &z[..10]));
}

#[test]
fn dev_null_comment_split_folds_into_the_right_commits() {
    let repo = topic_repo();
    repo.commit_file("x.lua", "-- /dev/null\nl1\nl2\nl3\nl4\n", "feat: lua a");
    repo.commit_file("x.lua", "-- /dev/null\nl1\nl2\nl3\nL4\n", "feat: lua b");
    stage(&repo, "x.lua", "l1\nl2\nl3\nL4x\n");
    let out = repo.gir(&["fixup", "--split"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    repo.git_with(&["rebase", "-q", "-i", "--autosquash", "main"], &[("GIT_SEQUENCE_EDITOR", "true")]);
    assert_eq!(repo.git(&["log", "--format=%s", "main..HEAD"]), "feat: lua b\nfeat: lua a\nfeat: add a");
    assert_eq!(repo.git(&["show", "HEAD~1:x.lua"]), "l1\nl2\nl3\nl4");
    assert_eq!(repo.git(&["show", "HEAD:x.lua"]), "l1\nl2\nl3\nL4x");
}

/// Two commits change `name`; a fixup for the first line goes to the first, and `--split`
/// followed by `rebase --autosquash` puts each change in its own commit.
fn trace_and_split_two_commits_in(name: &str) {
    let repo = topic_repo();
    repo.commit_file(name, "one\ntwo\nthree\nfour\n", "feat: first");
    let first = repo.git(&["rev-parse", "HEAD"]);
    repo.commit_file(name, "one\ntwo\nthree\nFOUR\n", "feat: second");
    stage(&repo, name, "ONE\ntwo\nthree\nFOUR\n");
    let out = repo.gir(&["fixup", "--dry-run"]);
    assert_eq!(out.status.code(), Some(0), "{name:?}: {}", stderr(&out));
    assert_eq!(String::from_utf8(out.stdout).unwrap(), format!("{} feat: first\n", &first[..10]), "{name:?}");
    stage(&repo, name, "ONE\ntwo\nthree\nFOURx\n");
    let out = repo.gir(&["fixup", "--split"]);
    assert_eq!(out.status.code(), Some(0), "{name:?}: {}", stderr(&out));
    repo.git_with(&["rebase", "-q", "-i", "--autosquash", "main"], &[("GIT_SEQUENCE_EDITOR", "true")]);
    assert_eq!(repo.git(&["log", "--format=%s", "main..HEAD"]), "feat: second\nfeat: first\nfeat: add a", "{name:?}");
    assert_eq!(repo.git(&["show", &format!("HEAD~1:{name}")]), "ONE\ntwo\nthree\nfour", "{name:?}");
    assert_eq!(repo.git(&["show", &format!("HEAD:{name}")]), "ONE\ntwo\nthree\nFOURx", "{name:?}");
}

#[test]
fn file_name_with_a_space_is_traced_and_split() {
    trace_and_split_two_commits_in("my file.txt");
}

// Windows file names cannot contain a double quote, a backslash or a tab, which are the
// characters that make git quote a name in diff headers.
#[cfg(unix)]
#[test]
fn quoted_file_names_are_traced_and_split() {
    for name in ["say \"hi\".txt", "back\\slash.txt", "tab\there.txt", "space and \"quote\".txt"] {
        trace_and_split_two_commits_in(name);
    }
}
