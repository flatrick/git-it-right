mod common;

use common::{Repo, stderr};

#[test]
fn version_flags_print_crate_version() {
    let repo = Repo::new();
    for flag in ["--version", "-V"] {
        let out = repo.gir(&[flag]);
        assert_eq!(out.status.code(), Some(0), "{flag}: {}", stderr(&out));
        assert_eq!(
            String::from_utf8_lossy(&out.stdout),
            format!("gir {}\n", env!("CARGO_PKG_VERSION")),
            "{flag}"
        );
    }
}

#[test]
fn help_flags_print_usage_for_every_subcommand() {
    let repo = Repo::new();
    let help = repo.gir(&[]);
    assert_eq!(help.status.code(), Some(0), "{}", stderr(&help));
    let usage = String::from_utf8_lossy(&help.stdout);
    assert!(usage.starts_with("gir: keeps git usage honest"), "{usage}");
    for args in [
        vec!["-h"],
        vec!["--help"],
        vec!["init", "-h"],
        vec!["init", "--help"],
        vec!["lint", "-h"],
        vec!["lint", "--help"],
        vec!["fixup", "-h"],
        vec!["fixup", "--help"],
        vec!["doctor", "-h"],
        vec!["doctor", "--help"],
        vec!["explain", "-h"],
        vec!["explain", "--help"],
        vec!["hook", "-h"],
        vec!["hook", "--help"],
    ] {
        let out = repo.gir(&args);
        assert_eq!(out.status.code(), Some(0), "{args:?}: {}", stderr(&out));
        assert_eq!(
            out.stdout, help.stdout,
            "{args:?} must print usage on stdout"
        );
        assert!(out.stderr.is_empty(), "{args:?}: {}", stderr(&out));
    }
}

#[test]
fn usage_lists_commands_with_their_arguments_and_flags() {
    let repo = Repo::new();
    let out = repo.gir(&["--help"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let usage = String::from_utf8_lossy(&out.stdout);
    for line in [
        "gir init [--force] [--optional]",
        "gir lint [<file>|-] [--fix] [--json]",
        "gir lint --range <A..B> [--json]",
        "gir fixup [<commit>] [--dry-run]",
        "gir doctor [--fix]",
        "gir explain [<topic>]",
        "gir hook commit-msg <file>",
        "gir hook pre-push <remote> <url>",
    ] {
        assert!(usage.contains(line), "usage must list {line:?}: {usage}");
    }
}

#[test]
fn unaccepted_long_flags_report_the_command_and_exit_two() {
    let repo = Repo::new();
    for (command, flag) in [
        ("init", "--json"),
        ("lint", "--force"),
        ("fixup", "--fix"),
        ("doctor", "--dry-run"),
        ("explain", "--json"),
        ("hook", "--force"),
    ] {
        let out = repo.gir(&[command, flag]);
        assert_eq!(
            out.status.code(),
            Some(2),
            "{command} {flag}: {}",
            stderr(&out)
        );
        assert!(
            stderr(&out).starts_with(&format!("gir: unknown option {flag} for `gir {command}`")),
            "{}",
            stderr(&out)
        );
        assert!(
            out.stdout.is_empty(),
            "{command} {flag} must not write stdout"
        );
    }
}

#[test]
fn range_is_rejected_outside_lint_and_accepted_by_lint() {
    let repo = Repo::new();
    repo.commit_file("base.txt", "base\n", "chore: base");
    for command in ["init", "fixup", "doctor", "explain", "hook"] {
        let out = repo.gir(&[command, "--range", "HEAD..HEAD"]);
        assert_eq!(out.status.code(), Some(2), "{command}: {}", stderr(&out));
        assert!(
            stderr(&out).starts_with(&format!("gir: unknown option --range for `gir {command}`")),
            "{}",
            stderr(&out)
        );
    }
    let out = repo.gir(&["lint", "--range", "HEAD..HEAD"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(out.stderr.is_empty(), "{}", stderr(&out));
}

#[test]
fn command_errors_print_gir_prefix_and_exit_two() {
    let repo = Repo::new();
    let missing = repo.gir(&["lint", "missing-message"]);
    assert_eq!(missing.status.code(), Some(2), "{}", stderr(&missing));
    assert!(
        stderr(&missing).starts_with("gir: cannot read missing-message:"),
        "{}",
        stderr(&missing)
    );
    assert_eq!(stderr(&missing).lines().count(), 1, "{}", stderr(&missing));

    repo.write(".girconfig", "[gir]\n\tsubjectMax = invalid\n");
    let invalid = repo.gir(&["lint", "missing-message"]);
    assert_eq!(invalid.status.code(), Some(2), "{}", stderr(&invalid));
    assert!(
        stderr(&invalid).starts_with("gir: .girconfig: `gir.subjectmax` is `invalid`"),
        "{}",
        stderr(&invalid)
    );
    assert_eq!(stderr(&invalid).lines().count(), 1, "{}", stderr(&invalid));

    repo.write(".girconfig", "");
    let git = repo.gir(&["lint", "--range", "not-a-revision"]);
    assert_eq!(git.status.code(), Some(2), "{}", stderr(&git));
    assert!(stderr(&git).starts_with("gir: fatal:"), "{}", stderr(&git));
    assert_eq!(
        stderr(&git)
            .lines()
            .filter(|line| line.starts_with("gir:"))
            .count(),
        1,
        "{}",
        stderr(&git)
    );
}

#[test]
fn unknown_short_flags_and_bad_arguments_exit_two() {
    let repo = Repo::new();
    let top = repo.gir(&["-C"]);
    assert_eq!(top.status.code(), Some(2), "{}", stderr(&top));
    assert_eq!(stderr(&top), "gir: invalid option '-C'\n");
    let sub = repo.gir(&["init", "-x"]);
    assert_eq!(sub.status.code(), Some(2), "{}", stderr(&sub));
    assert_eq!(stderr(&sub), "gir: unknown option -x\n");

    for args in [vec!["nope"], vec!["fixup", "a", "b"], vec!["doctor", "a"], vec!["explain", "a", "b"]] {
        let out = repo.gir(&args);
        assert_eq!(out.status.code(), Some(2), "{args:?}: {}", stderr(&out));
        assert!(stderr(&out).starts_with(&format!("gir: bad arguments for `{}`\n", args[0])), "{}", stderr(&out));
        assert!(stderr(&out).contains("usage:\n  gir init [--force]"), "{args:?} must print usage: {}", stderr(&out));
    }

    for args in [vec!["lint", "a", "b"], vec!["lint", "a", "--range", "HEAD"]] {
        let out = repo.gir(&args);
        assert_eq!(out.status.code(), Some(2), "{args:?}: {}", stderr(&out));
        assert_eq!(stderr(&out), "gir: gir lint takes one file, `-`, or --range\n", "{args:?}");
    }
}

#[test]
fn hook_takes_its_documented_arguments() {
    let repo = Repo::new();
    for args in [vec!["hook", "commit-msg"], vec!["hook", "commit-msg", "a", "b"], vec!["hook", "pre-push"], vec!["hook", "post-merge", "x"]] {
        let out = repo.gir(&args);
        assert_eq!(out.status.code(), Some(2), "{args:?}: {}", stderr(&out));
        assert_eq!(stderr(&out), format!("gir: unknown hook `{}`\n", args[1]), "{args:?}");
    }
    let bare = repo.gir(&["hook"]);
    assert_eq!(bare.status.code(), Some(2), "{}", stderr(&bare));
    assert!(stderr(&bare).starts_with("gir: bad arguments for `hook`\n"), "{}", stderr(&bare));
    for args in [vec!["hook", "pre-push", "origin"], vec!["hook", "pre-push", "origin", "url", "extra"]] {
        let out = repo.gir(&args);
        assert_eq!(out.status.code(), Some(0), "{args:?}: {}", stderr(&out));
    }
}

#[test]
fn repository_commands_work_from_a_subdirectory() {
    let repo = Repo::new();
    repo.commit_file("base.txt", "base\n", "chore: base");
    repo.git(&["switch", "-q", "-c", "topic"]);
    repo.commit_file("run.sh", "one\ntwo\n", "feat: add run");
    let target = repo.git(&["rev-parse", "HEAD"]);
    std::fs::create_dir(repo.dir.join("sub")).unwrap();
    let in_sub = |args: &[&str]| repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(repo.dir.join("sub")).args(args).output().unwrap();

    let doctor = in_sub(&["doctor"]);
    let report = String::from_utf8_lossy(&doctor.stdout);
    assert!(report.contains("warn  exec-bit: scripts not executable in git: run.sh"), "doctor must see root files: {report}");

    repo.write("run.sh", "one\nTWO\n");
    repo.git(&["add", "run.sh"]);
    let fixup = in_sub(&["fixup", "--dry-run"]);
    assert_eq!(fixup.status.code(), Some(0), "{}", stderr(&fixup));
    assert_eq!(String::from_utf8_lossy(&fixup.stdout), format!("{} feat: add run\n", &target[..10]));

    let init = in_sub(&["init"]);
    assert_eq!(init.status.code(), Some(0), "{}", stderr(&init));
    let staged = repo.git(&["ls-files", "--stage", "--", ".githooks"]);
    assert_eq!(staged.lines().filter(|l| l.starts_with("100755 ")).count(), 2, "{staged}");
}

#[test]
fn version_and_help_ignore_trailing_arguments() {
    let repo = Repo::new();
    let version = repo.gir(&["--version", "junk"]);
    assert_eq!(version.status.code(), Some(0), "{}", stderr(&version));
    assert_eq!(String::from_utf8_lossy(&version.stdout), format!("gir {}\n", env!("CARGO_PKG_VERSION")));
    let help = repo.gir(&["--help", "junk"]);
    assert_eq!(help.status.code(), Some(0), "{}", stderr(&help));
    assert!(String::from_utf8_lossy(&help.stdout).starts_with("gir: keeps git usage honest"));
}

#[test]
fn unknown_short_option_before_help_exits_two() {
    let repo = Repo::new();
    let out = repo.gir(&["lint", "-Z", "--help"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "gir: unknown option -Z\n");
    assert!(out.stdout.is_empty());
}

#[test]
fn closed_stdout_exits_141_without_output() {
    use std::io::Write;
    use std::process::Stdio;

    let repo = Repo::new();
    repo.commit_file("a.txt", "a\n", "feat: add a");
    repo.write("a.txt", "b\n");
    repo.git(&["add", "a.txt"]);
    let cases: [(&[&str], &str); 8] = [
        (&["--version"], ""),
        (&["--help"], ""),
        (&["explain"], ""),
        (&["explain", "doctor"], ""),
        (&["doctor"], ""),
        (&["lint", "--json", "-"], "feat: x\n"),
        (&["lint", "--fix", "-"], "Feat: x"),
        (&["fixup", "--dry-run"], ""),
    ];
    for (args, input) in cases {
        let (reader, writer) = std::io::pipe().unwrap();
        drop(reader);
        let mut child = repo
            .cmd(env!("CARGO_BIN_EXE_gir"))
            .args(args)
            .stdin(Stdio::piped())
            .stdout(writer)
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let _ = child.stdin.take().unwrap().write_all(input.as_bytes());
        let out = child.wait_with_output().unwrap();
        assert_eq!(stderr(&out), "", "{args:?}");
        assert_eq!(out.status.code(), Some(141), "{args:?}");
    }
}
