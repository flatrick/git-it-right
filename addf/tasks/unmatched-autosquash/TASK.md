# TASK — `an unmatched autosquash commit gets advice that works`

## Resume

**Contract version:** `2`

**State:** `DEFINE`

**State path:** `DEFINE`

**Resume at:** DEFINE dialogue with the operator: agree the objective, scope and success criteria (questions in `ledger.md` from Q4 on).

**Open obligations:** Objective, scope and success criteria agreed with the operator — blocks the DEFINE gate.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took (Q1–Q3, from `ledger/unmatched-autosquash-20261003.md`), and its DEFINE dialogue.

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
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, gir 0.2.1.
-   Consequence if false: there is no loop to fix.
-   Basis: one phase-0 probe on 2026-10-03 (`ledger.md` A1); to be re-run as a Probe owned by this Task in UNDERSTAND.
