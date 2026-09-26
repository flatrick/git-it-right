# TASK — `gir init respects removed optional files and generates GIT-IT-RIGHT.md`

## Resume

**Contract version:** `2`

**State:** `COMPLETED`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN -> COMPLETED`

**Resume at:** `NONE`

**Open obligations:** `NONE`

## Owned artifacts

-   `ledger.md` - pre-Task and DEFINE questions with the operator's answers.
-   `logs/test-red-20260926-1846.log` - Evidence: new init tests against unchanged `src/` (Windows).
-   `logs/test-docs-20260926-1847.log` - Evidence: `cargo test` on the final change (Windows).
-   `logs/clippy-20260926-1847.log` - Evidence: `cargo clippy --all-targets -- -D warnings` (Windows).
-   `logs/dogfood-20260926-1848.log` - Evidence: the built `gir init` and `gir init --optional` in this repository.
-   `logs/gate-linux-539416b-20260926-1849.log` - Evidence: clippy and tests at `539416b` in WSL2 Arch Linux.

## Specification impact

- Current contract: `framework:spec/init.md`, `framework:spec/cli.md#req-cli-usage-lists-commands`, `framework:spec/cli.md#req-cli-unknown-long-option`
- Proposed delta: changes to the `init` and `cli` modules.
  - init: define *optional files* (`cliff.toml`, `GIT-IT-RIGHT.md`) and *required files* (`.githooks/commit-msg`, `.githooks/pre-push`, `.girconfig`).
  - init: add a requirement that `gir init` creates `GIT-IT-RIGHT.md` with static content covering what gir is, a contributor quick-start, git-cliff, and a no-CI maintainer check.
  - init: add *adopted repository* — `.girconfig` is in the index. In an adopted repository, `gir init` without flags SHALL NOT create a missing optional file and SHALL print nothing about it; it SHALL still create missing required files.
  - init: add `--optional` — `gir init --optional` also creates each missing optional file and never replaces an existing file (existing-file rules unchanged).
  - init: `--force` also creates missing optional files in an adopted repository.
  - cli: accepted flags become `init --force --optional`; usage text lists `--optional`.
- Terminal publication: `framework:spec/init.md#req-init-accept-optional`, `framework:spec/init.md#req-init-file-kinds`, `framework:spec/init.md#req-init-adopted`, `framework:spec/init.md#req-init-required-files`, `framework:spec/init.md#req-init-optional-files`, `framework:spec/init.md#req-init-optional-no-overwrite`, `framework:spec/init.md#req-init-cliff-file`, `framework:spec/init.md#req-init-git-it-right-file`, `framework:spec/cli.md#req-cli-unknown-long-option`

## Define

### Objective

In a repository that has adopted gir, a contributor's `gir init` activates their clone without recreating optional files the maintainer removed.
`gir init` also generates a `GIT-IT-RIGHT.md` that explains gir to new contributors, as an optional file on the same terms as `cliff.toml`.
A maintainer can get a removed optional file back with `gir init --optional` without overwriting edited files.

### Success criteria

<a id="adopted-skips-optional"></a>
#### `adopted-skips-optional`

-   Claim: With `.girconfig` in the index and `cliff.toml` and `GIT-IT-RIGHT.md` absent, `gir init` exits `0`, creates neither file, and prints no stderr line naming either.
-   State: `VERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: a maintainer's removal is still undone by contributors.
-   Basis: [Verification](#verification-adopted-skips-optional).

<a id="adopted-restores-required"></a>
#### `adopted-restores-required`

-   Claim: With `.girconfig` in the index and a hook file absent, `gir init` recreates and stages that hook and sets `core.hooksPath`.
-   State: `VERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: contributors can't activate gir after a hook was lost.
-   Basis: [Verification](#verification-adopted-restores-required).

<a id="fresh-writes-all"></a>
#### `fresh-writes-all`

-   Claim: In a repository where `.girconfig` is not in the index, `gir init` creates the hooks, `.girconfig`, `cliff.toml` and `GIT-IT-RIGHT.md`, and prints `gir: wrote GIT-IT-RIGHT.md`.
-   State: `VERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: first adoption misses the onboarding file.
-   Basis: [Verification](#verification-fresh-writes-all).

<a id="optional-flag"></a>
#### `optional-flag`

-   Claim: In an adopted repository, `gir init --optional` creates each missing optional file and leaves an existing file whose content differs from its template unchanged, with the existing `kept` message and exit `1`.
-   State: `VERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: restoring an optional file risks clobbering edits, or does not work.
-   Basis: [Verification](#verification-optional-flag).

<a id="force-restores-optional"></a>
#### `force-restores-optional`

-   Claim: In an adopted repository, `gir init --force` creates missing optional files.
-   State: `VERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: `--force` stops meaning "write every template".
-   Basis: [Verification](#verification-force-restores-optional).

<a id="git-it-right-content"></a>
#### `git-it-right-content`

-   Claim: Generated `GIT-IT-RIGHT.md` is static (identical for any `.girconfig`) and covers what gir is and why, a contributor quick-start (install, `gir init` once per clone, `gir explain <rule>`), git-cliff (why `cliff.toml` exists, optional, maintainer commands, safe to delete), and the no-CI `gir lint --range` check plus the CI one-liner.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: the file doesn't do its onboarding job.
-   Basis: [Verification](#verification-git-it-right-content).

<a id="suite-green"></a>
#### `suite-green`

-   Claim: `cargo test` (including the 750 ms commit-msg latency budget) and `cargo clippy --all-targets -- -D warnings` pass.
-   State: `VERIFIED`
-   Scope: Windows and WSL2 Arch Linux, this branch.
-   Consequence if false: regression.
-   Basis: [Verification](#verification-suite-green).

<a id="dogfood"></a>
#### `dogfood`

-   Claim: Running the built `gir init --optional` in this repository adds `GIT-IT-RIGHT.md` and changes no other tracked file.
-   State: `VERIFIED`
-   Scope: this worktree.
-   Consequence if false: the flag misbehaves on a real adopted repo.
-   Basis: [Verification](#verification-dogfood).

### Constraints

-   Isolated worktree `.worktrees/init-activate-only`, branch `feat/init-activate-only`, from `main` at `04f9cee` (operator's request).
-   Adoption signal is `.girconfig` in the index; no new `.girconfig` key.
-   Skipping an optional file is silent.
-   `GIT-IT-RIGHT.md` is static text; it does not embed `.girconfig` values.
-   Docs that describe `gir init` (`README.md`, `gir explain` pages, usage text) are updated to match; no other docs.
-   No push, no PR.

### Material empirical premises

<a id="init-recreates-missing"></a>
#### `init-recreates-missing`

-   Claim: Today `gir init` recreates any missing template file, including `cliff.toml` in a repository whose `.girconfig` is tracked.
-   State: `VERIFIED`
-   Scope: `main` at `04f9cee`, Windows.
-   Consequence if false: the fix half of this Task has no problem to solve.
-   Basis: [Verification](#verification-init-recreates-missing).

DEFINE gate: `ESTABLISHED`.
The operator agreed to the objective, scope and success criteria on 2026-09-26 ("I agree: move ahead with this plan").

## Understand

### Relevant context

-   `src/cmd/init.rs` `init(force)` writes each template through `write()`; a missing or unreadable file falls through to `std::fs::write`.
    Order: hooks, `.girconfig`, load config, `cliff.toml`, stage hooks, set `core.hooksPath`, print `gir: next: gir doctor`.
-   `src/cmd/templates.rs` holds the hook shim and `cliff_toml(cfg)`; `GIT-IT-RIGHT.md` becomes a static constant there.
-   `src/main.rs` parses flags; `init` accepts only `force`; the usage text names `cliff.toml`.
-   `tests/init.rs` drives the real binary in temp repositories via `tests/common/mod.rs` (`Repo::new`, `gir`, `git`, `write`).
-   Docs that describe `init`: `README.md` (Set up a repository, Commands, Changelog), `src/explain.md` `## hooks`.
-   UNDERSTAND gate: `ESTABLISHED` — the change is local to `init`, its templates, its flag parsing and those docs.

### Assumptions

-   `gir init` does not stage `.girconfig`, so the adoption check gives the same answer before and after `init`'s own writes. Source: `init.rs` stages only hook paths. Verified by reading; consequence if false: first run would see itself as adopted.

### Open questions

`NONE`.

### Deferred verification

-   macOS behavior; no macOS host; no claim in this Task is scoped to macOS, and the standing statement in `framework:evidence/gir-v1-windows-verification.md` (macOS assumed, not verified) still covers it. Blocks nothing.

## Investigate

-   `init-recreates-missing`: resolved, `VERIFIED` (see Verify).
    Six new tests in `tests/init.rs` ran against unchanged `src/`; `adopted_init_skips_missing_optional_files` failed with `adopted init leaves the removed cliff.toml absent`, i.e. `cliff.toml` was recreated.
    `adopted_init_restores_missing_required_files` already passed, as expected: required-file restore is existing behavior.
-   INVESTIGATE gate: `ESTABLISHED` — no decision-relevant uncertainty remained.

## Decide

-   Compute `adopted` once at the start of `init` with `git ls-files -- .girconfig`, before any write.
-   Write each optional file only when `!adopted || force || optional || file exists`; the existing `write()` keeps its unchanged/kept/written rules, so `--optional` inherits keep-edits without new code.
-   `GIT-IT-RIGHT.md` is a `&str` constant beside the other templates; install instructions link to upstream `README.md#install` instead of copying them, so they cannot drift.
-   Rejected: a `.girconfig` opt-out key (deletion alone would do nothing; operator chose the index signal), and making `--force` the only restore path (it overwrites an edited `.girconfig`; operator chose a narrower flag).
-   Verification strategy: end-to-end tests per Claim, full suite and clippy on Windows and Linux, dogfood run here.
-   DECIDE gate: `ESTABLISHED`.

## Implement

-   `src/cmd/init.rs`: `init(force, optional)`; adoption check; optional-file loop over `cliff.toml` and `GIT-IT-RIGHT.md`.
-   `src/cmd/templates.rs`: `GIT_IT_RIGHT_MD`.
-   `src/main.rs`: `init` accepts `--optional`; usage line `gir init [--force] [--optional]`.
-   `tests/init.rs`: six tests; `tests/args.rs`: usage assertion names `--optional`.
-   `README.md` and `src/explain.md` (`## hooks`) describe optional files and `--optional`.
-   `GIT-IT-RIGHT.md` added to this repository by the built `gir init --optional`.
-   Commits: `d8bbeb2` (feature), `539416b` (dogfooded file).
-   IMPLEMENT gate: `ESTABLISHED`.

## Verify

<a id="verification-init-recreates-missing"></a>
### Verification: `init-recreates-missing`

- Claim: [init-recreates-missing](#init-recreates-missing)
- Method: new test `adopted_init_skips_missing_optional_files` run against `src/` at `04f9cee`.
- Evidence considered: [red run](logs/test-red-20260926-1846.log) — the test panics at `adopted init leaves the removed cliff.toml absent`. SUPPORTS.
- Conclusion: `VERIFIED` — the pre-change binary recreated a removed `cliff.toml` in an adopted repository.
- Limitations: Windows only; the code path has no platform branch.

<a id="verification-adopted-skips-optional"></a>
### Verification: `adopted-skips-optional`

- Claim: [adopted-skips-optional](#adopted-skips-optional)
- Method: `adopted_init_skips_missing_optional_files` on the final change.
- Evidence considered: `ok` in the [Windows run](logs/test-docs-20260926-1847.log) and the [Linux run](logs/gate-linux-539416b-20260926-1849.log); it failed before the change (red run), so it detects the defect. SUPPORTS.
- Conclusion: `VERIFIED`.
- Limitations: none blocking.

<a id="verification-adopted-restores-required"></a>
### Verification: `adopted-restores-required`

- Claim: [adopted-restores-required](#adopted-restores-required)
- Method: `adopted_init_restores_missing_required_files` removes `.githooks/pre-push` from disk and index and unsets `core.hooksPath` in an adopted repository.
- Evidence considered: `ok` on Windows and Linux (same logs). SUPPORTS.
- Conclusion: `VERIFIED`.
- Limitations: covers one hook; both hooks share one loop.

<a id="verification-fresh-writes-all"></a>
### Verification: `fresh-writes-all`

- Claim: [fresh-writes-all](#fresh-writes-all)
- Method: `init_writes_git_it_right_md_in_a_fresh_repository` plus existing `init_creates_files_and_reports_each_write`.
- Evidence considered: both `ok` on Windows and Linux; the new test failed in the red run. SUPPORTS.
- Conclusion: `VERIFIED`.
- Limitations: none.

<a id="verification-optional-flag"></a>
### Verification: `optional-flag`

- Claim: [optional-flag](#optional-flag)
- Method: `optional_flag_writes_missing_optional_files_and_keeps_edits` — missing `GIT-IT-RIGHT.md`, edited `cliff.toml` and `.girconfig`, run `gir init --optional`.
- Evidence considered: `ok` on Windows and Linux (exit `1`, file written, both edits kept with `gir: kept`); the [dogfood run](logs/dogfood-20260926-1848.log) wrote only `GIT-IT-RIGHT.md`. SUPPORTS.
- Conclusion: `VERIFIED`.
- Limitations: `--optional` together with `--force` is not tested; the spec does not claim it.

<a id="verification-force-restores-optional"></a>
### Verification: `force-restores-optional`

- Claim: [force-restores-optional](#force-restores-optional)
- Method: `force_writes_missing_optional_files_in_an_adopted_repository`.
- Evidence considered: `ok` on Windows and Linux; failed in the red run (no `GIT-IT-RIGHT.md` template yet). SUPPORTS.
- Conclusion: `VERIFIED`.
- Limitations: none.

<a id="verification-git-it-right-content"></a>
### Verification: `git-it-right-content`

- Claim: [git-it-right-content](#git-it-right-content)
- Method: `git_it_right_md_is_static_and_covers_onboarding` compares the file across default and custom `.girconfig` and checks for `Conventional Commits`, `gir init`, `gir explain`, `cliff.toml`, `git-cliff`, `gir lint --range`; the section structure was read in `src/cmd/templates.rs`.
- Evidence considered: `ok` on Windows and Linux. SUPPORTS. The keyword check alone cannot show the text is good onboarding.
- Conclusion: `VERIFIED` for the static property and topic coverage.
- Limitations: wording quality is an operator judgment, open for their review of `GIT-IT-RIGHT.md`; non-blocking.

<a id="verification-suite-green"></a>
### Verification: `suite-green`

- Claim: [suite-green](#suite-green)
- Method: `cargo test` and `cargo clippy --all-targets -- -D warnings` on Windows; the same at `539416b` in a fresh clone in WSL2 Arch Linux (uid `1000`).
- Evidence considered: [Windows tests](logs/test-docs-20260926-1847.log) (14 `test result: ok`, exit `0`), [Windows clippy](logs/clippy-20260926-1847.log) (exit `0`), [Linux gate](logs/gate-linux-539416b-20260926-1849.log) (`clippy exit=0`, `test exit=0`). The Windows and Linux test sets differ only by five platform-gated `doctor` tests that predate this Task. SUPPORTS.
- Conclusion: `VERIFIED`.
- Limitations: one run per platform.

<a id="verification-dogfood"></a>
### Verification: `dogfood`

- Claim: [dogfood](#dogfood)
- Method: built `target/debug/gir.exe` run in this worktree; `git status --short` before, after `gir init`, after `gir init --optional`.
- Evidence considered: [dogfood log](logs/dogfood-20260926-1848.log) — plain `init` exit `0`, no file changes; `--optional` prints `gir: wrote GIT-IT-RIGHT.md`, exit `0`, and the only new status entry is `?? GIT-IT-RIGHT.md`. SUPPORTS.
  Side effect: plain `init` printed `gir: set core.hooksPath=.githooks (this clone)` — the shared `.git/config` had no `core.hooksPath`; it is now set for every checkout of this repository.
- Conclusion: `VERIFIED`.
- Limitations: none.

VERIFY gate: `ESTABLISHED`.

## Learn

### Technical

The existing `write()` already separated "missing" from "differs", so the whole behavior change is one condition in `init`; no new write mode was needed.

### Process

The harness's worktree isolation refuses the Write tool and complex shell commands that target the main checkout's `.scratch/`, while simple redirected commands succeed; logs were written by redirection, and the Linux script was passed inline.
This is harness friction, not addf friction; no framework change.

LEARN gate: `ESTABLISHED`.

## Retention and promotion

No material Learning needs a permanent change.

### Promotion: `init-recreates-missing`

-   Claim: [init-recreates-missing](#init-recreates-missing)
-   Will this Claim's validity outlive this Task and inform a future decision? `no`, it described pre-change behavior that this Task removed.
-   Disposition: not promoted — Task-scoped only.

The success Claims are specified behavior now published in `spec/init.md`; they are covered by tests and need no Knowledge file.
No Claim promoted.

## Archive readiness

The bundle holds its ledger and every log it cites under `logs/`; internal links are relative.
External references (`framework:` spec and evidence) are supplemental.

## Terminal record

### Summary

`gir init` now generates `GIT-IT-RIGHT.md`, skips missing optional files in a repository that tracks `.girconfig`, and restores them with `--optional` (keeping edits) or `--force`.
The spec delta is published in `spec/init.md` and `spec/cli.md`.

### Gate basis

All eight success Claims and the material premise are `VERIFIED` through the Verifications above, on Windows and Linux where scoped.
macOS remains assumed, not verified, as recorded before this Task; no Claim here is scoped to it.
No open Claim is carried forward.

## Stop record

`NONE`
