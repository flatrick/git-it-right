# TASK — `ledger threads and several active Tasks`

## Resume

**Contract version:** `2`

**State:** `LEARN`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN`

**Resume at:** Record Learn, Retention and promotion and Archive readiness, then hand over to Stewardship for `COMPLETED`.

**Open obligations:** `NONE`

## Owned artifacts

-   `ledger.md` - the root Ledger's ten fixup findings, moved here unchanged, and the questions that shaped this Task with the operator's answers.
-   `logs/p1-references-20261002-0943.log` - Evidence: every current mention of the Ledger, the active Task and resuming, searched in `UNDERSTAND`.
-   `logs/c1-references-20261002-1000.log` - Evidence: the root `LEDGER.md` is absent, and every current mention of it.
-   `logs/c2-c3-rules-20261002-1000.log` - Evidence: the thread, handoff, archival and several-Task rules in the Skills and index.
-   `logs/c4-tests-20261002-1000.log` - Evidence: the checker's test suite at `32db650`.
-   `logs/c4-check-capsule-20261002-1000.log` - Evidence: the checker on this repository at `32db650`.
-   `logs/c4-mutations-20261002-1001.log` - Evidence: removing each new check makes its test fail.
-   `logs/c5-c6-20261002-1000.log` - Evidence: the findings in the thread compared with this Task's `ledger.md` and with the root Ledger at `e3e3c3b`; the SELF-IMPROVEMENT entry exists.

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

-   Claim: Pre-Task exploration lives in append-only thread files under `ledger/`; the root `LEDGER.md` no longer exists, and no current addf file outside `.archive/` tells anyone to read or write it. Mentions that name it as retired, or that are history, are allowed (narrowed by the operator in `VERIFY`; `ledger.md`).
-   State: `VERIFIED`
-   Scope: the framework root on this branch.
-   Consequence if false: two places hold exploration, and an agent following a stale reference writes to the old one.
-   Basis: [Verification](#verification-c1-ledger-threads).

<a id="c2-thread-handoff"></a>
#### `c2-thread-handoff`

-   Claim: Work control says that starting a Task copies the thread entries it takes into its bundle and appends an entry to the thread naming the Task; that a thread is archived only once every entry has a disposition; and that `INDEX.md` lists open threads.
-   State: `VERIFIED`
-   Scope: `skills/work-control.md`, `skills/stewardship.md`, `templates/INDEX.md` and `INDEX.md`.
-   Consequence if false: untasked findings are lost or buried in an unrelated Task.
-   Basis: [Verification](#verification-c2-thread-handoff).

<a id="c3-several-active-tasks"></a>
#### `c3-several-active-tasks`

-   Claim: Work control allows at most 3 active Tasks; when more than one is active, resuming asks the operator which one; Tasks that overlap in spec modules or files declare the overlap and their order in Constraints; and the Isolate question is asked once per Task.
-   State: `VERIFIED`
-   Scope: `skills/work-control.md` and `CORE.md`.
-   Consequence if false: concurrent Tasks are allowed without the rules the operator chose, or are still forbidden.
-   Basis: [Verification](#verification-c3-several-active-tasks).

<a id="c4-checker-layout"></a>
#### `c4-checker-layout`

-   Claim: `scripts/check-capsule` validates the new layout (thread files under `ledger/`, no root `LEDGER.md`) and its test suite passes.
-   State: `VERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: the checker rejects the new layout or silently stops checking ledger content.
-   Basis: [Verification](#verification-c4-checker-layout).

<a id="c5-findings-thread"></a>
#### `c5-findings-thread`

-   Claim: The ten fixup findings in this Task's `ledger.md` exist unchanged in `ledger/fixup-review-20261002.md` as open entries.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: the findings stay buried in this unrelated Task.
-   Basis: [Verification](#verification-c5-findings-thread).

<a id="c6-self-improvement"></a>
#### `c6-self-improvement`

-   Claim: A `SELF-IMPROVEMENT/` entry records this framework change and the friction that caused it.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: the framework change is not traceable through its own log.
-   Basis: [Verification](#verification-c6-self-improvement).

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Out of scope: changing the lifecycle States, and rewriting archived Tasks' `ledger.md` files.
-   Each State change is its own commit (`skills/work-control.md`, Work and transition).

### Material empirical premises

<a id="p1-single-task-assumption"></a>
#### `p1-single-task-assumption`

-   Claim: Besides `skills/work-control.md`'s "Maintain one Task as the resumable execution cursor", no current addf file assumes a single active Task.
-   State: `VERIFIED`
-   Scope: the framework root at `db33325`, excluding `.archive/`, `tasks/`, `evidence/` and `SELF-IMPROVEMENT/`.
-   Consequence if false: some rule still forbids or breaks concurrent Tasks after the amendment.
-   Basis: [Verification](#verification-p1-single-task-assumption).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, scope and success criteria are right, as written here (`ledger.md`).

## Understand

### Relevant context

-   Files that name the root Ledger and change under `c1-ledger-threads`: `CORE.md` (Is this Task-worthy?, Route), `skills/work-control.md` (Isolate, Ledger, Define, Start or resume, Exit check), `ADOPT.md` (instance data, Adoption invariants), `INDEX.md` and `templates/INDEX.md` (Ledger section, invariants), `templates/LEDGER.md`, `templates/SELF-IMPROVEMENT.md`, `rules/self-improvement-log.md` (Scope), `scripts/check-capsule` (`check_placeholders`) and `scripts/tests/test_check_capsule.py`.
-   Files that assume one Task, for `c3-several-active-tasks`: only `skills/work-control.md` — its opening line, "the active Task carries the proposal" under Define, "Work control owns the active cursor" under Responsibility handoff, and Isolate's "unless the operator already answered that question earlier in this conversation".
-   `INDEX.md` already lists Active Tasks as a list, and `check_index` already checks every active Task's entry, so neither assumes one Task.
-   `check_placeholders` checks `LEDGER.md` strictly, including inline code; files under the directories it lists are checked only in prose with inline code stripped. A thread quoting git output such as `<commit>` needs the prose mode.
-   `archived_checked` covers only `.archive/tasks/` and `.archive/open-claims/`, so an archived thread under `.archive/ledger/` is not checked, like other archived material outside Task bundles.
-   Stewardship owns archival and index removal; it has no Ledger rule today.

### Assumptions

-   A thread entry can be referred to by its position among the thread's `Q:` entries, because the file is append-only and positions never change; source: the existing append-only contract; not verified by any check; if false, a disposition could name the wrong entry.

### Open questions

-   `NONE`. The remaining choices (template name, disposition format, whether a thread taken whole by one Task is moved instead of copied, what the checker enforces) are design choices for `DECIDE`, not uncertainties a probe can settle.

### Deferred verification

-   `NONE`.

### UNDERSTAND gate

`ESTABLISHED`: every file that names the Ledger or assumes one Task is listed above, and `p1-single-task-assumption` is verified, so the change set is known.

## Investigate

No probe is needed. Dispositions:

-   Which files change, and whether anything besides `skills/work-control.md` assumes one Task: resolved under Understand (`p1-single-task-assumption`).
-   Whether a thread may quote text such as `<commit>` without failing the checker: resolved by reading `check_placeholders`; prose-mode checking strips inline code, so a `ledger/` directory checked in prose mode accepts it.
-   Whether entry positions are stable enough to name an entry: resolved by the append-only contract, which this Task keeps.
-   Template name, disposition format, move versus copy when one Task takes a whole thread, and what the checker enforces: not uncertainties; they are choices recorded under Decide.

### INVESTIGATE gate

`ESTABLISHED`: every decision-relevant uncertainty above has a disposition, and none is deferred.

## Decide

Selected design; the operator chose each open point (`ledger.md`).

-   **Threads.** Pre-Task exploration lives in `ledger/<slug>.md`, one append-only file per exploration, from `templates/LEDGER-THREAD.md` (renamed from `templates/LEDGER.md`). The root `LEDGER.md` is deleted. An entry is named by its position among the thread's `Q:` entries.
-   **Dispositions.** An entry is settled by appending a `D:` line, never by editing it: taken by a named Task, rejected with a reason, or moved to a named place.
-   **Handoff.** A Task that takes every entry of a thread moves the file into its bundle as `ledger.md`. A Task that takes some entries copies them into its `ledger.md` and appends a `D:` line to the thread.
-   **Archival.** A thread whose every `Q:` has a `D:` is settled; Stewardship moves it to `.archive/ledger/` and removes it from `INDEX.md` in the same pass. `INDEX.md` lists every open thread.
-   **Isolate.** Asked once per Task, when the Task is created; an answer given for another Task does not carry over. Writing to a thread does not trigger it; a thread is committed before a new workspace is branched from it.
-   **Several Tasks.** At most 3 active Tasks. With more than one active, resuming asks the operator which one, unless the operator named it. Tasks that overlap in spec modules or files record the overlap and their order in both Tasks' Constraints. The order is the order of spec publication: the earlier Task publishes and finishes first, and the later one re-reads the current spec before its own `VERIFY`.
-   **Checker.** `check-capsule` refuses a root `LEDGER.md`, requires `INDEX.md`'s Ledger section to list exactly the files under `ledger/`, refuses more than 3 active Tasks, and checks `ledger/` in prose mode. Each check gets a test.
-   **Migration.** The ten fixup findings, with the paragraph introducing them, are copied byte for byte from this Task's `ledger.md` into `ledger/fixup-review-20261002.md`.

Rejected: tagging entries in one root file (still one shared file that conflicts across worktrees); creating the Task early instead of threads (fails when one exploration leads to several Tasks or none); `ROADMAP.md` or a new `backlog/` for untasked findings (operator chose open thread entries).

Residual uncertainty: none known. Verification strategy: each success criterion by inspection of the changed files and a reference search, `c4-checker-layout` by the checker's test suite and a run on this repository.

### DECIDE gate

`ESTABLISHED`: the design is complete, every open point has the operator's answer, and each success criterion has a planned check.

## Implement

-   `7554b3d` changes the docs, templates and layout: `CORE.md`, `skills/work-control.md`, `skills/stewardship.md`, `ADOPT.md`, `INDEX.md`, `templates/INDEX.md`, `templates/LEDGER.md` renamed to `templates/LEDGER-THREAD.md`, `templates/SELF-IMPROVEMENT.md`, `rules/self-improvement-log.md`; deletes the root `LEDGER.md`; adds `ledger/fixup-review-20261002.md` and the `SELF-IMPROVEMENT/` entry.
-   `6a8bec8` changes `scripts/check-capsule` and its tests: `check_ledger` (root `LEDGER.md`, thread listing), the `TASK_LIMIT` check in `check_index`, and `ledger` in prose-mode placeholder checking. Two existing tests that wrote a root `LEDGER.md` now use `INDEX.md` and a thread instead.
-   The commits are in that order so that each passes the checker it contains: the previous checker reports the new layout consistent.
-   No deviation from Decide.
-   Checkpoint results: the checker's test suite passed (63 tests) after the change, against a baseline of 59 passing before it; removing each new check in turn made exactly its test fail; `scripts/check-capsule` reports the repository consistent.

## Verify

<a id="verification-p1-single-task-assumption"></a>
### Verification: `p1-single-task-assumption`

- Claim: [p1-single-task-assumption](#p1-single-task-assumption)
- Method: searched current addf files for `ledger`, `the active task`, `the current task`, `one task`, `active cursor`, `the task's` and `resum` (case-insensitive), then read each hit in context.
- Evidence considered: the search output (129 lines, `logs/p1-references-20261002-0943.log`); the four `skills/work-control.md` passages listed under Relevant context; `check_index` in `scripts/check-capsule` iterating over every active Task.
- Conclusion: `VERIFIED`: apart from `skills/work-control.md`, no current file assumes one active Task; within it, the opening line is the rule, and the other three passages are wording or the Isolate scope.
- Limitations: a keyword search can miss a paraphrase. `skills/work-control.md` was read in full; `skills/stewardship.md` and `skills/evidence-and-verification.md` only around their hits.

<a id="verification-c1-ledger-threads"></a>
### Verification: `c1-ledger-threads`

- Claim: [c1-ledger-threads](#c1-ledger-threads)
- Method: checked that the root `LEDGER.md` is absent and `ledger/` holds a thread; listed every current mention of `LEDGER.md` outside `.archive/` and read each in context.
- Evidence considered: `logs/c1-references-20261002-1000.log`. The mentions are this Task's own records (history), `scripts/check-capsule` and its test (which refuse the file), `ADOPT.md` (moving an old root Ledger into a thread), and the new `SELF-IMPROVEMENT/` entry (history). None tells a reader to read or write the file.
- Conclusion: `VERIFIED` for the narrowed Claim. The original wording, that no current file refers to the file at all, was false; the operator narrowed it after seeing that.
- Limitations: none beyond the search's; every mention was read.

<a id="verification-c2-thread-handoff"></a>
### Verification: `c2-thread-handoff`

- Claim: [c2-thread-handoff](#c2-thread-handoff)
- Method: read the Ledger section of `skills/work-control.md`, Archive a settled thread and Update the index in `skills/stewardship.md`, and the Ledger sections of `templates/INDEX.md` and `INDEX.md` at `32db650`.
- Evidence considered: `logs/c2-c3-rules-20261002-1000.log` (`skills/work-control.md` lines 59-74, `skills/stewardship.md` lines 134-137, `templates/INDEX.md` line 25); `INDEX.md` lists `ledger/fixup-review-20261002.md`, and the checker enforces that listing (`logs/c4-mutations-20261002-1001.log`).
- Conclusion: `VERIFIED`: starting a Task copies or moves the entries it takes, a partial take appends a `D:` line, a thread is archived only when every `Q:` has a `D:` line, and the index lists open threads.
- Limitations: inspection of the rules, not a run of a Task taking part of a thread; the fix Tasks that follow will be the first such run.

<a id="verification-c3-several-active-tasks"></a>
### Verification: `c3-several-active-tasks`

- Claim: [c3-several-active-tasks](#c3-several-active-tasks)
- Method: read Isolate, Start or resume and Exit check in `skills/work-control.md` at `32db650`.
- Evidence considered: `logs/c2-c3-rules-20261002-1000.log` (line 24 Isolate per Task, line 127 the limit, lines 140-141 overlap and order, line 146 the resume question); the `TASK_LIMIT` check and its test (`logs/c4-mutations-20261002-1001.log`).
- Conclusion: `VERIFIED`: all four rules are stated, and the checker also enforces the limit.
- Limitations: `CORE.md` needed no change for this criterion; it routes to Work control without assuming one Task.

<a id="verification-c4-checker-layout"></a>
### Verification: `c4-checker-layout`

- Claim: [c4-checker-layout](#c4-checker-layout)
- Method: ran `python3 -m unittest discover -s addf/scripts/tests -v` and `addf/scripts/check-capsule` at `32db650`; then removed each new check in turn and ran the suite again.
- Evidence considered: `logs/c4-tests-20261002-1000.log` (63 tests, `OK`); `logs/c4-check-capsule-20261002-1000.log` (`capsule is consistent`); `logs/c4-mutations-20261002-1001.log` (each removal fails its test).
- Conclusion: `VERIFIED`: the checker validates the new layout and its suite passes.
- Limitations: Linux and the local Python 3 only; the suite was not run on Windows.

<a id="verification-c5-findings-thread"></a>
### Verification: `c5-findings-thread`

- Claim: [c5-findings-thread](#c5-findings-thread)
- Method: compared the findings block in `ledger/fixup-review-20261002.md` with the same block in this Task's `ledger.md` and in the root `LEDGER.md` at `e3e3c3b`, using `diff`; counted `Q:` and `D:` lines.
- Evidence considered: `logs/c5-c6-20261002-1000.log` (both comparisons identical, 10 `Q:` entries, 0 `D:` lines).
- Conclusion: `VERIFIED`: the ten findings are in the thread unchanged and all open.
- Limitations: the thread's two-line introduction is new text, as intended.

<a id="verification-c6-self-improvement"></a>
### Verification: `c6-self-improvement`

- Claim: [c6-self-improvement](#c6-self-improvement)
- Method: checked that the entry exists and read it against `templates/SELF-IMPROVEMENT.md`.
- Evidence considered: `logs/c5-c6-20261002-1000.log`; `SELF-IMPROVEMENT/20261002T075940Z-ledger-threads-and-several-active-tasks.md` has Trigger, What changed, Files touched and Why.
- Conclusion: `VERIFIED`.
- Limitations: none.

VERIFY gate: `ESTABLISHED`; every success Claim is `VERIFIED`, `c1-ledger-threads` in the wording the operator narrowed.

## Learn

### Technical

A success Claim worded as "no file refers to X" is falsified by the change's own enforcement and history: the checker that refuses X, the adoption note that migrates X, and the records of the Task that retired X all name it.
`c1-ledger-threads` had to be narrowed in `VERIFY` for that reason. A retirement Claim should be worded as "no current file tells anyone to use X".

### Process

-   Hand-made State-change commits went wrong twice (`3aab4e9`, `922e673`), and the recommended transition message form exceeds this repository's 72-character header limit for long Task names. Recorded as framework friction in `SELF-IMPROVEMENT/20261002T080904Z-hand-made-transition-commits.md`; no framework change made.
-   The Isolate question was not asked before the ledger commit `e3e3c3b`, although the rule required it then. The rule was clear; the agent missed it. No change.

LEARN gate: `ESTABLISHED`.

## Retention and promotion

The technical Learning is about wording success Claims; it is recorded here and in the SELF-IMPROVEMENT log, with no other permanent change.

### Promotion: success Claims

-   Claims: `c1-ledger-threads`, `c2-thread-handoff`, `c3-several-active-tasks`, `c4-checker-layout`, `c5-findings-thread`, `c6-self-improvement`.
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; the rules now live in the Skills and are enforced by `scripts/check-capsule`, which is the current truth.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `p1-single-task-assumption`

-   Claim: [p1-single-task-assumption](#p1-single-task-assumption)
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; it described the files before this Task changed them.
-   Disposition: not promoted — Task-scoped only.

No Claim promoted to Knowledge, and no Claim carried forward to `open-claims/`.

## Archive readiness

The bundle holds its ledger and every log it cites under `logs/`; internal links are relative.
References to framework files and commits are supplemental.
The ten fixup findings live on in `ledger/fixup-review-20261002.md`; the copy in this bundle's `ledger.md` is history.

## Terminal record

### Summary

`PENDING`

### Gate basis

`PENDING`
