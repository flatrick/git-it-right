# gir fixup, amend, reword and squash

## Subspecifications

`NONE`.

## Subcommands

<a id="req-fixup-subcommands"></a>
**subcommands.** `gir fixup`, `gir amend`, `gir reword` and `gir squash` SHALL create their commit with `git commit --quiet` and `--fixup=<sha>`, `--fixup=amend:<sha>`, `--fixup=reword:<sha>` and `--squash=<sha>` respectively, where `<sha>` is the full target commit ID.

<a id="req-fixup-subcommand-scope"></a>
**subcommand-scope.** A requirement in this file that names `gir fixup` SHALL apply to all four subcommands unless it names others; a message it quotes with `gir fixup` SHALL name the subcommand that was run, and a quoted `fixup!` SHALL read `amend!` for `gir amend` and `gir reword` and `squash!` for `gir squash`.

## Arguments and flags

<a id="req-fixup-arguments"></a>
**arguments.** `gir fixup` SHALL accept zero or one positional commit argument and the optional `--dry-run` and `--split` flags in any order; `gir reword` SHALL NOT accept `--split`.

## Target selection

<a id="req-fixup-staged-required"></a>
**staged-required.** With no staged changes, `gir fixup` and `gir squash` SHALL print ``gir: nothing staged; `git add` the fix first (more: gir explain fixup)`` to stderr and exit `2`, including when a commit argument is supplied.

<a id="req-fixup-amend-staged-required"></a>
**amend-staged-required.** With no staged changes, `gir amend` SHALL print ``gir: nothing staged; `git add` the fix first, or change only the message: gir reword (more: gir explain fixup)`` to stderr and exit `2`, including when a commit argument is supplied.

<a id="req-fixup-reword-ignores-index"></a>
**reword-ignores-index.** `gir reword` SHALL NOT require staged changes; its commit SHALL have its parent's tree, and staged changes SHALL stay staged.

<a id="req-fixup-reword-target"></a>
**reword-target.** Without a commit argument and not interactive, `gir reword` SHALL print `gir: nothing to trace for a reword; pass one: gir reword <commit>` to stderr and exit `2`.

<a id="req-fixup-explicit-target"></a>
**explicit-target.** Given staged changes, `gir fixup COMMIT` SHALL target the named commit without checking which commit last changed the staged lines, including when a new file is staged, provided the target passes the checks below.

<a id="req-fixup-explicit-target-on-branch"></a>
**explicit-target-on-branch.** `gir fixup COMMIT` SHALL refuse a target that is not `HEAD` or an ancestor of `HEAD` with ``gir: `COMMIT` is not in the current branch's history`` on stderr and exit `2`, creating no commit.

<a id="req-fixup-explicit-target-after-base"></a>
**explicit-target-after-base.** When a base branch is found, `gir fixup COMMIT` SHALL refuse a target reachable from that base with ``gir: `COMMIT` is already on the base branch`` on stderr and exit `2`, creating no commit.

<a id="req-fixup-invalid-target"></a>
**invalid-target.** Given staged changes, `gir fixup no-such-commit` SHALL print ``gir: `no-such-commit` is not a commit`` to stderr and exit `2`.

<a id="req-fixup-staged-line-target"></a>
**staged-line-target.** Without a commit argument, `gir fixup`, `gir amend` and `gir squash` SHALL select the commit that last changed the staged lines when they identify one eligible commit.

<a id="req-fixup-insertion-target"></a>
**insertion-target.** For a staged insertion with no replaced lines, `gir fixup` SHALL use the adjacent lines in `HEAD` to identify its target.

<a id="req-fixup-base-lookup"></a>
**base-lookup.** The base commit SHALL be the merge base with `HEAD` of the first of `refs/remotes/origin/HEAD`, `refs/heads/main`, `refs/heads/master` and `@{upstream}` whose merge base resolves and is not `HEAD` itself; when none qualifies, no base is found.

<a id="req-fixup-base-limit"></a>
**base-limit.** When a base branch is found, automatic selection SHALL accept only commits after the base commit and SHALL refuse a hunk that replaces any line last changed on the base branch with `already on the base branch` on stderr, even when it also replaces later lines. A pure insertion SHALL be refused only when every neighbouring line is on the base branch.

<a id="req-fixup-no-base-limit"></a>
**no-base-limit.** When no branch base can be found, automatic selection SHALL permit a commit already present on the current branch if its staged lines identify that commit.

<a id="req-fixup-new-file"></a>
**new-file.** Without a commit argument, `gir fixup` SHALL refuse a staged new file with `<path> is a new file, so it has no earlier commit; pass one: gir fixup <commit>` on stderr and exit `2`.

<a id="req-fixup-unattributed-lines"></a>
**unattributed-lines.** If no commit can be identified for a staged hunk, or a staged file has no changed lines to trace (a binary or mode-only change), automatic selection SHALL print `cannot tell which commit` and `pass one: gir fixup <commit>` on stderr and exit `2`.

<a id="req-fixup-multiple-targets"></a>
**multiple-targets.** If staged hunks identify several eligible commits, `gir fixup` SHALL refuse with `staged changes belong to several commits:` on stderr.

<a id="req-fixup-multiple-target-details"></a>
**multiple-target-details.** On a multiple-target refusal, `gir fixup` SHALL list each target's 10-character commit ID, subject, and file location on stderr.

<a id="req-fixup-split-hint"></a>
**split-hint.** On a multiple-target refusal, `gir fixup` SHALL print `split: git restore --staged . && git add -p, then one gir fixup per commit` on stderr.

<a id="req-fixup-split-flag-hint"></a>
**split-flag-hint.** On a multiple-target refusal, `gir fixup` SHALL print `or: gir fixup --split creates one fixup! per commit` on stderr after the split hint.

<a id="req-fixup-absorb-hint"></a>
**absorb-hint.** On a multiple-target refusal of `gir fixup` itself with `git-absorb` installed, it SHALL print `or: git absorb (installed) creates one fixup per commit` on stderr.

## Asking in a terminal

<a id="req-fixup-interactive"></a>
**interactive.** A run SHALL be interactive when `GIR_INTERACTIVE` is `1`, or when `GIR_INTERACTIVE` is not `0` and stdin and stderr are both terminals; a run with `--dry-run` SHALL NOT be interactive.

<a id="req-fixup-ask-several"></a>
**ask-several.** When interactive and the staged hunks identify several eligible commits, `gir fixup` SHALL, instead of refusing, list each target numbered from `1` with its 10-character commit ID, subject and file locations, then `s) split: one fixup! per commit`, on stderr, and read an answer from stdin.

<a id="req-fixup-ask-branch-commit"></a>
**ask-branch-commit.** When interactive, and automatic selection meets a new file or a file or hunk it cannot trace, or `gir reword` has no commit argument, gir SHALL list up to 20 of the newest commits after the base commit, or on `HEAD` when no base is found, numbered from `1` with their 10-character commit ID and subject, on stderr, and read an answer from stdin.

<a id="req-fixup-ask-answers"></a>
**ask-answers.** A listed number SHALL select that commit as the target of all staged changes; `s`, where offered, SHALL split as `--split` does; any other answer except a cancel SHALL ask again.

<a id="req-fixup-ask-cancel"></a>
**ask-cancel.** An empty answer, `q`, or end of input SHALL cancel: gir SHALL print `gir: cancelled; nothing committed` to stderr, create no commit, and exit `1`.

## Splitting

<a id="req-fixup-split"></a>
**split.** `gir fixup --split` without a commit argument SHALL create one commit per target the staged hunks identify, in ascending order of full commit ID, each holding only the hunks traced to its target, leaving the working tree unchanged and `HEAD`'s tree equal to the tree staged before the command.

<a id="req-fixup-split-refusals"></a>
**split-refusals.** Before creating any commit, `--split` SHALL refuse a hunk whose lines trace to several commits with `gir: <path>:<line> spans several commits; split it with git add -p` on stderr and exit `2`; the other refusals of automatic selection SHALL apply unchanged.

<a id="req-fixup-split-with-target"></a>
**split-with-target.** `--split` with a commit argument SHALL print `gir: --split finds each commit itself; drop the commit argument` to stderr and exit `2`.

<a id="req-fixup-split-rollback"></a>
**split-rollback.** If creating a split commit fails, gir SHALL reset `HEAD` to the commit it started from and the index to the tree staged before the command, print a message ending `; restored HEAD and the index` on stderr, and exit `2`.

<a id="req-fixup-split-output"></a>
**split-output.** After splitting, gir SHALL print `gir: created fixup! for <10-character-sha> <target-subject>` for each commit and then one `  fold: git rebase --autosquash <10-character-base-sha>` line to stderr; with `--dry-run` it SHALL instead print each target's 10-character commit ID and subject on its own stdout line and create no commit.

## Results and repository changes

<a id="req-fixup-dry-run"></a>
**dry-run.** `gir fixup --dry-run` SHALL print only the target's 10-character commit ID, one space, and its subject to stdout, exit `0`, and leave `HEAD` and the index unchanged.

<a id="req-fixup-create-commit"></a>
**create-commit.** Without `--dry-run`, a successful `gir fixup` SHALL create a new commit with subject `fixup! <target-subject>` from the staged changes and exit `0`.

<a id="req-fixup-success-message"></a>
**success-message.** After creating the commit, `gir fixup` SHALL print `gir: created fixup! for <10-character-sha> <target-subject>` and `  fold: git rebase --autosquash <10-character-base-sha>` to stderr, using `<base>` when no base is found.

<a id="req-fixup-rebase-left-to-user"></a>
**rebase-left-to-user.** `gir fixup` SHALL leave the new `fixup!` commit in branch history without running `git rebase`.

<a id="req-fixup-config-unchanged"></a>
**config-unchanged.** `gir fixup` SHALL NOT change Git configuration.

<a id="req-fixup-commit-failure"></a>
**commit-failure.** If its `git commit` fails, gir SHALL print `gir: git commit --quiet ` followed by the subcommand's commit option with the full target SHA (for `gir fixup`, `--fixup=<sha>`) and ` failed` on stderr and exit `2`.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
