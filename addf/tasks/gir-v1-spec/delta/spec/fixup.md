# gir fixup

## Subspecifications

`NONE`.

## Arguments and flags

<a id="req-fixup-arguments"></a>
**arguments.** `gir fixup` SHALL accept zero or one positional commit argument and the optional `--dry-run` flag in either order.

## Target selection

<a id="req-fixup-staged-required"></a>
**staged-required.** With no staged changes, `gir fixup` SHALL print ``gir: nothing staged; `git add` the fix first (more: gir explain fixup)`` to stderr and exit `2`, including when a commit argument is supplied.

<a id="req-fixup-explicit-target"></a>
**explicit-target.** Given staged changes, `gir fixup COMMIT` SHALL target the named commit without checking which commit last changed the staged lines, including when a new file is staged, provided the target passes the checks below.

<a id="req-fixup-explicit-target-on-branch"></a>
**explicit-target-on-branch.** `gir fixup COMMIT` SHALL refuse a target that is not `HEAD` or an ancestor of `HEAD` with ``gir: `COMMIT` is not in the current branch's history`` on stderr and exit `2`, creating no commit.

<a id="req-fixup-explicit-target-after-base"></a>
**explicit-target-after-base.** When a base branch is found, `gir fixup COMMIT` SHALL refuse a target reachable from that base with ``gir: `COMMIT` is already on the base branch`` on stderr and exit `2`, creating no commit.

<a id="req-fixup-invalid-target"></a>
**invalid-target.** Given staged changes, `gir fixup no-such-commit` SHALL print ``gir: `no-such-commit` is not a commit`` to stderr and exit `2`.

<a id="req-fixup-staged-line-target"></a>
**staged-line-target.** Without a commit argument, `gir fixup` SHALL select the commit that last changed the staged lines when they identify one eligible commit.

<a id="req-fixup-insertion-target"></a>
**insertion-target.** For a staged insertion with no replaced lines, `gir fixup` SHALL use the adjacent lines in `HEAD` to identify its target.

<a id="req-fixup-base-lookup"></a>
**base-lookup.** The base commit SHALL be the merge base with `HEAD` of the first of `refs/remotes/origin/HEAD`, `refs/heads/main`, `refs/heads/master` and `@{upstream}` whose merge base resolves and is not `HEAD` itself; when none qualifies, no base is found.

<a id="req-fixup-base-limit"></a>
**base-limit.** When a base branch is found, automatic selection SHALL accept only commits after the base commit and SHALL refuse a staged line last changed on the base branch with `already on the base branch` on stderr.

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

<a id="req-fixup-absorb-hint"></a>
**absorb-hint.** On a multiple-target refusal with `git-absorb` installed, `gir fixup` SHALL print `or: git absorb (installed) creates one fixup per commit` on stderr.

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
**commit-failure.** If `git commit --quiet --fixup=<full-target-sha>` fails, `gir fixup` SHALL print `gir: git commit --quiet --fixup=` followed by the target SHA and ` failed` on stderr and exit `2`.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
