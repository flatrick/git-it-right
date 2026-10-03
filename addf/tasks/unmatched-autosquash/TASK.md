# TASK — `an unmatched autosquash commit gets advice that works`

## Resume

**Contract version:** `2`

**State:** `VERIFY`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY`

**Resume at:** Run `cargo test --no-fail-fast`, clippy, `cargo fmt --check` and `acceptance.py` on the final tree into `logs/`, then write a Verification for each of c1–c4.

**Open obligations:** A Verification for each of c1–c4 — blocks the VERIFY gate. The spec delta written into `spec/` — terminal checkpoint.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took (Q1–Q3, from `ledger/unmatched-autosquash-20261003.md`), and its DEFINE dialogue.
-   `probe-matching.py` - Probe: which autosquash subjects `git rebase --autosquash <base>` folds.
-   `logs/probe-matching-20261003-1354.log` - Evidence: the first run, whose reword cases reworded to a title the old one is a prefix of (see Understand).
-   `logs/probe-matching-20261003-1400-reword-fixed.log` - Evidence: the run after rewording to an unrelated title.
-   `logs/probe-matching-20261003-1412-edges.log`, `logs/probe-matching-20261003-1428-chains.log` - Evidence: runs with added edge and chain cases, before the leftover-classifier fix (see Investigate).
-   `logs/probe-matching-20261003-1431.log` - Evidence: the run of the current `probe-matching.py`.
-   `probe-advice.py` - Probe: whether blame names an unmatched commit's target, and whether advice built from it leaves no autosquash commit.
-   `logs/probe-advice-20261003-1420.log` - Evidence: the first run, which crashed in the probe's todo editor on the reword case.
-   `logs/probe-advice-20261003-1424.log` - Evidence: the run of the current `probe-advice.py`.
-   `acceptance.py` - Probe and acceptance: runs `gir lint --range`, follows its `try:` hint literally, and checks the resulting subjects and tree (c1, c2).
-   `logs/acceptance-20261003-1530.log` - Evidence: first run; its subject check missed an empty message, and one fixture was broken (see Implement).
-   `logs/acceptance-20261003-1540.log`, `logs/acceptance-20261003-1545.log` - Evidence: runs with exact subject checks, before and after fixing the script's own subject parsing; the second shows the `amend!` defect.
-   `logs/acceptance-20261003-1557.log` - Evidence: the run on `eb6fdce`, 7/7.
-   `logs/test-final-20261003-1615.log`, `logs/clippy-20261003-1615.log`, `logs/fmt-20261003-1615.log`, `logs/acceptance-final-20261003-1615.log`, `logs/explain-pages-20261003-1615.log` - Evidence: `cargo test --no-fail-fast`, clippy, `cargo fmt --check`, `acceptance.py` and both explain pages on the tree committed as `48bfb47`.

## Specification impact

- Current contract: `framework:spec/lint.md#req-lint-range-unsquashed`, `framework:spec/hooks.md#req-hooks-pre-push-rejects`, `framework:spec/explain.md#req-explain-known-pages`, `framework:spec/fixup.md#req-fixup-create-commit`, `framework:SPEC.md` (Boundaries)
- Proposed delta: `range-unsquashed` narrowed to commits that will fold, with a concrete base; a new `range-unmatched` with rule `fixup-unmatched`; `pre-push-rejects` extended with `fixup-unmatched` and its `--no-verify` hint; `fixup-unmatched` added to `known-pages`. `fixup.md` unchanged. Text under Decide.
- Terminal publication: `PENDING`

## Define

### Objective

`A user whose push or range lint is rejected as fixup-unsquashed is never sent into a loop: either following the advice folds the commit, or gir says it cannot be folded and what to do instead.`

DEFINE gate: `ESTABLISHED` — the operator agreed to the objective, scope and success criteria c1–c4 (`ledger.md` Q7, A7).

### Success criteria

<a id="c1-unmatched-detected"></a>
#### `c1-unmatched-detected`

-   Claim: `gir lint --range` and pre-push reject an autosquash commit that `git rebase --autosquash <base>` would not fold with a message that says it will not fold and why, and advice that, followed literally, leaves no autosquash commit behind.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user keeps looping between the rejection and a rebase that reports success.
-   Basis: [Verification](#verification-c1-unmatched-detected).

<a id="c2-published-target"></a>
#### `c2-published-target`

-   Claim: When an autosquash commit's target is already on the base branch or the remote, `gir lint --range` and pre-push still reject it, and the message says the target is already published and that pushing it anyway is the user's call.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user is told to run a rebase that cannot fold the commit, or is not told why.
-   Basis: [Verification](#verification-c2-published-target).

<a id="c3-explain-covers"></a>
#### `c3-explain-covers`

-   Claim: `gir explain fixup-unsquashed` describes the unmatched case and the published-target case, and what to do in each.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: the long-form help still sends the user into the loop.
-   Basis: [Verification](#verification-c3-explain-covers).

<a id="c4-survives-reword"></a>
#### `c4-survives-reword`

-   Claim: A commit made by `gir fixup`, `gir amend` or `gir squash` still folds into its target with `git rebase --autosquash <base>` after the target was reworded, or this Task records, from a probe, why that cannot be done without rewriting commits.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: gir's own fixup workflow keeps producing commits that cannot fold.
-   Basis: [Verification](#verification-c4-survives-reword).

### Constraints

-   gir never rewrites existing commits; squashing is left to `git rebase --autosquash` (`framework:SPEC.md`, Boundaries).
-   Rejections stay rejections; gir does not try to stop a user who pushes anyway (`ledger.md` A5, A6).
-   Out of scope: a warning at commit time (`ledger.md` A4).
-   Isolated: worktree `.worktrees/fix-unmatched-autosquash`, branch `fix-unmatched-autosquash`, from `3e10963` (operator answer, 2026-10-03).
-   Nothing under `.agents/` is committed; evidence from the phase-0 probes is copied into this bundle, not linked.

### Material empirical premises

<a id="p1-unmatched-survives"></a>
#### `p1-unmatched-survives`

-   Claim: With git 2.56.0, `git rebase --autosquash <base>` leaves a `squash!` or `fixup!` commit whose subject matches no commit in `<base>..HEAD` in place, reports success and exits `0`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, gir 0.2.1.
-   Consequence if false: there is no loop to fix.
-   Basis: [Verification](#verification-p1-unmatched-survives).

## Understand

### Relevant context

-   git 2.56.0's `--autosquash` documentation: the rest of the title after `squash! `, `fixup! ` or `amend! ` "is taken as a commit specifier, which matches a previous commit if it matches the title or the hash of that commit. If no commit matches fully, matches of the specifier with the start of commit titles are considered."
-   `probe-matching.py` with git 2.56.0 (`logs/probe-matching-20261003-1400-reword-fixed.log`) shows which subjects fold:
    -   Fold: an exact title; a plain string prefix of a title (`feat: add al` folds into `feat: add alpha`); a full or 7-character hash; a ref expression such as `feature~1`; chained prefixes (`fixup! fixup! X`); `amend!` and `fixup!` of one target in one rebase; a title shared by two commits.
    -   Left in place, with `Successfully rebased` and exit `0`: no matching title (`squash! wip`); `fixup! HEAD`, which resolves to the fixup itself; a target on the base branch, outside `main..HEAD`; a fixup older than its target; a target whose title was reworded by an earlier rebase to a title the subject is not a prefix of; a hash-form fixup after its target was reworded, because a reword changes the hash.
-   The first probe log (`logs/probe-matching-20261003-1354.log`) reported the reword case as folded only because the new title, `feat: add alpha`, starts with the old one; the re-run rewords to `feat: introduce a`.
-   In gir, `cc::check` (`src/cc/mod.rs`) judges one message with no context and returns `Kind::Autosquash` for the three prefixes; `lint::reject_recorded` (`src/cmd/lint.rs`) turns that into `fixup-unsquashed` with the hint `git rebase --autosquash <base>`.
    It is called per commit by `gir lint --range` (`Source::Range` in `src/cmd/lint.rs`) and by `hook::pre_push` (`src/cmd/hook.rs`).
    Knowing whether a commit can fold needs the ordered list of commits around it, which only those two callers have.
-   pre-push lints `<remote sha>..<local sha>`, or `<local sha> --not --remotes=<remote>` for a new branch: the commits not yet on the remote.
    `gir lint --range` lints the range the user gives.
-   `gir fixup`, `amend` and `squash` create the commit with `git commit --fixup=<sha>` (and `amend:`/`reword:`), so its subject is `<prefix> <target title>` (`src/cmd/fixup.rs`).
    `explicit_target` already refuses a target on the base branch; `find_base` tries `origin/HEAD`, `main`, `master` and the upstream.

### Assumptions

-   git resolves a hash or ref specifier only when the rest of the title contains no space, and tries the exact title before the hash and the hash before a prefix; source: recollection of git's `sequencer.c`; not verified; if false, gir's mirror of the rules misjudges subjects that fit several forms.
-   For pre-push, the commits being pushed stand in for the rebase todo: a target outside them is already on the remote, so folding it means rewriting published history; source: reasoning from the pre-push range; not verified against a user's actual `<base>`.

### Open questions

-   Which literal advice, for each kind of unmatched commit, leaves no autosquash commit behind when followed (c1)?
-   Can anything make a gir-made fixup fold after an out-of-band reword of its target (c4)? The probe shows neither the title form nor the hash form does; a body trailer git does not read (for example the target's `git patch-id`) could only let gir name the new target in its advice.
-   How faithfully must gir mirror git's rules: precedence, the no-space condition for revs, and ref expressions such as `feature~1` that git resolves when the rebase starts?

### UNDERSTAND gate

`ESTABLISHED`: the premise reproduces in a Probe this Task owns, git 2.56.0's matching behaviour is mapped for every case listed above, and the code paths that produce `fixup-unsquashed` and the ranges each caller has are known.
The two unverified assumptions and the open questions are carried into INVESTIGATE.

## Investigate

Probes: `probe-matching.py` (final run `logs/probe-matching-20261003-1431.log`) and `probe-advice.py` (`logs/probe-advice-20261003-1424.log`), git 2.56.0, Linux.
Superseded runs are kept as owned Evidence: `logs/probe-matching-20261003-1412-edges.log` and `-1428-chains.log` predate a classifier fix that missed a leftover `fixup!feat: add a`; `logs/probe-advice-20261003-1420.log` crashed in the probe's own todo editor on the reword case.

-   **Matching rules (assumption 1 and the faithfulness question): resolved.**
    Every case fits one rule: after stripping any chain of `fixup! `, `squash! ` and `amend! ` prefixes (mixed chains fold too), a commit folds if and only if some *earlier* commit in the rebased range matches the rest as an exact title, as a hash or rev expression that resolves to it (only when the rest has no space: `feature~1 x` is left), or as a plain string prefix of its title.
    Because all three forms only consider earlier commits in the range, the order git tries them in cannot change whether a commit folds; only which commit it folds into (an exact title only on a later commit still folds into an earlier prefix match).
    `fixup!feat: add a` (no space) is not an autosquash commit for git either, which matches gir's `structure::parse`.
    A rev is resolved when the rebase starts; `fixup! HEAD` resolves to the fixup itself and is left.
-   **pre-push range as the todo (assumption 2): accepted residual.**
    A target outside the pushed commits is on the remote already (or on the base for a new branch), so folding into it rewrites published history; `ledger.md` A5 and A6 make that a rejection with a reason either way.
    A user who rebases onto a base older than the remote tip could fold it, so the message must not say it is impossible, only that the target is published.
-   **Advice that works (c1): resolved.** Blaming, at the commit's parent, the lines the unmatched commit removes or changes names its intended target when those lines come from one commit:
    -   Target in the range (a reworded target; a hand-written `squash! wip` changing its lines): `git rebase -i <base>`, move the commit below the target and change `pick` to `fixup`; afterwards no autosquash commit is left and the tree is unchanged.
    -   Target on the base branch: no move is possible; rewording the commit into a normal Conventional Commit (`reword` in `git rebase -i <base>`) leaves no autosquash commit and the tree unchanged.
    -   No target found (the commit only adds files) or several (it changes lines of two commits): blame cannot name one; the advice has to stay generic (move it below the commit it belongs to, or reword it into a normal commit).
-   **Surviving a reword (c4): resolved as not possible without rewriting commits.**
    git reads only the title; after an out-of-band reword neither the title form nor the hash form folds (`logs/probe-matching-20261003-1431.log`), and a `git patch-id` trailer would stop matching as soon as an earlier fixup folds into the target.
    What gir can do instead is name the target by blame in its advice, as above.

### INVESTIGATE gate

`ESTABLISHED`: every Understand assumption and open question has a disposition above — three resolved, one accepted as residual with the message constraint it implies.

## Decide

Operator choices: `ledger.md` Q8–Q11.

### Approach

-   `gir lint --range` and pre-push already hold the commits they lint, newest first. For each autosquash commit, gir decides whether it will fold with the rule found in Investigate: strip the prefix chain, then look for an older commit in the same list whose title equals the rest, whose title starts with the rest, or, when the rest has no space, whose SHA `git rev-parse --verify --quiet <rest>^{commit}` returns. Only autosquash commits pay for this, and only a no-space rest costs a git call.
-   A commit that will fold keeps `fixup-unsquashed`, now with the concrete base.
-   A commit that will not fold gets `fixup-unmatched`. gir blames, at the commit's parent, the lines it removes or changes (`git diff -U0`, `git blame --porcelain -L`), reusing `gir fixup`'s hunk parsing and blame where they fit:
    -   one blamed commit in the list: the hint names it and says to move the commit below it in `git rebase -i <base>` and change `pick` to `fixup` (`squash` for `squash!`, `fixup -C` for `amend!`);
    -   one blamed commit outside the list (published): the message names it as already published; the hint says to reword the commit into a normal commit in `git rebase -i <base>`, and pre-push adds `or push it as is: git push --no-verify` (A10);
    -   none or several: a generic hint to move it below the commit it belongs to, or reword it into a normal commit.
-   Base: the first parent of the oldest commit in the list, as a 10-character ID; `<base>` when that commit has no parent. This is the start of a `lint --range A..B` range and the remote's commit for pre-push on a linear branch.
-   `gir explain fixup-unmatched` explains the three cases, why `git rebase --autosquash` leaves such a commit, that pushing anyway is the user's call (`git push --no-verify`), and that a reword of a target outside autosquash orphans its fixups (c4).
-   c4 is met by its alternative: the Task records, from the probes, that no subject git reads survives an out-of-band reword; no change to `gir fixup`.

### Proposed specification delta

-   `lint.md`, `range-unsquashed`: "`gir lint --range` SHALL reject a commit whose subject starts with `fixup! `, `squash! ` or `amend! ` and that `git rebase --autosquash` would fold into an older commit in the range with rule `fixup-unsquashed` and a hint `git rebase --autosquash <base>`, where `<base>` is the 10-character ID of the first parent of the oldest commit linted, or `<base>` when it has none."
-   `lint.md`, new `range-unmatched`: "`gir lint --range` SHALL reject such a commit that no older commit in the range matches — by exact title, title prefix, or (when the specifier after the prefixes has no space) a revision resolving to it — with rule `fixup-unmatched`. When blaming the lines it removes or changes names exactly one commit in the range, the hint SHALL name that commit and say to move it below it in `git rebase -i <base>`; when that commit is outside the range, the message SHALL name it as already published and the hint SHALL say to reword it into a normal commit; otherwise the hint SHALL say to move it below the commit it belongs to or reword it into a normal commit."
-   `hooks.md`, `pre-push-rejects`: add "commits that would not fold with rule `fixup-unmatched`, as `range-unmatched` describes, where a published target's hint also offers `git push --no-verify`".
-   `explain.md`, `known-pages`: add `fixup-unmatched`.

### Rejected alternatives

-   Separate `fixup-published` rule id; keeping `fixup-unsquashed` for both (A8).
-   Generic advice only, without blame (A9).
-   A `git patch-id` or hash-form subject to survive rewording (Investigate: neither survives).
-   A warning at commit time (A4).

### Consequences and residual uncertainty

-   `tests/lint.rs`'s `lint_range_rejects_each_unsquashed_autosquash_subject` lints a fixup without its target in the range; under the delta that is `fixup-unmatched`, so the test gains its target, and its `<base>` assertion becomes the concrete base.
-   A user who rebases onto an older base than gir's could fold a commit gir calls published; the message says published, not impossible.
-   Behaviour is probed on Linux with git 2.56.0 only.

### Verification strategy

-   End-to-end tests with real repositories in `tests/` for: a commit that folds (concrete base); one whose target is in the range but reworded; a hand-written `squash! wip`; a published target, through both `lint --range` and pre-push (`--no-verify` only in pre-push); none and several blamed commits; rev forms (full hash, `feature~1`, `HEAD`); a mixed prefix chain; the explain page.
-   `acceptance.py` in this bundle: for each unmatched case, run gir, follow its hint literally with `git rebase -i` and a todo editor, and check no autosquash commit is left and the tree is unchanged (c1, c2).
-   `cargo test`, `cargo clippy --all-targets -- -D warnings`, and the commit-msg latency budget test unchanged.

### DECIDE gate

`ESTABLISHED`: the approach follows from the Investigate results and the operator's choices, each part is feasible with git commands gir already runs, and every success criterion has a named check.

## Implement

Commit `eb6fdce`.

-   `lint::lint_recorded` (`src/cmd/lint.rs`) replaces the per-commit `reject_recorded`; `gir lint --range` and `hook::pre_push` both call it with their commit list, so the fold check, `fixup-unmatched`, the blame step and the concrete base live in one place. `reject_recorded` had no other caller and is gone.
-   `fixup::parse_hunks`, `Hunk` (`path`, `lines`) and a new `fixup::blame_at(rev, ..)` are crate-visible; `blame` delegates to `blame_at("HEAD", ..)`, so `gir fixup` is unchanged.
-   `src/explain.md`: a new `fixup-unmatched` page; the `fixup-unsquashed` page names both cases and what to do (c3).
-   Tests: `tests/lint.rs` gains five range tests and the existing unsquashed test now has its target in the range with a concrete base (Decide, Consequences); `tests/cli.rs` gains a pre-push test for a published target.

Deviations from Decide:

-   Found by `acceptance.py` (`logs/acceptance-20261003-1545.log`): following `change pick to fixup -C` for an `amend!` with no message after its title left the target with an empty message. The hint now says `fixup -C` only when the `amend!` carries a message, and `fixup` otherwise; `tests/lint.rs` covers both.
-   The blame step uses `gir fixup`'s hunk parsing as it is, so a pure insertion is blamed on the lines around it, not only removed or changed lines.
-   `lint::commits` now passes `--topo-order`, so in a list with merges an older commit is never listed before a newer one; for linear history the order is unchanged.

Checkpoint on `eb6fdce`: `cargo test --no-fail-fast` passed (16 test binaries), `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` clean, `acceptance.py` 7/7. Logs for the record are taken again in VERIFY.

### IMPLEMENT gate

`ESTABLISHED`: the change exists in `eb6fdce` and passes its tests and acceptance, so it can be evaluated.

## Verify

<a id="verification-p1-unmatched-survives"></a>
### Verification: `p1-unmatched-survives`

- Claim: [`p1-unmatched-survives`](#p1-unmatched-survives).
- Method: `probe-matching.py`, case `p1`, with git 2.56.0 on Linux: `feat: add a`, `squash! wip`, `squash! wip` on `feature`, then `git rebase --autosquash main` with `GIT_SEQUENCE_EDITOR=true`.
- Evidence considered: `logs/probe-matching-20261003-1354.log` and `logs/probe-matching-20261003-1400-reword-fixed.log` both show exit `0`, `Successfully rebased and updated refs/heads/feature.` and both `squash! wip` commits left; the same holds for `fixup!` in the cases with a target outside the range, a fixup older than its target, and a reworded target. The phase-0 probe (`ledger.md` A1) showed the same with gir 0.2.1's `lint --range` rejecting both again.
- Conclusion: `VERIFIED`; the rebase reports success and leaves the commits in place.
- Limitations: Linux and git 2.56.0 only.

All runs below are on the tree committed as `48bfb47`, Linux, git 2.56.0.

<a id="verification-c1-unmatched-detected"></a>
### Verification: `c1-unmatched-detected`

- Claim: [`c1-unmatched-detected`](#c1-unmatched-detected).
- Method: `acceptance.py` runs `gir lint --range` on each case, follows the `try:` hint literally with `git rebase` and a scripted todo editor, and checks the rebase exit code, that no `fixup!`/`squash!`/`amend!` subject is left, the exact resulting subjects, and an unchanged tree. End-to-end tests in `tests/lint.rs` check the rule, message and hint text for every case, and that the specifier forms git folds stay `fixup-unsquashed`.
- Evidence considered: `logs/acceptance-final-20261003-1615.log`, 7/7: a foldable `fixup!` with the concrete base; unmatched `fixup!`, `squash!`, `amend!` with and without a new message, each moved below the blamed target; generic advice followed by its reword alternative. `logs/test-final-20261003-1615.log`: 16 test binaries ok, none failed, including `lint_range_folds_every_specifier_form_git_matches`, `lint_range_unmatched_fixup_names_the_target_blame_finds`, `lint_range_unmatched_when_target_is_reworded_older_than_it_or_head` and `lint_range_unmatched_fixup_without_one_blamed_target_gets_generic_advice`. Contradicting evidence found and resolved before this run: `logs/acceptance-20261003-1545.log` showed `fixup -C` emptying a target's message (Implement, Deviations).
- Conclusion: `VERIFIED`; every unmatched case is told it will not fold and why, and every hint, followed literally, leaves no autosquash commit and the tree unchanged.
- Limitations: Linux and git 2.56.0 only. Generic advice cannot name the target, so following it literally means taking its reword branch; moving the commit needs the user to pick the target. A specifier git would match differently from gir's mirror was not found among the probed forms, but other forms were not exhausted.

<a id="verification-c2-published-target"></a>
### Verification: `c2-published-target`

- Claim: [`c2-published-target`](#c2-published-target).
- Method: `tests/cli.rs` `pre_push_rejects_a_fixup_of_a_pushed_commit_and_names_no_verify` pushes a target, then a fixup of it, through the installed hook; `tests/lint.rs` `lint_range_unmatched_fixup_of_a_published_target_says_to_reword_it` lints a range that starts after the target; `acceptance.py` follows the reword hint.
- Evidence considered: `logs/test-final-20261003-1615.log` (both tests ok): the push is rejected with `[fixup-unmatched] ... target <sha> feat: add a is already published` and `try: git rebase -i <remote sha>: reword it into a normal commit, or push it as is: git push --no-verify`; `lint --range` gives the same without `--no-verify`. `logs/acceptance-final-20261003-1615.log`: following the reword hint leaves `fix: correct a` and the tree unchanged.
- Conclusion: `VERIFIED`; the commit is still rejected, the message says the target is published, and pre-push leaves pushing anyway to the user.
- Limitations: Linux and git 2.56.0 only; "published" means outside the linted commits, as Investigate accepted.

<a id="verification-c3-explain-covers"></a>
### Verification: `c3-explain-covers`

- Claim: [`c3-explain-covers`](#c3-explain-covers).
- Method: `gir explain fixup-unsquashed` and `gir explain fixup-unmatched` from the built binary; `explain::tests::every_rule_and_type_has_a_page` lists `fixup-unmatched`.
- Evidence considered: `logs/explain-pages-20261003-1615.log`: the `fixup-unsquashed` page says a commit that matches no earlier commit, or whose target is published, is reported as `fixup-unmatched`, and to move it below its target in `git rebase -i <base>` or reword it into a normal commit; the `fixup-unmatched` page explains why the rebase leaves it, the move with the action per prefix, the reword for a published target, and `git push --no-verify`. The unit test passed in `logs/test-final-20261003-1615.log`.
- Conclusion: `VERIFIED`.
- Limitations: none that block completion.

<a id="verification-c4-survives-reword"></a>
### Verification: `c4-survives-reword`

- Claim: [`c4-survives-reword`](#c4-survives-reword), through its alternative: this Task records, from a probe, why a gir-made fixup cannot fold after an out-of-band reword without rewriting commits.
- Method: `probe-matching.py` cases "target reworded by an earlier rebase" in subject form (what `gir fixup` creates) and in hash form.
- Evidence considered: `logs/probe-matching-20261003-1431.log`: both are left in place after `git rebase --autosquash main`, because git reads only the title, the reworded title no longer starts with the old one, and the reword changed the hash. `logs/probe-matching-20261003-1354.log` folded only because its new title started with the old one, which the later runs corrected (Understand). Investigate records why a `git patch-id` trailer would not help.
- Conclusion: `VERIFIED` (alternative branch); the mitigation is the blame-named target in `fixup-unmatched`'s hint, verified under c1.
- Limitations: Linux and git 2.56.0 only.

### VERIFY gate

`ESTABLISHED`: c1–c4 are `VERIFIED` through the Verifications above; `cargo clippy --all-targets -- -D warnings` (`logs/clippy-20261003-1615.log`) and `cargo fmt --check` (`logs/fmt-20261003-1615.log`) are clean on the same tree. The spec delta is published in the terminal checkpoint.
