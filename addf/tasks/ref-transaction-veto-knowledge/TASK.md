# TASK — `record as Knowledge what a reference-transaction veto leaves behind`

## Resume

**Contract version:** `2`

**State:** `DEFINE`

**State path:** `DEFINE`

**Resume at:** DEFINE dialogue with the operator: the claim's scope (which git operations), and agreement on the objective and success criterion.

**Open obligations:** Objective, scope and success criterion agreed with the operator — blocks the DEFINE gate.

## Owned artifacts

-   `ledger.md` - the finding this Task records, the operator's answers, and its DEFINE dialogue.

## Specification impact

- Current contract: `NONE` — gir installs no `reference-transaction` hook; no requirement covers it.
- Proposed delta: `NONE`
- Terminal publication: `PENDING`

## Define

### Objective

`A future decision about guarding a branch with a reference-transaction hook can rely on a verified Knowledge file stating what git leaves behind when such a hook vetoes an update.`

Draft; not yet agreed with the operator.

### Success criteria

Draft; not yet agreed with the operator.

<a id="k1-knowledge-published"></a>
#### `k1-knowledge-published`

-   Claim: A Knowledge file under `knowledge/` states, as `VERIFIED` with a Basis in this Task's Verification and evidence anchors to a probe in this bundle, what a `prepared`-phase `reference-transaction` veto of a branch update leaves in the working tree, the index and the ref, for the operations agreed in DEFINE.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0.
-   Consequence if false: the finding stays only in a removed, uncommitted memo.
-   Basis: none yet; Task in DEFINE.

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
