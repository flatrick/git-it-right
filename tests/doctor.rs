mod common;

use common::{Repo, stderr};
use std::process::Output;

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn local_value(repo: &Repo, key: &str) -> Option<String> {
    let out = repo.git_out(&["config", "--local", "--get", key]);
    out.status.success().then(|| stdout(&out).trim().to_string())
}

#[test]
fn doctor_outside_git_repository_reports_error() {
    let repo = Repo::new();
    let outside = repo.dir.parent().unwrap();
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(outside).args(["doctor"]).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: not inside a git repository\n");
}

#[test]
fn doctor_reports_check_lines_and_summary_counts() {
    let repo = Repo::new();
    let out = repo.gir(&["doctor"]);
    let report = stdout(&out);
    let lines: Vec<&str> = report.lines().collect();
    let checks = &lines[..lines.len() - 1];
    assert!(checks.iter().all(|line| {
        let Some((status, rest)) = line.split_once(' ') else { return false };
        matches!(status, "warn" | "info") && rest.starts_with(' ') && rest.trim_start().split_once(": ").is_some()
    }), "check lines must carry a status, ID, and message: {report}");
    let warnings = checks.iter().filter(|line| line.starts_with("warn ")).count();
    let summary = lines.last().unwrap();
    assert_eq!(*summary, format!("gir doctor: 1 ok, {warnings} warnings, {} fixable with: gir doctor --fix   more: gir explain doctor", if cfg!(windows) { 13 } else { 12 }));
    let fixed = repo.gir(&["doctor", "--fix"]);
    let fixed_report = stdout(&fixed);
    assert!(fixed_report.lines().any(|line| line.starts_with("fixed .gitattributes: ")), "{fixed_report}");
    assert!(fixed_report.lines().last().unwrap().ends_with("   more: gir explain doctor"), "{fixed_report}");
    assert!(!fixed_report.contains("fixable with"), "{fixed_report}");
}

#[test]
fn doctor_info_checks_do_not_cause_failure() {
    let repo = Repo::new();
    assert!(repo.gir(&["init"]).status.success());
    assert!(repo.gir(&["doctor", "--fix"]).status.success());
    repo.write(".gitattributes", "# custom\n");
    let out = repo.gir(&["doctor"]);
    let report = stdout(&out);
    assert!(report.contains("info  .gitattributes: no `* text=auto` line"), "{report}");
    assert_eq!(out.status.code(), Some(0), "{report}");
}

#[test]
fn doctor_missing_hooks_are_not_created_by_fix() {
    let repo = Repo::new();
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.lines().any(|line| line == "warn  hooks: no .githooks/; run: gir init"), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.lines().any(|line| line == "warn  hooks: no .githooks/; run: gir init"), "{fixed}");
    assert!(!repo.dir.join(".githooks").exists());
}

#[test]
fn doctor_repairs_inactive_hooks_path_locally() {
    let repo = Repo::new();
    std::fs::create_dir(repo.dir.join(".githooks")).unwrap();
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("warn  core.hooksPath: hooks in .githooks/ are not active in this clone"), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.contains("fixed core.hooksPath: hooks in .githooks/ are not active in this clone"), "{fixed}");
    assert_eq!(local_value(&repo, "core.hooksPath").as_deref(), Some(".githooks"));
}

#[test]
fn doctor_recommends_and_sets_each_unset_git_setting() {
    let repo = Repo::new();
    let settings = [
        ("pull.ff", "only"), ("fetch.prune", "true"), ("push.autoSetupRemote", "true"),
        ("rerere.enabled", "true"), ("merge.conflictStyle", "zdiff3"),
        ("diff.algorithm", "histogram"), ("rebase.autoStash", "true"), ("rebase.updateRefs", "true"),
    ];
    let report = stdout(&repo.gir(&["doctor"]));
    for (key, value) in settings {
        assert!(report.lines().any(|line| line.starts_with(&format!("warn  {key}: unset; `{value}` recommended"))), "missing {key}: {report}");
    }
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    for (key, value) in settings {
        assert!(fixed.lines().any(|line| line.starts_with(&format!("fixed {key}: unset; `{value}` recommended"))), "not fixed {key}: {fixed}");
        assert_eq!(local_value(&repo, key).as_deref(), Some(value), "{key}");
    }
}

#[test]
fn doctor_preserves_nonempty_existing_recommendations() {
    let repo = Repo::new();
    repo.git(&["config", "--local", "fetch.prune", "false"]);
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("info  fetch.prune: is `false`; `true` recommended"), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.contains("info  fetch.prune: is `false`; `true` recommended"), "{fixed}");
    assert_eq!(local_value(&repo, "fetch.prune").as_deref(), Some("false"));
}

#[test]
fn doctor_respects_pull_rebase_without_setting_pull_ff() {
    let repo = Repo::new();
    repo.git(&["config", "--local", "pull.rebase", "true"]);
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(!report.lines().any(|line| line.contains("pull.ff:")), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(!fixed.lines().any(|line| line.contains("pull.ff:")), "{fixed}");
    assert_eq!(local_value(&repo, "pull.ff"), None);
}

#[cfg(windows)]
#[test]
fn doctor_repairs_unset_windows_longpaths() {
    let repo = Repo::new();
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("warn  core.longpaths: unset; paths over 260 chars fail on Windows"), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.contains("fixed core.longpaths: unset; paths over 260 chars fail on Windows"), "{fixed}");
    assert_eq!(local_value(&repo, "core.longpaths").as_deref(), Some("true"));
}

#[test]
fn doctor_suggests_missing_identity_and_default_branch_without_setting_them() {
    let repo = Repo::new();
    std::fs::write(&repo.global, "").unwrap();
    let report = stdout(&repo.gir(&["doctor"]));
    for key in ["user.name", "user.email"] {
        assert!(report.lines().any(|line| line.starts_with(&format!("warn  {key}: unset; run: git config --global {key} <value>"))), "{key}: {report}");
    }
    assert!(report.contains("info  init.defaultBranch: unset; suggest: git config --global init.defaultBranch main"), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    for key in ["user.name", "user.email"] {
        assert!(fixed.lines().any(|line| line.starts_with(&format!("warn  {key}: unset; run: git config --global {key} <value>"))), "{key}: {fixed}");
        assert_eq!(local_value(&repo, key), None, "{key} must remain unset locally");
    }
    assert!(fixed.contains("info  init.defaultBranch: unset; suggest: git config --global init.defaultBranch main"), "{fixed}");
    assert_eq!(local_value(&repo, "init.defaultBranch"), None);
}

#[test]
fn doctor_writes_complete_gitattributes_and_editorconfig() {
    let repo = Repo::new();
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.contains("fixed .gitattributes: missing"), "{fixed}");
    assert!(fixed.contains("fixed .editorconfig: missing"), "{fixed}");
    let attrs = std::fs::read_to_string(repo.dir.join(".gitattributes")).unwrap();
    assert!(attrs.lines().any(|line| line == "* text=auto eol=lf"), "{attrs}");
    for ext in ["cmd", "bat", "sln"] {
        assert!(attrs.lines().any(|line| line == format!("*.{ext} text eol=crlf")), "{ext}: {attrs}");
    }
    for ext in ["png", "jpg", "jpeg", "gif", "ico", "webp", "pdf", "zip", "gz", "7z", "woff", "woff2", "ttf", "exe", "dll", "so", "dylib"] {
        assert!(attrs.lines().any(|line| line == format!("*.{ext} binary")), "{ext}: {attrs}");
    }
    let editor = std::fs::read_to_string(repo.dir.join(".editorconfig")).unwrap();
    assert!(editor.lines().any(|line| line == "root = true"), "{editor}");
    for section in [
        "[*]\ncharset = utf-8\nend_of_line = lf\ninsert_final_newline = true\ntrim_trailing_whitespace = true\nindent_style = space\nindent_size = 4\n",
        "[*.{md,markdown}]\ntrim_trailing_whitespace = false\n",
        "[*.{yml,yaml,json,toml}]\nindent_size = 2\n",
        "[{*.cmd,*.bat,*.sln}]\nend_of_line = crlf\n",
        "[Makefile]\nindent_style = tab\n",
    ] {
        assert!(editor.contains(section), "{section}: {editor}");
    }
}

#[test]
fn doctor_reports_autocrlf_when_gitattributes_is_missing() {
    let repo = Repo::new();
    repo.git(&["config", "--local", "core.autocrlf", "true"]);
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("warn  .gitattributes: missing and core.autocrlf=true, so line endings depend on each clone"), "{report}");
}

#[test]
fn doctor_does_not_replace_readable_gitattributes_without_auto_rule() {
    let repo = Repo::new();
    repo.write(".gitattributes", "# keep this\n*.txt text\n");
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("info  .gitattributes: no `* text=auto` line"), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.contains("info  .gitattributes: no `* text=auto` line"), "{fixed}");
    assert_eq!(std::fs::read_to_string(repo.dir.join(".gitattributes")).unwrap(), "# keep this\n*.txt text\n");
}

#[test]
fn doctor_reports_invalid_girconfig_without_editing_it() {
    let repo = Repo::new();
    repo.write(".girconfig", "[gir]\nsubjectMax = many\n");
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.lines().any(|line| line.starts_with("warn  .girconfig: ") && line.contains("gir.subjectmax") && line.contains("many")), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.lines().any(|line| line.starts_with("warn  .girconfig: ") && line.contains("gir.subjectmax")), "{fixed}");
    assert_eq!(std::fs::read_to_string(repo.dir.join(".girconfig")).unwrap(), "[gir]\nsubjectMax = many\n");
}

#[test]
fn doctor_appends_missing_ignore_rules_after_newline() {
    let repo = Repo::new();
    repo.write("Cargo.toml", "[package]\n");
    repo.write(".gitignore", "existing-rule");
    let report = stdout(&repo.gir(&["doctor"]));
    for pattern in ["/target/", ".DS_Store", "Thumbs.db", ".env"] {
        assert!(report.contains(pattern), "{pattern}: {report}");
    }
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.contains("fixed .gitignore: does not ignore:"), "{fixed}");
    let ignored = std::fs::read_to_string(repo.dir.join(".gitignore")).unwrap();
    assert!(ignored.starts_with("existing-rule\n/target/\n"), "{ignored}");
    for pattern in [".DS_Store", "Thumbs.db", ".env"] {
        assert!(ignored.lines().any(|line| line == pattern), "{pattern}: {ignored}");
    }
}

#[test]
fn doctor_checks_stack_and_local_directory_ignore_patterns_when_present() {
    let repo = Repo::new();
    repo.write("package.json", "{}\n");
    std::fs::create_dir(repo.dir.join(".claude")).unwrap();
    std::fs::create_dir(repo.dir.join(".scratch")).unwrap();
    let report = stdout(&repo.cmd(env!("CARGO_BIN_EXE_gir")).arg("doctor").output().unwrap());
    assert!(report.contains("node_modules/"), "{report}");
    assert!(report.contains(".claude/settings.local.json"), "{report}");
    assert!(report.contains(".scratch/"), "{report}");
    assert!(!report.contains("/target/"), "{report}");
    let fixed = stdout(&repo.cmd(env!("CARGO_BIN_EXE_gir")).args(["doctor", "--fix"]).output().unwrap());
    assert!(fixed.contains("fixed .gitignore: does not ignore:"), "{fixed}");
    let ignored = std::fs::read_to_string(repo.dir.join(".gitignore")).unwrap();
    for pattern in ["node_modules/", ".claude/settings.local.json", ".scratch/"] {
        assert!(ignored.lines().any(|line| line == pattern), "{pattern}: {ignored}");
    }
    assert!(!ignored.lines().any(|line| line == "/target/"), "{ignored}");
}

#[test]
fn doctor_does_not_warn_for_exact_tracked_ignore_probe() {
    let repo = Repo::new();
    repo.write(".env", "tracked\n");
    repo.git(&["add", ".env"]);
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(!report.contains(".gitignore: does not ignore: .env"), "{report}");
    assert!(!report.lines().any(|line| line.starts_with("warn  .gitignore:") && line.split_whitespace().any(|part| part == ".env")), "{report}");
}

#[test]
fn doctor_reports_missing_git_cliff_on_path() {
    let repo = Repo::new();
    let cliff = if cfg!(windows) { "git-cliff.exe" } else { "git-cliff" };
    let dirs = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()).filter(|dir| !dir.join(cliff).exists()).collect::<Vec<_>>();
    let out = repo.cmd(env!("CARGO_BIN_EXE_gir")).env("PATH", std::env::join_paths(dirs).unwrap()).arg("doctor").output().unwrap();
    let report = stdout(&out);
    assert!(report.contains("info  git-cliff: not installed (optional: changelog + next version from commits)"), "{report}");
}

#[test]
fn doctor_reports_case_collisions_without_renaming_index_entries() {
    let repo = Repo::new();
    repo.write("safe.txt", "content\n");
    let blob = repo.git(&["hash-object", "-w", "safe.txt"]);
    for name in ["Name.txt", "name.txt"] {
        repo.git(&["update-index", "--add", "--cacheinfo", &format!("100644,{blob},{name}")]);
    }
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("warn  case-collision: paths differ only in case"), "{report}");
    assert!(report.contains("Name.txt = name.txt"), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.contains("warn  case-collision: paths differ only in case"), "{fixed}");
    assert_eq!(repo.git(&["ls-files"]), "Name.txt\nname.txt");
}

#[test]
fn doctor_reports_windows_unsafe_index_names_without_renaming() {
    let repo = Repo::new();
    repo.write("safe.txt", "content\n");
    let blob = repo.git(&["hash-object", "-w", "safe.txt"]);
    for name in ["CON.txt", "bad?.txt"] {
        repo.git(&["-c", "core.protectNTFS=false", "update-index", "--add", "--cacheinfo", &format!("100644,{blob},{name}")]);
    }
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("warn  windows-names: cannot be checked out on Windows: CON.txt bad?.txt"), "{report}");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(fixed.contains("warn  windows-names: cannot be checked out on Windows: CON.txt bad?.txt"), "{fixed}");
    assert_eq!(repo.git(&["ls-files"]), "CON.txt\nbad?.txt");
}

#[test]
fn doctor_counts_clean_index_as_ok_without_index_warnings() {
    let repo = Repo::new();
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(!report.contains("case-collision:"), "{report}");
    assert!(!report.contains("windows-names:"), "{report}");
    assert!(!report.contains("exec-bit:"), "{report}");
    assert!(report.lines().last().unwrap().starts_with("gir doctor: 1 ok, "), "{report}");
}

#[test]
fn doctor_checks_each_stack_pattern_for_its_marker() {
    for (marker, patterns) in [
        ("App.csproj", &["bin/", "obj/"][..]),
        ("App.sln", &["bin/", "obj/"][..]),
        ("pyproject.toml", &["__pycache__/", ".venv/"][..]),
        ("requirements.txt", &["__pycache__/", ".venv/"][..]),
        ("go.mod", &["/vendor/"][..]),
    ] {
        let repo = Repo::new();
        repo.write(marker, "marker\n");
        let report = stdout(&repo.cmd(env!("CARGO_BIN_EXE_gir")).arg("doctor").output().unwrap());
        for pattern in patterns {
            assert!(report.contains(pattern), "{marker} should require {pattern}: {report}");
        }
        assert!(!report.contains("/target/"), "{marker} should not require Cargo ignores: {report}");
    }
}

#[test]
fn doctor_does_not_report_the_stages_of_an_unmerged_path_as_a_case_collision() {
    let repo = Repo::new();
    repo.commit_file("c.txt", "base\n", "chore: base");
    repo.git(&["switch", "-q", "-c", "other"]);
    repo.commit_file("c.txt", "other\n", "feat: other");
    repo.git(&["switch", "-q", "main"]);
    repo.commit_file("c.txt", "main\n", "feat: main");
    assert!(!repo.git_out(&["merge", "-q", "other"]).status.success(), "the merge must conflict");
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(!report.contains("case-collision"), "{report}");
}

#[test]
fn doctor_appends_ignore_rules_without_losing_invalid_utf8_bytes() {
    let repo = Repo::new();
    let original = b"custom-\xff-rule";
    std::fs::write(repo.dir.join(".gitignore"), original).unwrap();
    let out = repo.gir(&["doctor", "--fix"]);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    assert_eq!(std::fs::read(repo.dir.join(".gitignore")).unwrap(), b"custom-\xff-rule\n.DS_Store\nThumbs.db\n.env\n");
}

#[test]
fn doctor_reports_invalid_utf8_gitattributes_without_replacing_it() {
    let repo = Repo::new();
    let original = b"# keep \xff\n*.txt text\n";
    std::fs::write(repo.dir.join(".gitattributes"), original).unwrap();
    let out = repo.gir(&["doctor", "--fix"]);
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    assert!(stdout(&out).lines().any(|line| line == "info  .gitattributes: no `* text=auto` line; line endings are not normalised"), "{}", stdout(&out));
    assert_eq!(std::fs::read(repo.dir.join(".gitattributes")).unwrap(), original);
}

#[test]
fn doctor_reports_gitattributes_directory_as_unreadable() {
    let repo = Repo::new();
    std::fs::create_dir(repo.dir.join(".gitattributes")).unwrap();
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("warn  .gitattributes: cannot read: "), "{report}");
    assert!(!report.contains(".gitattributes: missing"), "{report}");
    let fixed = repo.gir(&["doctor", "--fix"]);
    assert_eq!(stderr(&fixed), "");
    assert!(repo.dir.join(".gitattributes").is_dir());
}

#[cfg(unix)]
fn with_mode(repo: &Repo, rel: &str, mode: u32) -> bool {
    use std::os::unix::fs::PermissionsExt;
    let path = repo.dir.join(rel);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).unwrap();
    let denied = std::fs::read(&path).is_err();
    if !denied {
        eprintln!("skipped: mode {mode:o} does not deny reading {rel} (running as root?)");
    }
    denied
}

#[cfg(unix)]
fn content_with_mode(repo: &Repo, rel: &str) -> String {
    use std::os::unix::fs::PermissionsExt;
    let path = repo.dir.join(rel);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    std::fs::read_to_string(path).unwrap()
}

#[cfg(unix)]
#[test]
fn doctor_reports_unreadable_gitattributes_and_does_not_fix_it() {
    let repo = Repo::new();
    repo.write(".gitattributes", "# mine\n");
    if !with_mode(&repo, ".gitattributes", 0o000) {
        return;
    }
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("warn  .gitattributes: cannot read: "), "{report}");
    assert!(!report.contains(".gitattributes: missing"), "{report}");
    let fixed = repo.gir(&["doctor", "--fix"]);
    assert_eq!(stderr(&fixed), "");
    assert!(stdout(&fixed).contains("warn  .gitattributes: cannot read: "), "{}", stdout(&fixed));
    assert_eq!(content_with_mode(&repo, ".gitattributes"), "# mine\n");
}

#[cfg(unix)]
#[test]
fn doctor_fix_keeps_write_only_gitattributes() {
    let repo = Repo::new();
    repo.write(".gitattributes", "# mine\n");
    if !with_mode(&repo, ".gitattributes", 0o200) {
        return;
    }
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(!fixed.contains("fixed .gitattributes"), "{fixed}");
    assert_eq!(content_with_mode(&repo, ".gitattributes"), "# mine\n");
}
