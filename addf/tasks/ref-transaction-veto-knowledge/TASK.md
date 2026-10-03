# TASK — `record as Knowledge what a reference-transaction veto leaves behind`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Read git 2.56.0's `githooks` documentation for `reference-transaction`, then write the probe for the five operations.

**Open obligations:** The hook's documented contract and the probe design recorded — blocks the UNDERSTAND gate.

## Owned artifacts

-   `ledger.md` - the finding this Task records, the operator's answers, and its DEFINE dialogue.

## Specification impact

- Current contract: `NONE` — gir installs no `reference-transaction` hook; no requirement covers it.
- Proposed delta: `NONE`
- Terminal publication: `PENDING`

## Define

### Objective

`A future decision about guarding a branch with a reference-transaction hook can rely on a verified Knowledge file stating what git leaves behind when such a hook vetoes an update.`

DEFINE gate: `ESTABLISHED` — the operator agreed to the objective and success criteria k1–k5 (`ledger.md` Q7, A7).

### Success criteria

Each criterion: a Knowledge file under `knowledge/`, `VERIFIED` with its Basis in this Task's Verification and evidence anchors to a probe in this bundle, stating what a `prepared`-phase `reference-transaction` veto of the update of the checked-out branch leaves in the ref, the index and the working tree for one operation, and, as an observation, what `git reset --hard HEAD` then restores and what it does to uncommitted changes that existed before the operation (`ledger.md` A4–A6).

<a id="k1-ff-merge"></a>
#### `k1-ff-merge`

-   Claim: The criterion above holds for `git merge --ff-only`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: the finding that started this Task stays unrecorded.
-   Basis: none yet; pending.

<a id="k2-commit"></a>
#### `k2-commit`

-   Claim: The criterion above holds for a direct `git commit`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: a guard design misjudges vetoed commits.
-   Basis: none yet; pending.

<a id="k3-merge-commit"></a>
#### `k3-merge-commit`

-   Claim: The criterion above holds for `git merge --no-ff`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: a guard design misjudges vetoed merges.
-   Basis: none yet; pending.

<a id="k4-pull"></a>
#### `k4-pull`

-   Claim: The criterion above holds for `git pull` from a remote whose branch is ahead.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: a guard design misjudges vetoed pulls.
-   Basis: none yet; pending.

<a id="k5-rebase"></a>
#### `k5-rebase`

-   Claim: The criterion above holds for `git rebase` of the checked-out branch.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: a guard design misjudges vetoed rebases.
-   Basis: none yet; pending.

### Constraints

-   Isolated: worktree `.worktrees/knowledge-ref-transaction-veto`, branch `knowledge-ref-transaction-veto`, from `3e10963` (`ledger.md` A3).
-   No change to gir's code or specification.
-   Nothing under `.agents/` is committed.

### Material empirical premises

<a id="p1-veto-leaves-staged"></a>
#### `p1-veto-leaves-staged`

-   Claim: With git 2.56.0, a `reference-transaction` hook that exits non-zero in `prepared` for `refs/heads/main` makes `git merge --ff-only` fail with `main` unmoved and the incoming changes staged in the index and present in the working tree.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: there is nothing to record, or a different behaviour to record.
-   Basis: the phase-0 probe (`ledger.md` A1), whose logs were not kept; to be re-run as a Probe owned by this Task.
