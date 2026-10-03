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

Draft; not yet agreed with the operator.

### Success criteria

Draft; not yet agreed with the operator.

<a id="c1-unmatched-detected"></a>
#### `c1-unmatched-detected`

-   Claim: `gir lint --range` and pre-push tell an autosquash commit with no target in the range apart from one with a target, and give the unmatched one advice that, followed literally, leaves no autosquash commit behind.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the user keeps looping between the rejection and a rebase that reports success.
-   Basis: none yet; Task in DEFINE.

### Constraints

-   gir never rewrites existing commits; squashing is left to `git rebase --autosquash` (`framework:SPEC.md`, Boundaries).
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
