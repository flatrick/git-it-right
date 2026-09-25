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
        "gir init [--force]",
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
