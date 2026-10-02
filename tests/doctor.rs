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

#[cfg(windows)]
fn icacls(repo: &Repo, rel: &str, args: &[&str]) {
    let out = std::process::Command::new("icacls").arg(repo.dir.join(rel)).args(args).output().unwrap();
    assert!(out.status.success(), "icacls {args:?}: {}", stdout(&out));
}

#[cfg(windows)]
fn deny(repo: &Repo, rel: &str, right: &str) {
    let user = std::env::var("USERNAME").unwrap();
    icacls(repo, rel, &["/deny", &format!("{user}:({right})")]);
    assert!(std::fs::read(repo.dir.join(rel)).is_err(), "denying {right} did not deny reading {rel}");
}

#[cfg(windows)]
fn content_after_reset(repo: &Repo, rel: &str) -> String {
    icacls(repo, rel, &["/reset"]);
    std::fs::read_to_string(repo.dir.join(rel)).unwrap()
}

#[cfg(windows)]
#[test]
fn doctor_reports_read_denied_gitattributes_and_does_not_fix_it() {
    let repo = Repo::new();
    repo.write(".gitattributes", "# mine\n");
    deny(&repo, ".gitattributes", "R");
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report.contains("warn  .gitattributes: cannot read: "), "{report}");
    assert!(!report.contains(".gitattributes: missing"), "{report}");
    let fixed = repo.gir(&["doctor", "--fix"]);
    assert_eq!(stderr(&fixed), "");
    assert!(stdout(&fixed).contains("warn  .gitattributes: cannot read: "), "{}", stdout(&fixed));
    assert_eq!(content_after_reset(&repo, ".gitattributes"), "# mine\n");
}

#[cfg(windows)]
#[test]
fn doctor_fix_keeps_read_data_denied_gitattributes() {
    let repo = Repo::new();
    repo.write(".gitattributes", "# mine\n");
    deny(&repo, ".gitattributes", "RD");
    let fixed = stdout(&repo.gir(&["doctor", "--fix"]));
    assert!(!fixed.contains("fixed .gitattributes"), "{fixed}");
    assert_eq!(content_after_reset(&repo, ".gitattributes"), "# mine\n");
}

/// Adds index entries with exactly these path bytes and no files, so names that Windows
/// cannot hold as files can still be tested there.
fn add_index_entries(repo: &Repo, mode: &str, paths: &[&[u8]]) {
    use std::io::Write;
    repo.write("blob.txt", "content\n");
    let blob = repo.git(&["hash-object", "-w", "blob.txt"]);
    let mut input = Vec::new();
    for path in paths {
        input.extend_from_slice(format!("{mode} {blob}\t").as_bytes());
        input.extend_from_slice(path);
        input.push(0);
    }
    let mut child = repo
        .cmd("git")
        .args(["-c", "core.protectNTFS=false", "update-index", "--add", "-z", "--index-info"])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&input).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "git update-index: {}", String::from_utf8_lossy(&out.stderr));
}

fn report_line<'a>(report: &'a str, id: &str) -> Option<&'a str> {
    report.lines().find(|l| l.contains(&format!(" {id}: ")))
}

fn index_modes(repo: &Repo) -> Vec<Vec<u8>> {
    repo.git_out(&["ls-files", "-s", "-z"]).stdout.split(|&b| b == 0).filter(|r| !r.is_empty()).map(<[u8]>::to_vec).collect()
}

#[test]
fn doctor_reports_exec_bit_for_hook_names_git_quotes() {
    let repo = Repo::new();
    add_index_entries(&repo, "100644", &[b".githooks/pre \"x\"", b".githooks/back\\slash", b".githooks/my hook"]);
    let report = stdout(&repo.gir(&["doctor"]));
    assert_eq!(
        report_line(&report, "exec-bit"),
        Some("warn  exec-bit: scripts not executable in git: .githooks/back\\slash .githooks/my hook .githooks/pre \"x\""),
        "{report}"
    );
}

/// Commits files with exactly these names.
fn commit_files_named(repo: &Repo, names: &[&[u8]]) {
    for name in names {
        #[cfg(unix)]
        let path = {
            use std::os::unix::ffi::OsStrExt;
            repo.dir.join(std::ffi::OsStr::from_bytes(name))
        };
        #[cfg(not(unix))]
        let path = repo.dir.join(std::str::from_utf8(name).unwrap());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, "#!/bin/sh\n").unwrap();
    }
    repo.git(&["add", "-A"]);
    repo.git(&["commit", "-q", "-m", "chore: hooks"]);
}

#[test]
fn doctor_fix_sets_exec_bit_for_deleted_hook_without_restoring_file() {
    let repo = Repo::new();
    let hook = ".githooks/pre-commit";
    commit_files_named(&repo, &[hook.as_bytes()]);
    std::fs::remove_file(repo.dir.join(hook)).unwrap();

    let out = repo.gir(&["doctor", "--fix"]);
    let report = stdout(&out);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    assert!(report.lines().last().is_some_and(|line| line.starts_with("gir doctor:")), "{report}");
    assert!(index_modes(&repo).iter().any(|r| r.starts_with(b"100755 ") && r.ends_with(b"\t.githooks/pre-commit")));
    assert!(!repo.dir.join(hook).exists());
}

#[test]
fn doctor_fix_does_not_stage_unstaged_hook_edit() {
    let repo = Repo::new();
    let hook = ".githooks/pre-commit";
    commit_files_named(&repo, &[hook.as_bytes()]);
    repo.write(hook, "#!/bin/sh\necho edited\n");

    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    assert!(index_modes(&repo).iter().any(|r| r.starts_with(b"100755 ") && r.ends_with(b"\t.githooks/pre-commit")));
    assert_eq!(repo.git(&["show", ":.githooks/pre-commit"]), "#!/bin/sh");
    assert_eq!(repo.git(&["diff", "--name-only"]), hook);
}

#[test]
fn doctor_fix_keeps_hooks_outside_sparse_cone_skipped() {
    let repo = Repo::new();
    let hooks = [".githooks/pre-commit", ".githooks/pre-push"];
    commit_files_named(&repo, &[hooks[0].as_bytes(), hooks[1].as_bytes()]);
    repo.git(&["sparse-checkout", "set", "--no-cone", "/*", "!/.githooks/"]);
    for hook in hooks {
        assert!(!repo.dir.join(hook).exists());
    }

    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    let modes = index_modes(&repo);
    let tagged = repo.git(&["ls-files", "-v"]);
    for hook in hooks {
        assert!(modes.iter().any(|r| r.starts_with(b"100755 ") && r.ends_with(format!("\t{hook}").as_bytes())), "{modes:?}");
        assert!(tagged.lines().any(|line| line == format!("S {hook}")), "{tagged}");
        assert!(!repo.dir.join(hook).exists());
    }
}

#[test]
fn doctor_fix_sets_exec_bit_for_unchanged_hook() {
    let repo = Repo::new();
    let hook = ".githooks/pre-commit";
    commit_files_named(&repo, &[hook.as_bytes()]);

    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    assert!(index_modes(&repo).iter().any(|r| r.starts_with(b"100755 ") && r.ends_with(b"\t.githooks/pre-commit")));
    assert!(repo.dir.join(hook).exists());
}

#[test]
fn doctor_fix_leaves_unmerged_hook_stages_untouched() {
    let repo = Repo::new();
    commit_files_named(&repo, &[b".githooks/h", b".githooks/other"]);
    repo.git(&["switch", "-q", "-c", "other"]);
    repo.commit_file(".githooks/h", "#!/bin/sh\necho other\n", "feat: other hook");
    repo.git(&["switch", "-q", "main"]);
    repo.commit_file(".githooks/h", "#!/bin/sh\necho main\n", "feat: main hook");
    assert!(!repo.git_out(&["merge", "-q", "other"]).status.success(), "the merge must conflict");

    let before = repo.git_out(&["ls-files", "-s", "--", ".githooks/h"]).stdout;
    assert_eq!(String::from_utf8_lossy(&before).lines().count(), 3, "{}", String::from_utf8_lossy(&before));
    let report = stdout(&repo.gir(&["doctor"]));
    assert!(report_line(&report, "exec-bit").is_some_and(|line| line.contains(".githooks/other")), "{report}");
    assert!(!report_line(&report, "exec-bit").is_some_and(|line| line.contains(".githooks/h")), "{report}");

    let out = repo.gir(&["doctor", "--fix"]);
    let fixed = stdout(&out);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    assert!(!report_line(&fixed, "exec-bit").is_some_and(|line| line.contains(".githooks/h")), "{fixed}");
    assert_eq!(repo.git_out(&["ls-files", "-s", "--", ".githooks/h"]).stdout, before);
    assert!(index_modes(&repo).iter().any(|r| r.starts_with(b"100755 ") && r.ends_with(b"\t.githooks/other")));
}

// Windows file names cannot contain a double quote or a backslash.
#[cfg(unix)]
#[test]
fn doctor_fix_sets_exec_bit_on_hook_names_git_quotes() {
    let repo = Repo::new();
    commit_files_named(&repo, &[b".githooks/pre \"x\"", b".githooks/back\\slash"]);
    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    let hooks: Vec<Vec<u8>> = index_modes(&repo).into_iter().filter(|r| r.windows(10).any(|w| w == b".githooks/")).collect();
    assert_eq!(hooks.len(), 2, "{hooks:?}");
    assert!(hooks.iter().all(|r| r.starts_with(b"100755")), "{hooks:?}");
}

#[test]
fn doctor_reports_windows_unsafe_names_as_stored() {
    let repo = Repo::new();
    add_index_entries(&repo, "100644", &[b"say \"hi\".txt", b"back\\slash.txt"]);
    let report = stdout(&repo.gir(&["doctor"]));
    assert_eq!(
        report_line(&report, "windows-names"),
        Some("warn  windows-names: cannot be checked out on Windows: back\\slash.txt say \"hi\".txt"),
        "{report}"
    );
}

#[test]
fn doctor_flags_control_characters_and_non_utf8_names_in_git_quoted_form() {
    let repo = Repo::new();
    add_index_entries(&repo, "100644", &[b"tab\there.txt", b"bell\x07.txt", b"caf\xe9.txt"]);
    let report = stdout(&repo.gir(&["doctor"]));
    assert_eq!(
        report_line(&report, "windows-names"),
        Some("warn  windows-names: cannot be checked out on Windows: \"bell\\a.txt\" \"caf\\351.txt\" \"tab\\there.txt\""),
        "{report}"
    );
}

#[test]
fn doctor_compares_names_as_stored_for_case_collisions() {
    let repo = Repo::new();
    add_index_entries(&repo, "100644", &[b"Q \"a\".txt", b"q \"a\".txt", b"x\xe8", b"x\xe9"]);
    let report = stdout(&repo.gir(&["doctor"]));
    assert_eq!(
        report_line(&report, "case-collision"),
        Some("warn  case-collision: paths differ only in case, which breaks Windows/macOS checkouts: Q \"a\".txt = q \"a\".txt"),
        "{report}"
    );
}

#[test]
fn doctor_suggests_ignore_rules_for_a_non_ascii_marker_name() {
    let repo = Repo::new();
    repo.write("\u{c5}ngstr\u{f6}m.csproj", "<Project/>\n");
    let report = stdout(&repo.gir(&["doctor"]));
    let line = report_line(&report, ".gitignore").unwrap_or_default();
    assert!(line.contains(" bin/ obj/"), "{report}");
}

// Windows cannot pass non-UTF-8 path bytes as arguments, and macOS cannot create this name
// in the working tree. Git can store the raw bytes in its index on Unix.
#[cfg(unix)]
#[test]
fn doctor_fix_sets_exec_bit_on_a_non_utf8_hook_name() {
    let repo = Repo::new();
    add_index_entries(&repo, "100644", &[b".githooks/hook\xe9"]);
    let report = stdout(&repo.gir(&["doctor"]));
    assert_eq!(report_line(&report, "exec-bit"), Some("warn  exec-bit: scripts not executable in git: \".githooks/hook\\351\""), "{report}");
    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    assert!(index_modes(&repo).iter().any(|r| r.starts_with(b"100755") && r.ends_with(b"hook\xe9")), "{:?}", index_modes(&repo));
}

#[test]
fn doctor_fix_leaves_no_unstaged_change_for_fixed_scripts() {
    let repo = Repo::new();
    commit_files_named(&repo, &[b".githooks/pre-commit", b"tools/run.sh"]);

    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    let modes = index_modes(&repo);
    for path in [".githooks/pre-commit", "tools/run.sh"] {
        assert!(modes.iter().any(|r| r.starts_with(b"100755 ") && r.ends_with(format!("\t{path}").as_bytes())), "{modes:?}");
        assert_eq!(std::fs::read_to_string(repo.dir.join(path)).unwrap(), "#!/bin/sh\n");
    }
    assert_eq!(repo.git(&["diff", "--name-only"]), "");
}

#[cfg(unix)]
fn disk_mode(repo: &Repo, rel: &str) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    std::fs::symlink_metadata(repo.dir.join(rel)).unwrap().permissions().mode() & 0o777
}

// Windows has no executable bit to check.
#[cfg(unix)]
#[test]
fn doctor_fix_makes_fixed_scripts_executable_on_disk() {
    let repo = Repo::new();
    commit_files_named(&repo, &[b".githooks/pre-commit", b"tools/run.sh", b"README.md"]);

    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    for path in [".githooks/pre-commit", "tools/run.sh"] {
        assert_ne!(disk_mode(&repo, path) & 0o100, 0, "{path}: {:o}", disk_mode(&repo, path));
    }
    assert_eq!(disk_mode(&repo, "README.md") & 0o111, 0);
    repo.git(&["add", "--", ".githooks/pre-commit", "tools/run.sh"]);
    let modes = index_modes(&repo);
    for path in [".githooks/pre-commit", "tools/run.sh"] {
        assert!(modes.iter().any(|r| r.starts_with(b"100755 ") && r.ends_with(format!("\t{path}").as_bytes())), "{modes:?}");
    }
}

// Git ignores a hook that is not executable, and only Unix has that bit.
#[cfg(unix)]
#[test]
fn doctor_fix_lets_git_run_the_fixed_hook() {
    let repo = Repo::new();
    repo.write(".githooks/pre-commit", "#!/bin/sh\n: > .git/pre-commit-ran\n");
    repo.git(&["add", "-A"]);
    repo.git(&["commit", "-q", "--no-verify", "-m", "chore: hook"]);

    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    let commit = repo.git_out(&["commit", "-q", "-m", "chore: fixed hook mode"]);
    assert!(commit.status.success(), "{}", stderr(&commit));
    assert!(!stderr(&commit).contains("hook was ignored"), "{}", stderr(&commit));
    assert!(repo.dir.join(".git/pre-commit-ran").exists());
}

// Windows has no executable bit, so git shows no mode change there either way.
#[cfg(unix)]
#[test]
fn doctor_fix_makes_an_edited_script_executable_and_keeps_the_edit_unstaged() {
    let repo = Repo::new();
    commit_files_named(&repo, &[b"tools/run.sh"]);
    repo.write("tools/run.sh", "#!/bin/sh\necho edited\n");

    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    assert_eq!(std::fs::read_to_string(repo.dir.join("tools/run.sh")).unwrap(), "#!/bin/sh\necho edited\n");
    assert_eq!(repo.git(&["show", ":tools/run.sh"]), "#!/bin/sh");
    let diff = repo.git(&["diff", "--", "tools/run.sh"]);
    assert!(diff.contains("+echo edited"), "{diff}");
    assert!(!diff.contains("old mode"), "{diff}");
}

// Creating a symlink on Windows needs a privilege that test machines may not have.
#[cfg(unix)]
#[test]
fn doctor_fix_does_not_change_the_target_of_a_symlink_in_place_of_a_script() {
    use std::os::unix::fs::PermissionsExt;
    let repo = Repo::new();
    commit_files_named(&repo, &[b"tools/run.sh"]);
    let target = repo.dir.with_file_name("outside.txt");
    std::fs::write(&target, "not a script\n").unwrap();
    std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o644)).unwrap();
    std::fs::remove_file(repo.dir.join("tools/run.sh")).unwrap();
    std::os::unix::fs::symlink(&target, repo.dir.join("tools/run.sh")).unwrap();

    let out = repo.gir(&["doctor", "--fix"]);
    assert!(!stderr(&out).contains("fatal"), "{}", stderr(&out));
    assert!(index_modes(&repo).iter().any(|r| r.starts_with(b"100755 ") && r.ends_with(b"\ttools/run.sh")));
    assert!(repo.dir.join("tools/run.sh").is_symlink());
    assert_eq!(std::fs::metadata(&target).unwrap().permissions().mode() & 0o777, 0o644);
}
