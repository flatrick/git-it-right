# TASK — `Specify gir v1 and verify it conforms`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Draft the lint, hooks, config and CLI modules under
`delta/spec/`; review Codex's fixup, doctor, init and explain drafts against
the source.

**Open obligations:** each code/README/CHEATSHEET conflict needs an
operator ruling (blocks `DECIDE`); each requirement without a detecting test
needs a new test (blocks `VERIFY`).

## Owned artifacts

-   `delta/spec/` - proposed specification modules, published to
    `framework:spec/` at the terminal checkpoint.
-   `trace/` - one table per module mapping each requirement anchor to the
    tests that detect its violation.
-   `trace.py` - checks that every requirement is traced to a test the suite
    actually contains.

## Specification impact

- Current contract: `framework:SPEC.md`
- Proposed delta: add one `spec/` module per gir surface (CLI, config, lint
  rules, hooks, fixup, doctor, init, explain), linked from the
  `SPEC.md` specification map, stating the v1 behavior the operator accepts.
- Terminal publication: `PENDING`

## Define

### Objective

`gir's accepted v1 behavior is written into the addf specification, and every requirement in it is backed by a passing test that detects its violation, so feat/gir-v1 can merge into main.`

### Success criteria

<a id="spec-covers-gir"></a>
#### `spec-covers-gir`

-   Claim: Every user-observable behavior of gir at the final revision
    (subcommands, flags, exit codes, output shape, config keys, rule ids,
    generated files) is stated by a requirement in `delta/spec/`, and no
    requirement states behavior gir lacks.
-   State: `UNVERIFIED`
-   Scope: The final revision of `feat/gir-v1`, Linux.
-   Consequence if false: `The published spec misleads the next Task about what gir promises.`
-   Basis: Pending an inventory of `src/` against the modules plus an
    independent Codex review.

<a id="every-requirement-traced"></a>
#### `every-requirement-traced`

-   Claim: Every requirement anchor in `delta/spec/` is traced to at least
    one test that asserts it, and each traced test exists in the suite.
-   State: `UNVERIFIED`
-   Scope: The final revision of `feat/gir-v1`, `trace.py` against
    `cargo test -- --list`.
-   Consequence if false: `A spec requirement can regress without any test failing.`
-   Basis: Pending the trace tables and a `trace.py` run.

<a id="suite-green"></a>
#### `suite-green`

-   Claim: `cargo test`, `cargo clippy --all-targets -- -D warnings` and
    `check-capsule` all pass.
-   State: `UNVERIFIED`
-   Scope: The final revision of `feat/gir-v1`, Linux.
-   Consequence if false: `The branch is not mergeable.`
-   Basis: Pending the final-revision run.

### Constraints

-   Isolation: the operator chose to work in the existing worktree
    `.worktrees/gir-v1` on branch `feat/gir-v1`; no new worktree or branch.
-   When code, README or CHEATSHEET disagree, the operator rules per
    conflict; the spec states only accepted behavior.
-   Codex drafts the fixup, doctor, init and explain modules and writes
    missing tests; Claude drafts lint, hooks, config and CLI and verifies
    every Codex artifact before it lands.
-   Nothing is pushed; merging into main is the operator's action.

### Material empirical premises

<a id="spec-is-a-stub"></a>
#### `spec-is-a-stub`

-   Claim: At revision `8961126`, `SPEC.md` states only gir's purpose and
    one boundary, with an empty specification map.
-   State: `VERIFIED`
-   Scope: Revision `8961126`.
-   Consequence if false: `The Task would duplicate an existing specification.`
-   Basis: [Verification](#verification-spec-is-a-stub)

<a id="baseline-green"></a>
#### `baseline-green`

-   Claim: At revision `8961126` on Linux, `cargo test` passes all 36
    tests and `check-capsule` reports the capsule consistent.
-   State: `VERIFIED`
-   Scope: Revision `8961126`, Linux, before any Task change.
-   Consequence if false: `Later failures could not be attributed to this Task.`
-   Basis: [Verification](#verification-baseline-green)

## Understand

### Relevant context

gir was implemented before addf was adopted, so no Task produced its
behavior and `SPEC.md` never received it. README.md, CHEATSHEET.md and
`gir explain` pages describe the behavior informally.

### Assumptions

-   NONE.

### Open questions

-   NONE yet.

### Deferred verification

-   NONE.

## Investigate

## Decide

## Implement

## Verify

<a id="verification-spec-is-a-stub"></a>
### Verification: `spec-is-a-stub`

- Claim: [spec-is-a-stub](#spec-is-a-stub)
- Method: Read `framework:SPEC.md` at `8961126`.
- Evidence considered: Its Purpose section is one paragraph, Boundaries
  holds one item, and Specification map is `NONE`.
- Conclusion: `VERIFIED`, the file contains no requirement beyond purpose.
- Limitations: NONE.

<a id="verification-baseline-green"></a>
### Verification: `baseline-green`

- Claim: [baseline-green](#baseline-green)
- Method: Ran `cargo test` and `python addf/scripts/check-capsule --root addf`
  from the worktree root at `8961126`, with the full output of each in `.scratch/`.
- Evidence considered: Six test binaries reported 23, 0, 10, 2, 1 and 0
  passed with 0 failed; the checker printed `capsule is consistent` and
  exited 0.
- Conclusion: `VERIFIED`, the baseline is green on Linux.
- Limitations: Windows and macOS were not run locally.

## Learn

### Technical

### Process

## Retention and promotion

## Archive readiness

## Terminal record

### Summary

### Gate basis

## Stop record
