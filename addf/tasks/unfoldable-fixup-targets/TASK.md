# TASK — `gir never targets a commit that autosquash cannot fold`

## Resume

**Contract version:** `2`

**State:** `DEFINE`

**State path:** `DEFINE`

**Resume at:** Ask the operator to agree that the Objective, Success criteria and Constraints below are viable and desirable (DEFINE gate).

**Open obligations:** (1) DEFINE gate: operator agreement on objective, scope and success criteria — blocks `DEFINE -> UNDERSTAND`.
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

`NONE` yet.

### Assumptions

-   `NONE` yet.

### Open questions

-   See Open obligations.

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
