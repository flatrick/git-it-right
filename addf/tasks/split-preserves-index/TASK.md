# TASK — `gir fixup --split leaves untouched index entries alone`

## Resume

**Contract version:** `2`

**State:** `INVESTIGATE`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE`

**Resume at:** Probe whether a temporary index (`GIT_INDEX_FILE`) leaves the real index unchanged through read-tree, apply and commit.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`; `p1-split-drops-flags` blocks `UNDERSTAND`; re-reading the spec after `diff-header-parsing` terminalizes blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the thread entries this Task took, and the questions that shaped it with the operator's answers.
-   `probe_split_index.py` - Probe: a sparse checkout, a `skip-worktree` file with a local edit, and a `git add -N` file, each through a successful `--split` and one a `commit-msg` hook makes fail.
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

`PENDING`

## Decide

`PENDING`

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
