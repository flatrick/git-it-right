# TASK — `ledger threads and several active Tasks`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Settle `p1-single-task-assumption`: read every current addf file that mentions the Ledger, the active Task, or `INDEX.md`'s Task list, and record the result under Understand.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the root Ledger's ten fixup findings, moved here unchanged, and the questions that shaped this Task with the operator's answers.

## Specification impact

- Current contract: `framework:SPEC.md`, read only to confirm it specifies gir's product behavior and not addf's own mechanics.
- Proposed delta: `NONE`. This Task changes addf's framework mechanics, which `rules/self-improvement-log.md` records instead.
- Terminal publication: `PENDING`

## Define

### Objective

addf supports exploration that leads to several Tasks or to none, and several active Tasks at once, without losing or mixing entries.

### Success criteria

<a id="c1-ledger-threads"></a>
#### `c1-ledger-threads`

-   Claim: Pre-Task exploration lives in append-only thread files under `ledger/`; the root `LEDGER.md` no longer exists, and no current addf file outside `.archive/` still refers to it.
-   State: `UNVERIFIED`
-   Scope: the framework root on this branch.
-   Consequence if false: two places hold exploration, and an agent following a stale reference writes to the old one.
-   Basis: pending check.

<a id="c2-thread-handoff"></a>
#### `c2-thread-handoff`

-   Claim: Work control says that starting a Task copies the thread entries it takes into its bundle and appends an entry to the thread naming the Task; that a thread is archived only once every entry has a disposition; and that `INDEX.md` lists open threads.
-   State: `UNVERIFIED`
-   Scope: `skills/work-control.md`, `skills/stewardship.md`, `templates/INDEX.md` and `INDEX.md`.
-   Consequence if false: untasked findings are lost or buried in an unrelated Task.
-   Basis: pending check.

<a id="c3-several-active-tasks"></a>
#### `c3-several-active-tasks`

-   Claim: Work control allows at most 3 active Tasks; when more than one is active, resuming asks the operator which one; Tasks that overlap in spec modules or files declare the overlap and their order in Constraints; and the Isolate question is asked once per Task.
-   State: `UNVERIFIED`
-   Scope: `skills/work-control.md` and `CORE.md`.
-   Consequence if false: concurrent Tasks are allowed without the rules the operator chose, or are still forbidden.
-   Basis: pending check.

<a id="c4-checker-layout"></a>
#### `c4-checker-layout`

-   Claim: `scripts/check-capsule` validates the new layout (thread files under `ledger/`, no root `LEDGER.md`) and its test suite passes.
-   State: `UNVERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: the checker rejects the new layout or silently stops checking ledger content.
-   Basis: pending check.

<a id="c5-findings-thread"></a>
#### `c5-findings-thread`

-   Claim: The ten fixup findings in this Task's `ledger.md` exist unchanged in `ledger/fixup-review-20261002.md` as open entries.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the findings stay buried in this unrelated Task.
-   Basis: pending check.

<a id="c6-self-improvement"></a>
#### `c6-self-improvement`

-   Claim: A `SELF-IMPROVEMENT/` entry records this framework change and the friction that caused it.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the framework change is not traceable through its own log.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Out of scope: changing the lifecycle States, and rewriting archived Tasks' `ledger.md` files.
-   Each State change is its own commit (`skills/work-control.md`, Work and transition).

### Material empirical premises

<a id="p1-single-task-assumption"></a>
#### `p1-single-task-assumption`

-   Claim: Besides `skills/work-control.md`'s "Maintain one Task as the resumable execution cursor", no current addf file assumes a single active Task.
-   State: `UNVERIFIED`
-   Scope: the framework root on this branch.
-   Consequence if false: some rule still forbids or breaks concurrent Tasks after the amendment.
-   Basis: pending check in `UNDERSTAND`.

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, scope and success criteria are right, as written here (`ledger.md`).

## Understand

### Relevant context

`PENDING`

### Assumptions

-   `NONE` yet.

### Open questions

-   `NONE` yet.

### Deferred verification

-   `NONE` yet.

## Investigate

`PENDING`

## Decide

`PENDING`

## Implement

`PENDING`

## Verify

`PENDING`

## Learn

### Technical

`PENDING`

### Process

`PENDING`

## Retention and promotion

`PENDING`

## Archive readiness

`PENDING`

## Terminal record

### Summary

`PENDING`

### Gate basis

`PENDING`
