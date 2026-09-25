# TASK — `Specify gir v1 and verify it conforms`

## Resume

**Contract version:** `2`

**State:** `COMPLETED`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN -> COMPLETED`

**Resume at:** `NONE`

**Open obligations:** `NONE`

## Owned artifacts

-   `delta/spec/` - proposed specification modules, published to
    `framework:spec/` at the terminal checkpoint.
-   `trace/` - one table per module mapping each requirement anchor to the
    tests that detect its violation.
-   `trace.py` - checks that every requirement is traced to a test the suite
    actually contains, that every cited `path:line` holds an assertion inside
    the named test, and remaps citations through a diff with `--follow`.
-   `inventory.py` - lists user-visible identifiers from the source and binary
    that `delta/spec/` does not mention.

## Specification impact

- Current contract: `framework:SPEC.md`
- Proposed delta: add one `spec/` module per gir surface (CLI, config, lint
  rules, hooks, fixup, doctor, init, explain), linked from the
  `SPEC.md` specification map, stating the v1 behavior the operator accepts.
- Terminal publication: `framework:spec/cli.md`, `framework:spec/config.md`,
  `framework:spec/lint.md`, `framework:spec/hooks.md`,
  `framework:spec/fixup.md`, `framework:spec/doctor.md`,
  `framework:spec/init.md`, `framework:spec/explain.md`, linked from
  `framework:SPEC.md`

## Define

### Objective

`gir's accepted v1 behavior is written into the addf specification, and every requirement in it is backed by a passing test that detects its violation, so feat/gir-v1 can merge into main.`

### Success criteria

<a id="spec-covers-reviewed-gir"></a>
#### `spec-covers-reviewed-gir`

-   Claim: Every user-visible identifier gir's source and binary expose
    (rule ids, config keys, flags, doctor check ids, explain topics) appears
    in `delta/spec/`, and every defect found by three independent read-only
    Codex reviews of `delta/spec/` against `src/` is resolved in code or spec.
-   State: `VERIFIED`
-   Scope: `src/` and `tests/` at `f83b6bb`, Linux; the three review passes
    recorded under Implement.
-   Consequence if false: `The published spec misleads the next Task about what gir promises.`
-   Basis: [Verification](#verification-spec-covers-reviewed-gir)

<a id="every-requirement-traced"></a>
#### `every-requirement-traced`

-   Claim: Every requirement anchor in `delta/spec/` is traced to at least
    one test that asserts it, and each traced test exists in the suite.
-   State: `VERIFIED`
-   Scope: `src/` and `tests/` at `f83b6bb`, `trace.py` against
    `cargo test -- --list` on Linux; the one Windows-gated row is checked
    only on Windows.
-   Consequence if false: `A spec requirement can regress without any test failing.`
-   Basis: [Verification](#verification-every-requirement-traced)

<a id="suite-green"></a>
#### `suite-green`

-   Claim: `cargo test`, `cargo clippy --all-targets -- -D warnings` and
    `check-capsule` all pass.
-   State: `VERIFIED`
-   Scope: `src/` and `tests/` at `f83b6bb`, Linux.
-   Consequence if false: `The branch is not mergeable.`
-   Basis: [Verification](#verification-suite-green)

### Constraints

-   Isolation: the operator chose to work in the existing worktree
    `.worktrees/gir-v1` on branch `feat/gir-v1`; no new worktree or branch.
-   When code, README or CHEATSHEET disagree, the operator rules per
    conflict; the spec states only accepted behavior.
-   Codex drafts the fixup, doctor, init and explain modules and writes
    missing tests; Claude drafts lint, hooks, config and CLI and verifies
    every Codex artifact before it lands.
-   Nothing is pushed; merging into main is the operator's action.
-   2026-09-25: the operator bounded coverage after three review passes,
    replacing the unbounded `spec-covers-gir` Claim with
    `spec-covers-reviewed-gir`, and chose to run the Windows checks
    personally instead of pushing for CI.

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

<a id="gir-v1-windows-macos"></a>
#### `gir-v1-windows-macos`

-   Claim: `cargo test` and `cargo clippy --all-targets -- -D warnings`
    pass on Windows and macOS, including the Windows-only
    `doctor_repairs_unset_windows_longpaths`.
-   State: `UNVERIFIED`
-   Scope: `src/` and `tests/` at `f83b6bb`, Windows and macOS.
-   Consequence if false: `gir, which promises Windows, Linux and macOS, could fail on a platform the Linux evidence does not cover.`
-   Basis: `DEFERRED_VERIFICATION`; see Deferred verification.

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

-   [gir-v1-windows-macos](#gir-v1-windows-macos): the operator has no push
    for CI and will run the suite on Windows personally; macOS has no
    available machine. Earliest checkpoint: the operator's Windows run.
    Settling observation: `cargo test` and clippy exit `0` on each OS.
    Consequence if false: platform-specific failure in a cross-platform
    tool. Blocked work: none in this Task; carried forward as an open Claim.

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
    per module) wrote the missing tests for all eight modules and the C4, C5
    and C6 changes. Claude reviewed every diff before it landed and changed
    three things: the missing-gir and missing-git-cliff tests filter `PATH`
    instead of copying the `git` binary; the Windows-names test disables
    `core.protectNTFS` for its own setup; `Repo::cmd` points
    `XDG_CONFIG_HOME` into the sandbox so a global ignore file cannot change
    doctor's report.
-   `07382af` (C4, C5): explicit fixup targets are validated; README and
    CHEATSHEET name the base branches. `506c49e` (C6): init stages a hook
    only when written or not `100755` in the index. `2cb383f`, `8ffbf1f`:
    tests for the remaining modules.
-   Mutation checks, each reverting only the source change and running the
    named test binaries (full output in `.scratch/mutation-*.log`, not
    committed): reverting C1 failed `pre_push_rejects_commits_that_skipped_safe_fixes`
    and `lint_range_rejects_every_pending_safe_fix_and_shows_fixed_subject`;
    C2 failed `range_is_rejected_outside_lint_and_accepted_by_lint`; C4 failed
    `explicit_target_must_be_after_base` and
    `explicit_target_must_be_in_current_branch_history`; C6 failed four
    `tests/init.rs` tests.
-   An independent read-only Codex review of `delta/spec/` against `src/`
    reported eight defects. Claude reproduced each, plus one it missed
    (`gir fixup` from a subdirectory). Fixed in code, each with a test that
    fails when the fix is reverted: init, doctor and fixup from a
    subdirectory (`c52089c`); a malformed `.girconfig` silently meaning
    defaults (`7e146de`); `feat: .` reported as `spec` instead of
    `desc-empty` (`a88739d`). Corrected in the spec to the code's
    intentional behavior: `gir hook pre-push` ignores extra arguments, and
    `gir fixup` passes git's own output through before its error. Added to
    the spec: the full `.editorconfig` content.
-   `trace.py` gained two checks after cited line numbers drifted four
    times: a cited range must lie inside the test the row names, and
    `--follow REV` remaps citations through the diff since `REV`.
-   `inventory.py` lists every rule id, config key, flag, doctor check id and
    explain topic from the source and binary that `delta/spec/` does not
    mention.
-   Second Codex review pass (5 findings), each reproduced by Claude first.
    Fixed with detecting tests: a staged binary change reported an empty
    "several commits" list (`f996721`); an unmerged path's stages were
    reported as a case collision (`0cd6708`); `--range --fix` printed
    "fixed" lines for recorded commits and is now refused (`0427bf4`).
    Specified: `--range` takes any revision `git log` takes; `--version`
    and `--help` ignore trailing arguments.
-   Third Codex review pass (10 findings), each reproduced by Claude first.
    Fixed with detecting tests: `doctor --fix` replaced a non-UTF-8
    `.gitignore` and could overwrite a non-UTF-8 `.gitattributes`
    (`7aab161`); `init` overwrote a differing non-UTF-8 file without
    `--force` (`95cf5c7`); fixup folded a hunk replacing a base-branch line
    into a later commit, and gave a staged new binary file the wrong refusal
    (`9802fb6`). Specified: footer-block scope, body separator only for
    Conventional headers, a repeated `--range`, init's last line on exit
    `1`, pre-push skipping malformed stdin lines, and an unknown short flag
    before `--help`. Codex wrote these tests (`640ffc0` and the fix
    commits); reverting each source fix failed exactly its new tests.
-   The operator bounded coverage after pass 3, since the passes found
    8, 5 and 10 increasingly marginal defects and would not converge.

## Verify

Gate `IMPLEMENT` exit: `ESTABLISHED`. Every ruling and review fix is
committed, and each has a test that fails when it is reverted.

Final evidence run at `f83b6bb` on Linux (full logs in `.scratch/`, suffix
`final-20260925-2109`, not committed): `cargo clippy --all-targets -- -D
warnings` exit `0`; `cargo test --no-fail-fast` exit `0` with 153 passed and
0 failed across 14 test binaries; `trace.py` exit `0`, `167 requirements, 201
trace rows, 0 problems, 1 checked only on another OS`; `inventory.py` exit
`0`, `92 identifiers, 0 missing`; `check-capsule` exit `0`, `capsule is
consistent`.

<a id="verification-spec-covers-reviewed-gir"></a>
### Verification: `spec-covers-reviewed-gir`

- Claim: [spec-covers-reviewed-gir](#spec-covers-reviewed-gir)
- Method: `inventory.py` extracted 20 rule ids, 8 config keys, 5 flags, 21
  doctor check ids and 38 explain topics from `src/` and `gir explain`, and
  searched `delta/spec/` for each. Three independent read-only Codex reviews
  (`codex exec -s read-only`, same prompt) compared `delta/spec/` against
  `src/`; Claude reproduced every finding before acting on it.
- Evidence considered: The inventory reported `92 identifiers, 0 missing`.
  The reviews reported 8, 5 and 10 defects; every one is resolved under
  Implement, by a code fix with a detecting test or by a spec correction to
  intentional behavior. One further defect Claude found while reproducing
  (fixup from a subdirectory) is fixed too.
- Conclusion: `VERIFIED`, every inventoried identifier is specified and
  every review finding is resolved.
- Limitations: a substring match shows an identifier is mentioned, not that
  its requirement is correct. The review passes did not converge, so
  behaviors none of them examined may remain unspecified; the operator
  accepted that bound. Not blocking.

<a id="verification-every-requirement-traced"></a>
### Verification: `every-requirement-traced`

- Claim: [every-requirement-traced](#every-requirement-traced)
- Method: `trace.py` against `cargo test -- --list` at `f83b6bb`. It fails
  on an untraced requirement, an unknown requirement, a test missing from
  the suite, a cited range without an `assert`, or a cited range outside the
  named test. Each rule was exercised during Implement: untraced
  requirements were reported throughout, and a deliberately wrong citation
  was reported as outside its test. Mutation checks under Implement show the
  tests for every ruling and review fix fail when the fix is reverted.
- Evidence considered: `167 requirements, 201 trace rows, 0 problems, 1
  checked only on another OS`. The OS-gated row is
  `doctor_repairs_unset_windows_longpaths`.
- Conclusion: `VERIFIED`, every requirement is traced to a listed test on
  Linux, and the Windows-gated row names a test compiled only there.
- Limitations: a trace row shows that a test asserts the cited lines; only
  the mutated requirements were proven to fail under violation. The
  Windows-gated test's run is deferred with
  [gir-v1-windows-macos](#gir-v1-windows-macos). Not blocking.

<a id="verification-suite-green"></a>
### Verification: `suite-green`

- Claim: [suite-green](#suite-green)
- Method: the final evidence run above.
- Evidence considered: clippy exit `0`, `cargo test` exit `0` with 153
  passed and 0 failed, `check-capsule` exit `0`.
- Conclusion: `VERIFIED` on Linux.
- Limitations: Windows and macOS are deferred with
  [gir-v1-windows-macos](#gir-v1-windows-macos). Not blocking for the
  Linux-scoped Claim.

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

Gate `VERIFY` exit: `ESTABLISHED`. All three success Claims are `VERIFIED`.

### Technical

-   Treating a failed `read_to_string` as "file missing" caused all three
    data-loss bugs (doctor on `.gitignore` and `.gitattributes`, init on
    kept files). Reading bytes and checking `NotFound` explicitly is the
    fix pattern for any gir code that decides whether to overwrite.
-   Commands that pass root-relative paths to git must run from the
    repository root; `git::enter_toplevel` does that for init, doctor and
    fixup.

### Process

-   Hand-cited test line numbers drifted four times. The fix is encoded in
    `trace.py` (in-test check, `--follow`) rather than in guidance.
-   Repeated read-only reviews of a spec against its code found real bugs on
    every pass but never converged; bounding coverage needed an operator
    decision, which similar Tasks should plan for in DEFINE.

Gate `LEARN` exit: `ESTABLISHED`.

## Retention and promotion

### Promotion: `spec-covers-reviewed-gir`

-   Claim: [spec-covers-reviewed-gir](#spec-covers-reviewed-gir)
-   Will this Claim's validity outlive this Task and inform a future
    decision? `no`, the published spec is the durable artifact; this Claim
    only records how it was checked.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `every-requirement-traced`

-   Claim: [every-requirement-traced](#every-requirement-traced)
-   Will this Claim's validity outlive this Task and inform a future
    decision? `no`, it holds only for `f83b6bb` and the trace tables archive
    with this bundle.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `suite-green`

-   Claim: [suite-green](#suite-green)
-   Will this Claim's validity outlive this Task and inform a future
    decision? `no`, the next run supersedes it.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `gir-v1-windows-macos`

-   Claim: [gir-v1-windows-macos](#gir-v1-windows-macos)
-   Will this Claim's validity outlive this Task and inform a future
    decision? `yes`, it decides whether gir's cross-platform promise holds
    before `feat/gir-v1` merges.
-   Disposition: carried forward to `open-claims/gir-v1-windows-macos.md`.

Learning disposition: the two technical learnings are encoded in code
(`git::enter_toplevel`, byte reads); the drift learning is encoded in
`trace.py`; the review-convergence learning is recorded here only, since no
permanent destination fits it yet. `trace.py` and `inventory.py` archive
with this bundle; the friction that causes is logged in `SELF-IMPROVEMENT/`.

## Archive readiness

The bundle is self-contained: `TASK.md`, `delta/spec/` (the verified
proposal, kept as provenance beside its published copy), `trace/`,
`trace.py` and `inventory.py` reference only each other and repository
source. `trace.py` and `inventory.py` locate the repository from their own
path, so reproducing the evidence means checking out `f83b6bb` and running
them from `addf/tasks/gir-v1-spec/`. `.scratch/` logs and Codex run
directories are supplemental and not required to reconstruct the outcome.

## Terminal record

### Summary

gir v1's behavior is published as eight specification modules linked from
`SPEC.md`. Every one of its 167 requirements is traced to a passing test.
The operator ruled on six code/doc conflicts. The review passes surfaced
code defects fixed in nine commits, each with tests that fail when the fix
is reverted.

### Gate basis

`spec-covers-reviewed-gir`, `every-requirement-traced` and `suite-green` are
`VERIFIED` through the Verifications above. The deferred Windows and macOS
verification is carried forward as
`framework:open-claims/gir-v1-windows-macos.md`.

## Stop record
