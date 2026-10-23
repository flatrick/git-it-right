use std::fmt::Write;

struct Tip {
    key: &'static str,
    value: &'static str,
    effect: &'static str,
    useful_when: &'static str,
}

struct Catalog {
    presentation: &'static [Tip],
    behavior: &'static [Tip],
}

const CATALOG: Catalog = Catalog {
    presentation: &[
        Tip {
            key: "diff.colorMoved",
            value: "plain",
            effect: "Colors lines Git detects as moved when diff coloring is enabled.",
            useful_when: "You want to distinguish moved code from additions and deletions.",
        },
        Tip {
            key: "branch.sort",
            value: "-committerdate",
            effect: "Lists branches by newest commit date, which is not the branch creation date.",
            useful_when: "You have many branches and want recent activity first.",
        },
        Tip {
            key: "tag.sort",
            value: "version:refname",
            effect: "Sorts version-like tag names by version components, so v1.9 comes before v1.10.",
            useful_when: "You often inspect release tags.",
        },
        Tip {
            key: "column.ui",
            value: "auto",
            effect: "Uses columns for supported listings in a terminal and leaves redirected output plain.",
            useful_when: "You want compact interactive branch or tag listings.",
        },
        Tip {
            key: "diff.mnemonicPrefix",
            value: "true",
            effect: "Uses prefixes such as i/ and w/ to identify the index and working-tree sides of a diff.",
            useful_when: "You compare staged and unstaged changes.",
        },
        Tip {
            key: "diff.context",
            value: "10",
            effect: "Shows ten context lines around each diff hunk instead of three, making diffs longer.",
            useful_when: "Surrounding code helps you review a change.",
        },
        Tip {
            key: "commit.verbose",
            value: "true",
            effect: "Shows the staged diff in the commit-message editor without adding it to the saved message.",
            useful_when: "You want to review staged changes while writing the message.",
        },
    ],
    behavior: &[
        Tip {
            key: "help.autoCorrect",
            value: "prompt",
            effect: "After a mistyped Git command, prompts before running a suggested correction; accepting it executes that command.",
            useful_when: "You want typo help and can review the suggested command before confirming.",
        },
        Tip {
            key: "rebase.missingCommitsCheck",
            value: "error",
            effect: "Stops an interactive rebase if a commit disappears from its todo list; an explicit drop still works.",
            useful_when: "You hand-edit rebase todo lists and want to catch deleted lines.",
        },
        Tip {
            key: "user.useConfigOnly",
            value: "true",
            effect: "Stops Git from guessing your author or committer identity; commits fail until the required identity is configured.",
            useful_when: "Work and personal repositories need different names or email addresses.",
        },
        Tip {
            key: "fetch.all",
            value: "true",
            effect: "A bare git fetch updates all configured remotes except ones set to skipFetchAll; it can make extra network requests and take longer.",
            useful_when: "You regularly work with multiple remotes and want to fetch them together.",
        },
        Tip {
            key: "push.followTags",
            value: "true",
            effect: "Also publishes missing annotated tags reachable from the refs you push, even if you did not name those tags; lightweight tags are excluded.",
            useful_when: "You intend to publish annotated release tags with their commits.",
        },
    ],
};

pub fn page() -> String {
    let mut page = String::from(concat!(
        "Optional Git settings\n",
        "gir tips changes nothing. Run only the settings that fit your work.\n",
        "These examples use global scope; repository-local config can override them."
    ));
    append_group(&mut page, "Presentation and editor context", CATALOG.presentation);
    append_group(&mut page, "Workflow and publishing choices", CATALOG.behavior);
    page
}

fn append_group(page: &mut String, heading: &str, tips: &[Tip]) {
    write!(page, "\n\n{heading}").unwrap();
    for tip in tips {
        write!(
            page,
            "\n\n  git config --global {} {}\n    Effect: {}\n    Useful when: {}",
            tip.key, tip.value, tip.effect, tip.useful_when
        )
        .unwrap();
    }
}
