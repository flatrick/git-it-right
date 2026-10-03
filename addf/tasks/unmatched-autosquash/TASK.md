# TASK — `an unmatched autosquash commit gets advice that works`

## Resume

**Contract version:** `2`

**State:** `DECIDE`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE`

**Resume at:** Settle with the operator the rule ids, whether advice names the target found by blame, how the published-target message words pushing anyway, and the base shown in hints; then write the proposed spec delta.

**Open obligations:** The selected approach, its spec delta and its verification strategy recorded under Decide — blocks the DECIDE gate.

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

## Specification impact

- Current contract: `framework:spec/lint.md#req-lint-range-unsquashed`, `framework:spec/hooks.md#req-hooks-pre-push-rejects`, `framework:spec/explain.md#req-explain-known-pages`, `framework:spec/fixup.md#req-fixup-create-commit`, `framework:SPEC.md` (Boundaries)
- Proposed delta: `PENDING` — settled in DECIDE.
- Terminal publication: `PENDING`

## Define

### Objective

`A user whose push or range lint is rejected as fixup-unsquashed is never sent into a loop: either following the advice folds the commit, or gir says it cannot be folded and what to do instead.`

DEFINE gate: `ESTABLISHED` — the operator agreed to the objective, scope and success criteria c1–c4 (`ledger.md` Q7, A7).

### Success criteria

<a id="c1-unmatched-detected"></a>
#### `c1-unmatched-detected`

-   Claim: `gir lint --range` and pre-push reject an autosquash commit that `git rebase --autosquash <base>` would not fold with a message that says it will not fold and why, and advice that, followed literally, leaves no autosquash commit behind.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user keeps looping between the rejection and a rebase that reports success.
-   Basis: none yet; Task in DEFINE.

<a id="c2-published-target"></a>
#### `c2-published-target`

-   Claim: When an autosquash commit's target is already on the base branch or the remote, `gir lint --range` and pre-push still reject it, and the message says the target is already published and that pushing it anyway is the user's call.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user is told to run a rebase that cannot fold the commit, or is not told why.
-   Basis: none yet; Task in DEFINE.

<a id="c3-explain-covers"></a>
#### `c3-explain-covers`

-   Claim: `gir explain fixup-unsquashed` describes the unmatched case and the published-target case, and what to do in each.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the long-form help still sends the user into the loop.
-   Basis: none yet; Task in DEFINE.

<a id="c4-survives-reword"></a>
#### `c4-survives-reword`

-   Claim: A commit made by `gir fixup`, `gir amend` or `gir squash` still folds into its target with `git rebase --autosquash <base>` after the target was reworded, or this Task records, from a probe, why that cannot be done without rewriting commits.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: gir's own fixup workflow keeps producing commits that cannot fold.
-   Basis: none yet; Task in DEFINE.

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

## Verify

<a id="verification-p1-unmatched-survives"></a>
### Verification: `p1-unmatched-survives`

- Claim: [`p1-unmatched-survives`](#p1-unmatched-survives).
- Method: `probe-matching.py`, case `p1`, with git 2.56.0 on Linux: `feat: add a`, `squash! wip`, `squash! wip` on `feature`, then `git rebase --autosquash main` with `GIT_SEQUENCE_EDITOR=true`.
- Evidence considered: `logs/probe-matching-20261003-1354.log` and `logs/probe-matching-20261003-1400-reword-fixed.log` both show exit `0`, `Successfully rebased and updated refs/heads/feature.` and both `squash! wip` commits left; the same holds for `fixup!` in the cases with a target outside the range, a fixup older than its target, and a reworded target. The phase-0 probe (`ledger.md` A1) showed the same with gir 0.2.1's `lint --range` rejecting both again.
- Conclusion: `VERIFIED`; the rebase reports success and leaves the commits in place.
- Limitations: Linux and git 2.56.0 only.
