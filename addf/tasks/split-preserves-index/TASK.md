# TASK — `gir fixup --split leaves untouched index entries alone`

## Resume

**Contract version:** `2`

**State:** `DECIDE`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE`

**Resume at:** Record the temporary-index design under Decide, then transition to `IMPLEMENT`.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`; `p1-split-drops-flags` blocks `UNDERSTAND`; re-reading the spec after `diff-header-parsing` terminalizes blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `probe_split_index.py` - Probe: a sparse checkout, a `skip-worktree` file with a local edit, and a `git add -N` file, each through a successful `--split` and one a `commit-msg` hook makes fail.
-   `probe_temp_index.py` - Probe: the split loop run by hand against a temporary index (`GIT_INDEX_FILE`), with the real index hashed before and after.
-   `logs/probe-temp-index-20261002-1020.log` - Evidence: that probe's output.
-   `logs/probe-head-ae3d2ca-20261002-1019.log` - Evidence: the probe at `ae3d2ca`, before any change.

## Specification impact

- Current contract: `framework:spec/fixup.md#req-fixup-split`, `framework:spec/fixup.md#req-fixup-split-rollback`
- Proposed delta: split and split-rollback also leave the `skip-worktree` and intent-to-add state of every index entry as it was before the command, and change no entry the split does not commit.
- Terminal publication: `PENDING`

## Define

### Objective

`gir fixup --split` leaves alone every index entry it does not commit.

### Success criteria

<a id="sp-entries-kept"></a>
#### `sp-entries-kept`

-   Claim: After a successful `gir fixup --split`, and after a failed one that restores, `skip-worktree` bits, intent-to-add entries and the sparse-checkout view are as they were before the command, and `git status` differs from before only by the effect of the new commits.
-   State: `UNVERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: a later `git add -A` or `git commit -a` deletes sparse files, commits private edits, or loses intent-to-add entries.
-   Basis: pending check.

<a id="sp-tests"></a>
#### `sp-tests`

-   Claim: Integration tests cover a sparse checkout, a `skip-worktree` file with a local edit, and a `git add -N` file, each for a successful split and for a restored failure.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: pending check.

<a id="sp-gates-green"></a>
#### `sp-gates-green`

-   Claim: `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: regressions ship.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Overlaps `diff-header-parsing` in `src/cmd/fixup.rs`. Order: `diff-header-parsing` publishes and terminalizes first; this Task re-reads the current specification after that, before its own `VERIFY`.
-   Not promised: the stat-cache cost (Q8). If the fix also cures it, record that under Verify.

### Material empirical premises

<a id="p1-split-drops-flags"></a>
#### `p1-split-drops-flags`

-   Claim: At this Task's starting revision, after a successful `gir fixup --split` in a sparse checkout, `git status` shows the files outside the sparse set as deleted.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, `ae3d2ca`.
-   Consequence if false: the review's reproduction does not hold here, and the defect needs re-establishing.
-   Basis: [Verification](#verification-p1-split-drops-flags).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

-   `split` in `src/cmd/fixup/split.rs` records `orig` (`HEAD`) and `goal` (`git write-tree` of the staged index), then `commit_each` runs, per target, `git read-tree <orig>`, `git apply --cached --unidiff-zero` of that target's and earlier targets' hunks, and `commit` (`git commit --fixup=...` through `git::passthrough`, inheriting stdio and environment). On failure it runs `git reset --soft <orig>` and `git read-tree <goal>`.
-   Every `read-tree` rebuilds the real index from a tree, which drops `skip-worktree` bits, intent-to-add entries and cached stat data for every path, not only the ones the split commits.
-   The probe at `ae3d2ca` shows all six cases changed: a sparse file shows ` D`, a `skip-worktree` file with a local edit shows ` M`, an intent-to-add file becomes `??`, and `git ls-files -v` loses the `S` flags, after a success and after a restored failure alike.
-   `git::run`, `run_with_stdin` and `passthrough` in `src/git.rs` take no environment, so a temporary index needs a way to pass `GIT_INDEX_FILE`.

### Assumptions

-   `write-tree` leaves intent-to-add entries out of `goal`; source: git's documented behavior, consistent with the probe (the split commits do not contain `ita`); not separately verified.

### Open questions

-   Do `git read-tree`, `git apply --cached` and `git commit` with `GIT_INDEX_FILE` set to a temporary file leave the real index byte-for-byte unchanged, also in a sparse checkout and when a `commit-msg` hook runs? Settled in `INVESTIGATE`.

### Deferred verification

-   `NONE`.

### UNDERSTAND gate

`ESTABLISHED`: the defect reproduces at `HEAD`, the cause is the `read-tree` of the real index, and the one uncertainty that decides the design is named above.

## Investigate

-   Whether a temporary index leaves the real one alone: resolved by `probe_temp_index.py` (`logs/probe-temp-index-20261002-1020.log`). For a sparse checkout in cone mode with a sparse index, in non-cone mode, a `skip-worktree` file with a local edit, and a `git add -N` file, `read-tree`, `apply --cached` and `commit --fixup` against `GIT_INDEX_FILE` left `.git/index` byte-for-byte unchanged, both when both commits succeeded and when a `commit-msg` hook refused the second. On success `HEAD`'s tree equals the staged tree and `git status` shows only the untouched entries; on failure `git reset --soft` alone restores `HEAD`.
-   The `write-tree` assumption: consistent with the probe, where `ita` stays out of the commits; no decision depends on it further.

### INVESTIGATE gate

`ESTABLISHED`: the deciding uncertainty is resolved, and nothing is deferred.

## Decide

-   **Temporary index.** `--split` builds every round in a temporary index file inside the git directory and never writes the real one. `read-tree`, `apply --cached` and `commit` run with `GIT_INDEX_FILE` set to it.
-   **`git::TempIndex`.** A small type in `src/git.rs` owns the file: it names it `gir-split-index-<pid>` in `git rev-parse --absolute-git-dir`, offers `run`, `run_with_stdin` and `passthrough` that set `GIT_INDEX_FILE`, and deletes the file when dropped, on success and failure alike. The existing three functions share one private builder with it, so there is no copied process-handling code.
-   **Final check.** The check that the split adds up compares `HEAD^{tree}` with the staged tree; comparing `write-tree` of the untouched real index would always pass.
-   **Rollback.** `git reset --soft <orig>` only; the `read-tree <goal>` that rebuilt the index goes away. The message stays `; restored HEAD and the index`, because the index is as staged.
-   **Spec delta.** `split`: also leaving the index unchanged. `split-rollback`: leave the index as it was before the command instead of resetting it to the staged tree.
-   **Rejected:** `std::env::set_var` around the loop, which is `unsafe` in edition 2024 and leaks process-global state; resetting only the hunks' paths with `git reset <orig> -- <paths>`, which still rewrites those entries and needs care with intent-to-add; plumbing (`write-tree`, `commit-tree`, `update-ref`), which would skip the user's commit hooks and editor that `git commit` runs today.
-   **Verification strategy.** Tests first in `tests/fixup_modes.rs`: for a sparse checkout, a `skip-worktree` file with a local edit and a `git add -N` file, a successful split and a hook-refused split each leave `git status` (apart from the committed paths), `git ls-files -v` and the bytes of `.git/index` as before. Then rerun `probe_split_index.py`, `cargo test` and `cargo clippy --all-targets -- -D warnings`.

### DECIDE gate

`ESTABLISHED`: the design is probed, and each success Claim has a planned test.

## Implement

`PENDING`

## Verify

<a id="verification-p1-split-drops-flags"></a>
### Verification: `p1-split-drops-flags`

- Claim: [p1-split-drops-flags](#p1-split-drops-flags)
- Method: built `target/debug/gir` at `ae3d2ca` and ran `probe_split_index.py`.
- Evidence considered: `logs/probe-head-ae3d2ca-20261002-1019.log`, "sparse checkout, success": status after the split is ` D out/c`, and `git ls-files -v` changes `S out/c` to `H out/c`.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.56.0.

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
