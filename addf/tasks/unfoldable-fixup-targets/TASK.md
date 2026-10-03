# TASK — `gir never targets a commit that autosquash cannot fold`

## Resume

**Contract version:** `2`

**State:** `IMPLEMENT`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT`

**Resume at:** Write the tests for c2–c4 in `tests/fixup.rs` and `tests/fixup_modes.rs`, and log them failing on the current code.

**Open obligations:** (1) Tests for c2–c4 fail first, then pass — blocks `IMPLEMENT` exit.
(2) `spec/fixup.md` delta written — blocks `VERIFY` exit (c6).

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `probe.py` - Probe: the picker after a `--no-ff` merge, what a plain autosquash folds, and what explicit and automatic targets gir accepts today.
-   `logs/probe-02b4061-20261003-1623.log` - Evidence: `probe.py` on the build of `02b4061` (see Investigate).
-   `logs/test-red-d9ab2b0-20261003-1654.log` - Evidence: the new tests on `d9ab2b0`'s code, before the change.
-   `logs/test-final-20261003-1655.log`, `logs/clippy-20261003-1655.log`, `logs/fmt-20261003-1655.log`, `logs/fmt-20261003-1700.log`, `logs/test-fixup-20261003-1700.log` - Evidence: tests, clippy and formatting on the change (see Implement).
-   `logs/probe-new-20261003-1702.log`, `logs/probe-new-20261003-1705.log` - Evidence: `probe.py` on the new build; the first is superseded (see Implement).

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-ask-branch-commit`, `framework:spec/fixup.md#req-fixup-explicit-target`, `framework:spec/fixup.md#req-fixup-explicit-target-on-branch`, `framework:spec/fixup.md#req-fixup-explicit-target-after-base`, `framework:spec/fixup.md#req-fixup-staged-line-target`, `framework:spec/fixup.md#req-fixup-base-limit`
- Proposed delta: `ask-branch-commit` lists only non-merge commits; a new `explicit-target-not-merge` refuses a merge target; a new `merge-limit` refuses a hunk with a line last changed by a merge, and an insertion whose neighbouring lines all were. Wording under Decide.
- Terminal publication: `PENDING`

## Define

### Objective

`gir fixup`, `gir amend`, `gir squash` and `gir reword` never offer, accept or select a target commit that a default `git rebase -i --autosquash <base>` would leave unfolded.

DEFINE gate: `ESTABLISHED` — the operator agreed to the objective, scope and success criteria c1–c6 (`ledger.md` Q6, A6).

### Success criteria

<a id="c1-autosquash-behaviour"></a>
#### `c1-autosquash-behaviour`

-   Claim: For a branch with a `--no-ff` merge of a side branch, it is observed (not inferred) whether `git rebase -i --autosquash <base>` folds a `fixup!` of the merge commit, and whether it folds a `fixup!` of a commit from the side branch.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, throwaway repositories.
-   Consequence if false: the fix leaves out the wrong commits: it either still offers unfoldable targets or hides foldable ones.
-   Basis: pending; A7 states the merge case as inferred only.

<a id="c2-picker"></a>
#### `c2-picker`

-   Claim: The commit picker (`ask-branch-commit`) lists none of the commits `c1-autosquash-behaviour` shows cannot be folded, and still lists up to 20 of the others.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user can still pick a target that never folds.
-   Basis: pending.

<a id="c3-explicit-target"></a>
#### `c3-explicit-target`

-   Claim: `gir fixup COMMIT` (and `amend`, `squash`, `reword`) given such a commit refuses on stderr with exit `2` and creates no commit.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: a named unfoldable target is still accepted.
-   Basis: pending.

<a id="c4-automatic-selection"></a>
#### `c4-automatic-selection`

-   Claim: Automatic selection, including `--split`, never selects such a commit, for example when a merge's conflict resolution last changed a staged line; what it does instead is settled in `DECIDE`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: gir creates an unfoldable `fixup!` without asking.
-   Basis: pending.

<a id="c5-no-regression"></a>
#### `c5-no-regression`

-   Claim: `cargo test` and `cargo clippy --all-targets -- -D warnings` pass, and the change follows `rules/os-agnostic-code.md`.
-   State: `UNVERIFIED`
-   Scope: this branch; Linux run, Windows by inspection and CI.
-   Consequence if false: the change breaks existing behaviour or another OS.
-   Basis: pending.

<a id="c6-spec"></a>
#### `c6-spec`

-   Claim: `spec/fixup.md` states the new behaviour for the picker, explicit targets and automatic selection, and matches the code.
-   State: `UNVERIFIED`
-   Scope: this branch at completion.
-   Consequence if false: the specification no longer describes gir.
-   Basis: pending.

### Constraints

-   Isolated workspace and line of development, chosen by the operator on 2026-10-03: worktree `.worktrees/unfoldable-fixup-targets`, branch `unfoldable-fixup-targets`, started from `ledger-settle-open-entries` at `0f4aac1`.
-   No other Task is active, so there is no overlap to order.
-   Scope chosen by the operator: the picker, explicit targets and automatic selection.
    Whether `--first-parent` applies is decided after INVESTIGATE, not presumed.
-   `rules/os-agnostic-code.md`: behaviour is the same on Windows and Linux.
-   "Cannot fold" means under a default `git rebase -i --autosquash <base>`; `--rebase-merges` is out of scope (operator, 2026-10-03).
-   When no base is found, the same filter applies to the commits on `HEAD` that the picker lists.
-   An explicit unfoldable target is refused with exit `2`, not warned about.

### Material empirical premises

<a id="p1-picker-lists-merges"></a>
#### `p1-picker-lists-merges`

-   Claim: On `fc55fbb`, after `git merge --no-ff side`, the picker lists the merge commit and the side branch's commits, because `pick_branch_commit` (`src/cmd/fixup.rs:162`) runs `rev-list --max-count=20 <base>..HEAD` with no filter.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0.
-   Consequence if false: Q7 describes a defect that is not there.
-   Basis: A7 reproduced it up to the picker list on `d246ab2`; not yet repeated on this branch.

## Understand

### Relevant context

A target commit reaches `commit()` (`src/cmd/fixup.rs`) by one of four paths, read from the code at `5131456`:

-   **Picker**, `pick_branch_commit`: `rev-list --max-count=20 <base>..HEAD`, or `HEAD` with no base, unfiltered.
    Used by `gir reword` without a commit, by `gir fixup` when a staged file cannot be traced, and by `--split` for each untraced file (`split.rs`).
-   **Explicit target**, `explicit_target`: checks only that the commit is `HEAD` or its ancestor and, with a base, not reachable from it.
-   **Automatic selection**, `trace`: `blame` runs `git blame -L n,n HEAD` per staged line, without `--first-parent`.
    The allowed set is `rev-list <base>..HEAD`, which includes merges and side-branch commits; with no base, any blamed commit is allowed.
    One target is used directly.
    Several targets go to the `ask-several` list or to the `multiple-targets` refusal, and either may name a merge.
-   **Split**: each round commits for the shas `trace` found; an insertion between two commits' lines is asked about with those two (`neighbours`).

`git blame` without `--first-parent` blames a line merged cleanly from a side branch on the side-branch commit.
It blames a merge commit only for a line that the merge result changed against both parents, such as a conflict resolution.

Earlier evidence: `history:tasks/autosquash-review-fixes/logs/probe-git-rules-20261003-1726.log` (git 2.56.0, Linux) shows a plain `git rebase -i --autosquash` dropping the merge from its todo and leaving `fixup! Merge branch 'side'` unfolded, while the side-branch commit `feat: side` stays, linearised.
No fixup of a side-branch commit was tried there.
Since that Task, `gir lint --range` and pre-push leave merges out of the fold targets (`src/cmd/lint.rs`, `merges`), so a `fixup!` that gir creates for a merge today is then rejected by gir's own pre-push as `fixup-unmatched`.

Tests: `tests/fixup.rs` (targets, base, trace, split), `tests/fixup_modes.rs` (modes and picker), on the `Repo` helper in `tests/common/mod.rs`.

### Assumptions

-   A `fixup!` of a side-branch commit folds under a plain autosquash rebase; inferred from the side commit staying in the todo; unverified; if false, side-branch commits must be left out too (`--first-parent`).
-   A merge is the only kind of commit in `<base>..HEAD` that a plain rebase drops; inferred from how `git rebase` builds its todo (it skips merges, and also skips commits already upstream by patch ID, which do not apply inside `<base>..HEAD` of the branch being rebased); unverified beyond merges.

### Open questions

-   See Open obligations (2) and (3).
-   Should `ask-several` and `multiple-targets` leave a merge out of the list, or must the whole hunk be refused when one of its lines is blamed on a merge? Part of obligation (3).

### UNDERSTAND gate

`ESTABLISHED`: every path by which a target is chosen or checked is named, with the earlier evidence for merges. The two assumptions and the open questions carry into INVESTIGATE.

### Deferred verification

-   `NONE`.

## Investigate

`probe.py` on `target/debug/gir` built from `02b4061`, git 2.56.0, Linux (`logs/probe-02b4061-20261003-1623.log`).
Each repository has `main` (`chore: base`) and `topic` with `feat: a`, a `--no-ff` merge of `side` (`feat: side`), and `feat: b`; `side` forks from `main` in one run and from `feat: a` in the other.

-   `p1-picker-lists-merges`: in both runs, `gir reword` without a commit lists `Merge branch 'side' into topic` and `feat: side` among its four commits.
-   `c1-autosquash-behaviour`: in both runs, `git rebase -i --autosquash main` leaves `fixup! Merge branch 'side' into topic` unfolded on the linearised branch, and folds `fixup! feat: side` into `feat: side` (`s.txt` becomes `S`).
    This agrees with `history:tasks/autosquash-review-fixes/logs/probe-git-rules-20261003-1726.log` for the merge.
-   Explicit target: `gir fixup --dry-run <merge>` exits `0` and names the merge.
-   Automatic selection: with `f.txt` line 2 changed on both branches and the conflict resolved to `two-merged`, `git blame` names the merge for line 2.
    Staging a change to that line, `gir fixup --dry-run` and `gir fixup --split --dry-run` both select the merge.

Dispositions:

-   Assumption "a side-branch commit folds": resolved, true. Leaving out side-branch commits (`--first-parent`) would hide targets that fold, so obligation (2) is resolved: leave out merge commits only.
-   Assumption "a merge is the only commit in `<base>..HEAD` a plain rebase drops": not fully resolved.
    `git rebase <upstream>` also drops a commit whose patch is already in `<upstream>`, for example one cherry-picked to `main` after the branch forked; a `fixup!` of it would then not fold.
    gir does not compare patch IDs today; the operator decides in DECIDE whether this is in scope.
-   Obligation (3), how automatic selection treats a line blamed on a merge: the evidence shows it must change; the choice of behaviour goes to DECIDE.

### INVESTIGATE gate

`ESTABLISHED`: p1 and c1 are observed, every assumption has a disposition, and the remaining choices (merge-blamed lines, cherry-picked commits) are decisions, not uncertainties a probe can settle.

## Decide

Leave out merge commits, and only merge commits (Investigate; `ledger.md` Q7–Q9).

-   **Picker:** `pick_branch_commit` runs `rev-list --no-merges --max-count=20 <range>`, so it lists up to 20 non-merge commits.
-   **Explicit target:** after the existing checks, `explicit_target` refuses a merge with ``gir: `COMMIT` is a merge commit, which a rebase drops; pass the commit the change belongs to``, exit `2`.
    A merge on the base branch keeps its existing `already on the base branch` refusal.
-   **Automatic selection:** `trace` learns which blamed commits are merges with one `git rev-list --merges --no-walk --stdin` call, made only when a hunk names any commit.
    After the `base-limit` check, a hunk with a line blamed on a merge is refused with `<place> was last changed by the merge <sha>, which a rebase drops; commit it normally, or pass one: gir <subcommand> <commit>`; a pure insertion is refused only when every neighbouring line is blamed on a merge, and otherwise drops the merge from its candidates.
    `trace` takes the `Mode` to name the subcommand.
    `--split` gets its hunks from `trace`, so it refuses the same way.
-   The merge lookup is a new `git::merges(ids)` in `src/git.rs`.
    `lint::merges` makes the same git call over `(sha, subject)` pairs; it is left as is, to keep the diff to this Task.
-   **Spec:** `ask-branch-commit` says "non-merge commits"; a new `explicit-target-not-merge` after `explicit-target-after-base`; a new `merge-limit` after `base-limit`.

Rejected: `--first-parent` (it hides side-branch commits that fold, Investigate); treating a merge-blamed line as untraced or dropping it from a modified hunk (operator, `ledger.md` A8); cherry-picked commits (moved to the thread as Q17).

Residual: a commit cherry-picked to the base branch can still be targeted (thread Q17); `--rebase-merges` is out of scope (Constraints).

Verification strategy: tests first in `tests/fixup.rs` and `tests/fixup_modes.rs`, each failing on `d99d543`'s code for the reason in c2–c4, then passing; `probe.py` re-run against the new build; `cargo test` and `cargo clippy --all-targets -- -D warnings` for c5; `spec/fixup.md` compared with the code for c6.

### DECIDE gate

`ESTABLISHED`: the approach follows from the Investigate evidence and the operator's choices, each part uses git commands gir already runs, and every success criterion has a named check.

## Implement

Code in `ebdd50d`:

-   `src/cmd/fixup.rs`: `pick_branch_commit` adds `--no-merges`; `explicit_target` refuses a merge; `trace` takes the `Mode`, blames every hunk first, asks `merges` once for all blamed commits, and after the `base-limit` check refuses a merge-blamed hunk or drops the merge from an insertion's candidates.
-   `tests/fixup.rs`: seven tests: the picker leaves out the merge but lists the side commit; an explicit merge is refused by `fixup`, `amend`, `squash` and `reword`; an explicit side-branch commit is accepted; a merge-resolved line is refused by `fixup`, `fixup --split` and `amend`; a hunk with one merge line among others is refused; an insertion between a merge line and a topic line targets the topic commit; an insertion between a base line and a merge line is refused.

Deviation from Decide: `merges` is a private function in `src/cmd/fixup.rs`, beside `lint::merges` in `src/cmd/lint.rs`, not a new `git::merges`; both make the same call, each in its own module, as `lint` already did.
`trace` now blames all hunks before checking any, so an early refusal no longer skips the remaining blames; the order of checks and their results are unchanged.

Checkpoints:

-   Tests first: `logs/test-red-d9ab2b0-20261003-1654.log`: the six tests for c2–c4 fail on `d9ab2b0`'s code for the expected reasons (the picker lists the merge; the merge is accepted as an explicit target and selected automatically; the mixed hunk and the insertion name the merge as a second target); the side-branch test passes there, as it should.
-   `logs/test-fixup-20261003-1700.log`: `cargo test --test fixup`, 35 passed, on the committed tree.
-   `logs/test-final-20261003-1655.log`: `cargo test --no-fail-fast`, every suite passes, on the tree before the two assertions were reformatted (layout only); `logs/clippy-20261003-1655.log`: clean; `logs/fmt-20261003-1655.log` flagged two new test assertions, reformatted by hand; `logs/fmt-20261003-1700.log`: clean.
-   `logs/probe-new-20261003-1705.log`: `probe.py` on the new build: the picker lists the side commit but not the merge; the explicit merge, automatic selection and `--split` are refused with exit `2`.
    `logs/probe-new-20261003-1702.log` is the same run before `probe.py` required exit `0` for "picks the merge", so its two `True` lines only matched the merge's ID inside the refusal.

Specification delta for `spec/fixup.md`, to publish at completion:

-   `ask-branch-commit`: "list up to 20 of the newest non-merge commits after the base commit, or on `HEAD` when no base is found", the rest unchanged.
-   New `explicit-target-not-merge`, after `explicit-target-after-base`: `gir fixup COMMIT` SHALL refuse a merge commit with ``gir: `COMMIT` is a merge commit, which a rebase drops; pass the commit the change belongs to`` on stderr and exit `2`, creating no commit.
-   New `merge-limit`, after `base-limit`: automatic selection SHALL refuse a hunk that replaces any line last changed by a merge commit with `<path>:<line> was last changed by the merge <sha>, which a rebase drops; commit it normally, or pass one: gir <subcommand> <commit>` on stderr and exit `2`. A pure insertion SHALL be refused so only when none of its neighbouring lines was last changed by an eligible non-merge commit; otherwise the merge SHALL NOT be a target.

### IMPLEMENT gate

`ESTABLISHED`: the change exists in `ebdd50d`, its tests failed before and pass now, and the spec wording is ready for VERIFY to compare with the code.

## Verify

`NONE` yet.

## Learn

`NONE` yet.

## Retention and promotion

`NONE` yet.

## Archive readiness

`NONE` yet.

## Terminal record

`NONE` yet.
