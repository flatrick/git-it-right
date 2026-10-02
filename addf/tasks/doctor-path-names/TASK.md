# TASK — `gir doctor reads file names exactly as git stores them`

## Resume

**Contract version:** `2`

**State:** `DEFINE`

**State path:** `DEFINE`

**Resume at:** Probe each suspicion in `c1-suspicions-settled` against the current build, and record each as confirmed or refuted.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the questions that shaped this Task, with the operator's answers.

## Specification impact

- Current contract: `framework:spec/doctor.md#req-doctor-case-collision`, `framework:spec/doctor.md#req-doctor-windows-names`, `framework:spec/doctor.md#req-doctor-exec-bit`, `framework:spec/doctor.md#req-doctor-ignore-rules`
- Proposed delta: To be settled in `DECIDE` from what the investigation confirms; `NONE` if no requirement changes.
- Terminal publication: `PENDING`

## Define

### Objective

`gir doctor` reads every file name exactly as git stores it, so its checks judge real names, not git's quoted display form.

### Success criteria

<a id="c1-suspicions-settled"></a>
#### `c1-suspicions-settled`

-   Claim: Each suspicion is probed on the build at this Task's start and recorded as confirmed or refuted, with evidence: (a) names git quotes (double quote, backslash, tab, newline) reach the index checks in quoted form; (b) non-ASCII names reach the ignore-rule check in quoted form; (c) `windows_unsafe` does not treat control characters as unsafe; (d) non-UTF-8 names are mangled.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: a fix is made for a defect that does not exist, or a real one is missed.
-   Basis: pending check.

<a id="c2-real-names"></a>
#### `c2-real-names`

-   Claim: For every confirmed defect, `gir doctor` reports the real name and decides from it: `case-collision`, `windows-names`, `exec-bit` and the ignore-rule detection work for names with spaces, double quotes, backslashes, tabs, non-ASCII characters and, on Unix, non-UTF-8 bytes.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: doctor misjudges or misreports repositories with such names.
-   Basis: pending check.

<a id="c3-tests"></a>
#### `c3-tests`

-   Claim: Regression tests cover each confirmed defect and fail on the build at this Task's start; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision; tests for names Windows cannot hold are Unix-only with the reason stated.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: pending check.

<a id="c4-spec"></a>
#### `c4-spec`

-   Claim: If a requirement changes (for example control characters in `windows-names`), the delta is recorded in `DECIDE` and published at completion; otherwise Specification impact stays `NONE`.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the specification no longer describes doctor.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Code and test scripts follow `rules/os-agnostic-code.md`.
-   Out of scope: other `doctor` checks.
-   No other Task is active.

### Material empirical premises

<a id="p1-suspicions-from-reading"></a>
#### `p1-suspicions-from-reading`

-   Claim: The four suspicions in `c1-suspicions-settled` come from reading `src/cmd/doctor.rs` at `76de5c5`, not from running it.
-   State: `UNVERIFIED`
-   Scope: `76de5c5`.
-   Consequence if false: the investigation starts from a wrong reading.
-   Basis: pending check.

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

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
