# TASK — `gir never targets a commit that autosquash cannot fold`

## Resume

**Contract version:** `2`

**State:** `INVESTIGATE`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE`

**Resume at:** Write `probe.py`: in throwaway repos on `target/debug/gir` built from this branch, repeat `p1-picker-lists-merges`, observe c1 (autosquash of a `fixup!` of a merge and of a side-branch commit), and observe what `gir fixup` does today for a staged line blamed on a conflict-resolving merge.

**Open obligations:** (1) `p1-picker-lists-merges` repeated on this branch — blocks `INVESTIGATE` exit.
(2) Whether side-branch commits are also left out (`--first-parent`) — blocks `DECIDE`; settled from the evidence of `c1-autosquash-behaviour`.
(3) How automatic selection treats a staged line last changed by a merge commit (untraced, refused, or traced past the merge) — blocks `DECIDE`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-ask-branch-commit`, `framework:spec/fixup.md#req-fixup-explicit-target`, `framework:spec/fixup.md#req-fixup-explicit-target-on-branch`, `framework:spec/fixup.md#req-fixup-explicit-target-after-base`, `framework:spec/fixup.md#req-fixup-staged-line-target`, `framework:spec/fixup.md#req-fixup-base-limit`
- Proposed delta: `ask-branch-commit` lists no commit that `git rebase --autosquash <base>` cannot fold (at least merge commits); explicit targets refuse such a commit; automatic selection never selects one. Which commits that covers, and the exact wording and messages, are settled in `DECIDE`.
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

`NONE` yet.

## Decide

`NONE` yet.

## Implement

`NONE` yet.

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
