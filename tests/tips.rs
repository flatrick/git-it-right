mod common;

use common::{Repo, stderr};

const COMMANDS: [&str; 12] = [
    "diff.colorMoved plain",
    "branch.sort -committerdate",
    "tag.sort version:refname",
    "column.ui auto",
    "diff.mnemonicPrefix true",
    "diff.context 10",
    "commit.verbose true",
    "help.autoCorrect prompt",
    "rebase.missingCommitsCheck error",
    "user.useConfigOnly true",
    "fetch.all true",
    "push.followTags true",
];

#[test]
fn tips_lists_safe_settings_first_with_effects_and_use_cases() {
    let repo = Repo::new();
    let out = repo.gir(&["tips"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(out.stderr.is_empty(), "{}", stderr(&out));
    let report = String::from_utf8(out.stdout).unwrap();
    assert!(
        report.starts_with(concat!(
            "Optional Git settings\n",
            "gir tips changes nothing. Run only the settings that fit your work.\n",
            "These examples use global scope; repository-local config can override them.\n\n",
            "Presentation and editor context"
        )),
        "{report}"
    );
    assert!(report.contains("repository-local config can override"), "{report}");
    let lines: Vec<&str> = report.lines().collect();
    let commands: Vec<&str> = lines.iter().filter_map(|line| line.strip_prefix("  git config --global ")).collect();
    assert_eq!(commands, COMMANDS);

    let presentation = report.find("Presentation and editor context").unwrap();
    let behavior = report.find("Workflow and publishing choices").unwrap();
    assert!(presentation < behavior, "{report}");
    for command in COMMANDS {
        let line = format!("  git config --global {command}");
        let index = lines.iter().position(|candidate| *candidate == line).unwrap();
        assert!(lines[index + 1].starts_with("    Effect: ") && lines[index + 1].len() > "    Effect: ".len(), "{command}: {report}");
        assert!(
            lines[index + 2].starts_with("    Useful when: ") && lines[index + 2].len() > "    Useful when: ".len(),
            "{command}: {report}"
        );
    }
    for (command, effects) in [
        ("help.autoCorrect prompt", ["prompts", "accepting it executes"]),
        ("rebase.missingCommitsCheck error", ["Stops an interactive rebase", "explicit drop"]),
        ("user.useConfigOnly true", ["commits fail", "configured"]),
        ("fetch.all true", ["all configured remotes", "take longer"]),
        ("push.followTags true", ["publishes missing annotated tags", "even if you did not name"]),
    ] {
        let line = format!("  git config --global {command}");
        let index = lines.iter().position(|candidate| *candidate == line).unwrap();
        assert!(
            effects.iter().all(|effect| lines[index + 1].contains(effect)),
            "missing behavior disclosure for {command}: {}",
            lines[index + 1]
        );
    }
}

#[test]
fn tips_works_outside_a_repository_without_git_or_config_writes() {
    let repo = Repo::new();
    let global_before = std::fs::read(&repo.global).unwrap();
    let local = repo.dir.join(".git/config");
    let local_before = std::fs::read(&local).unwrap();
    let inside = repo.gir(&["tips"]);
    let outside = repo.cmd(env!("CARGO_BIN_EXE_gir")).current_dir(repo.dir.parent().unwrap()).env("PATH", "").arg("tips").output().unwrap();
    assert_eq!(outside.status.code(), Some(0), "{}", stderr(&outside));
    assert!(outside.stderr.is_empty(), "{}", stderr(&outside));
    assert_eq!(outside.stdout, inside.stdout);
    assert_eq!(std::fs::read(&repo.global).unwrap(), global_before);
    assert_eq!(std::fs::read(&local).unwrap(), local_before);
}
