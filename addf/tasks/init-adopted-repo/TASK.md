# TASK — `gir init respects removed optional files and generates GIT-IT-RIGHT.md`

## Resume

**Contract version:** `2`

**State:** `DEFINE`

**State path:** `DEFINE`

**Resume at:** Operator confirms the Objective, Constraints and Success criteria below; then record the DEFINE gate as `ESTABLISHED` and move to `UNDERSTAND`.

**Open obligations:** three items.
-   Operator agreement on objective, scope and success criteria — blocks the DEFINE gate.
-   Spec delta written into `spec/init.md` and `spec/cli.md` — blocks Complete (Stewardship publication).
-   macOS behavior — deferred; no macOS host (same as prior Tasks).

## Owned artifacts

-   `ledger.md` - pre-Task and DEFINE questions with the operator's answers.

## Specification impact

- Current contract: `framework:spec/init.md`, `framework:spec/cli.md#req-cli-usage-lists-commands`, `framework:spec/cli.md#req-cli-repository-root`
- Proposed delta: changes to the `init` and `cli` modules.
  - init: define *optional files* (`cliff.toml`, `GIT-IT-RIGHT.md`) and *required files* (`.githooks/commit-msg`, `.githooks/pre-push`, `.girconfig`).
  - init: add a requirement that `gir init` creates `GIT-IT-RIGHT.md` with static content covering what gir is, a contributor quick-start, git-cliff, and a no-CI maintainer check.
  - init: add *adopted repository* — `.girconfig` is in the index. In an adopted repository, `gir init` without flags SHALL NOT create a missing optional file and SHALL print nothing about it; it SHALL still create missing required files.
  - init: add `--optional` — `gir init --optional` also creates each missing optional file and never replaces an existing file (existing-file rules unchanged).
  - init: `--force` also creates missing optional files in an adopted repository.
  - cli: accepted flags become `init --force --optional`; usage text lists `--optional`.
- Terminal publication: `PENDING`

## Define

### Objective

In a repository that has adopted gir, a contributor's `gir init` activates their clone without recreating optional files the maintainer removed.
`gir init` also generates a `GIT-IT-RIGHT.md` that explains gir to new contributors, as an optional file on the same terms as `cliff.toml`.
A maintainer can get a removed optional file back with `gir init --optional` without overwriting edited files.

### Success criteria

<a id="adopted-skips-optional"></a>
#### `adopted-skips-optional`

-   Claim: With `.girconfig` in the index and `cliff.toml` and `GIT-IT-RIGHT.md` absent, `gir init` exits `0`, creates neither file, and prints no stderr line naming either.
-   State: `UNVERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: a maintainer's removal is still undone by contributors.
-   Basis: pending end-to-end test in `tests/init.rs`.

<a id="adopted-restores-required"></a>
#### `adopted-restores-required`

-   Claim: With `.girconfig` in the index and a hook file absent, `gir init` recreates and stages that hook and sets `core.hooksPath`.
-   State: `UNVERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: contributors can't activate gir after a hook was lost.
-   Basis: pending end-to-end test.

<a id="fresh-writes-all"></a>
#### `fresh-writes-all`

-   Claim: In a repository where `.girconfig` is not in the index, `gir init` creates the hooks, `.girconfig`, `cliff.toml` and `GIT-IT-RIGHT.md`, and prints `gir: wrote GIT-IT-RIGHT.md`.
-   State: `UNVERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: first adoption misses the onboarding file.
-   Basis: pending end-to-end test.

<a id="optional-flag"></a>
#### `optional-flag`

-   Claim: In an adopted repository, `gir init --optional` creates each missing optional file and leaves an existing file whose content differs from its template unchanged, with the existing `kept` message and exit `1`.
-   State: `UNVERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: restoring an optional file risks clobbering edits, or does not work.
-   Basis: pending end-to-end test.

<a id="force-restores-optional"></a>
#### `force-restores-optional`

-   Claim: In an adopted repository, `gir init --force` creates missing optional files.
-   State: `UNVERIFIED`
-   Scope: Windows and Linux, this branch.
-   Consequence if false: `--force` stops meaning "write every template".
-   Basis: pending end-to-end test.

<a id="git-it-right-content"></a>
#### `git-it-right-content`

-   Claim: Generated `GIT-IT-RIGHT.md` is static (identical for any `.girconfig`) and covers what gir is and why, a contributor quick-start (install, `gir init` once per clone, `gir explain <rule>`), git-cliff (why `cliff.toml` exists, optional, maintainer commands, safe to delete), and the no-CI `gir lint --range` check plus the CI one-liner.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the file doesn't do its onboarding job.
-   Basis: pending test plus operator review of the text.

<a id="suite-green"></a>
#### `suite-green`

-   Claim: `cargo test` (including the 750 ms commit-msg latency budget) and `cargo clippy --all-targets -- -D warnings` pass.
-   State: `UNVERIFIED`
-   Scope: Windows, this branch; Linux if a host is available.
-   Consequence if false: regression.
-   Basis: pending run with logs under `.scratch/init-activate-only/`.

<a id="dogfood"></a>
#### `dogfood`

-   Claim: Running the built `gir init --optional` in this repository adds `GIT-IT-RIGHT.md` and changes no other tracked file.
-   State: `UNVERIFIED`
-   Scope: this worktree.
-   Consequence if false: the flag misbehaves on a real adopted repo.
-   Basis: pending run.

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
-   State: `UNVERIFIED`
-   Scope: `main` at `04f9cee`.
-   Consequence if false: the fix half of this Task has no problem to solve.
-   Basis: code reading of `src/cmd/init.rs` `write()` (a missing file falls through to `std::fs::write`); pending a reproducing test before the change.

## Understand

### Relevant context

`PENDING`

### Assumptions

`PENDING`

### Open questions

`PENDING`

### Deferred verification

-   macOS behavior; no macOS host; checkpoint: any future macOS run; consequence if false: platform-specific init bug; blocks nothing in this Task.

## Investigate

`PENDING`

## Decide

`PENDING`

## Implement

`PENDING`

## Verify

`PENDING`

## Learn

`PENDING`

## Retention and promotion

`PENDING`

## Archive readiness

`PENDING`

## Terminal record

`PENDING`

## Stop record

`NONE`
