# TASK — `Specify gir v1 and verify it conforms`

## Resume

**Contract version:** `2`

**State:** `IMPLEMENT`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT`

**Resume at:** Implement rulings C1 and C2 in `src/`, then dispatch the Codex
test and ruling workers described under Decide.

**Open obligations:** rulings C1 to C6 need their code and doc changes
(blocks `VERIFY`); each requirement without a detecting test needs a new test
(blocks `VERIFY`).

## Owned artifacts

-   `delta/spec/` - proposed specification modules, published to
    `framework:spec/` at the terminal checkpoint.
-   `trace/` - one table per module mapping each requirement anchor to the
    tests that detect its violation.
-   `trace.py` - checks that every requirement is traced to a test the suite
    actually contains, and that every cited `path:line` holds an assertion.

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

Claude drafted `cli`, `config`, `lint` and `hooks` from `src/main.rs`,
`src/config.rs`, `src/cc/`, `src/message.rs`, `src/report.rs`,
`src/cmd/lint.rs` and `src/cmd/hook.rs`. Codex (`codex-cli 0.157.0`,
`-s workspace-write`, one throwaway worktree per module at `7870ffd`)
drafted `fixup`, `doctor`, `init` and `explain`. Claude reviewed each Codex
module against its source file and removed 22 requirements that restated
argument handling owned by the `cli` module or hook-script behavior owned by
the `hooks` module.

### Assumptions

-   NONE.

### Open questions

-   NONE. The six conflicts found are resolved under Investigate.

### Deferred verification

-   NONE.

## Investigate

Gate `UNDERSTAND` exit: `ESTABLISHED`. Every gir source file was read by the
module's drafter and every Codex draft was reviewed against its source.

Drafting surfaced six places where code and documentation disagree. Each got
an operator ruling on 2026-09-25.

-   C1. README says `gir lint --range` catches `--no-verify` commits and the
    pre-push hook blocks messages that skipped the hook. Observed: a commit
    `Feature(api) :Add retry.` made with `--no-verify` passed
    `gir lint --range HEAD~1..HEAD` with exit `0` and `"ok":true`, because
    only rejections fail. Ruling: `--range` and pre-push reject a commit a safe
    fix would change.
-   C2. `src/main.rs` parses `--range` for every subcommand, so `init`,
    `doctor`, `fixup` and `explain` silently ignore it. Ruling: reject it
    outside `lint`.
-   C3. README says `gir explain config` shows its sample, which includes
    `bugfix = fix`; the page in `src/explain.md` lists only `feature = feat`.
    Ruling: add `bugfix = fix` to the page.
-   C4. README and CHEATSHEET say `gir fixup` refuses new-file, multi-commit
    and base-branch lines; an explicit `gir fixup COMMIT` skips every check.
    Ruling: validate an explicit target, which must be in `HEAD`'s history
    and after the base.
-   C5. CHEATSHEET says fixup refuses lines from `main`; the code uses the
    first base of `origin/HEAD`, `main`, `master`, `@{upstream}`. Ruling:
    keep the behavior and say "base branch" in CHEATSHEET.
-   C6. README says rerunning `gir init` changes nothing, but every run
    re-stages both hooks. Ruling: stage a hook only when init wrote it or its
    index mode is not `100755`.

Every material uncertainty is resolved by these rulings; none is deferred.

## Decide

Gate `INVESTIGATE` exit: `ESTABLISHED`. Each conflict has an operator
ruling, recorded above.

Selected approach: the delta in `delta/spec/` states the ruled behavior.
Claude implements C1 (a `fix-pending` rejection in `src/cmd/lint.rs`, used
by `--range` and pre-push, with an explain page) and C2 (`src/main.rs`).
Before any test is added, Claude moves the `Repo` helper from `tests/cli.rs`
into `tests/common/mod.rs`, so parallel workers each add a separate test
file instead of editing one shared file. Codex workers, one per module in
its own worktree, then implement C4 and C5 (fixup), C6 (init) and C3
(explain), and write a detecting test for every untraced requirement of
their module, with a trace row for each. Claude reviews every diff before it
lands.

Rejected alternatives: one Codex run for all tests (slower, and one large
diff to review); tests added to `tests/cli.rs` by all workers (concurrent
edits to one file).

Verification strategy: `trace.py` reports `0 problems`; `cargo test`,
`cargo clippy --all-targets -- -D warnings` and `check-capsule` pass; for
each ruling, reverting the code change makes its new test fail.

Gate `DECIDE` exit: `ESTABLISHED`.

## Implement

-   `b1530d0` moved the `Repo` helper to `tests/common/mod.rs`; clippy and all
    36 tests passed before and after.
-   `ed7459b` (C2): `src/main.rs` parses `--range` only for `lint`. Observed:
    `gir init|doctor|fixup|explain --range x` each print
    `gir: unknown option --range for` and exit `2`; `gir lint --range` still
    runs.
-   `f820c92` (C1): `reject_recorded` in `src/cmd/lint.rs` adds `fix-pending`
    for `--range` and pre-push, with a `fix-pending` explain page. Observed: a
    `--no-verify` commit `Feature(api) :Add retry.` is rejected with
    `pending fixes: header-spacing type-case type-alias desc-period` and
    `try: feat(api): Add retry`, exit `1`; a clean range exits `0`.
-   `e3dffb3` (C3): the `src/explain.md` config page shows `bugfix = fix`;
    observed in `gir explain config` output.
-   Codex workers (`codex-cli 0.157.0`, `-s workspace-write`, one worktree
    each) are implementing C4 and C5 (fixup), C6 (init) and the missing tests
    for all eight modules.

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
