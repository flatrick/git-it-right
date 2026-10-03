# TASK — `record as Knowledge what a reference-transaction veto leaves behind`

## Resume

**Contract version:** `2`

**State:** `DECIDE`

**State path:** `DEFINE -> UNDERSTAND -> DECIDE`

**Resume at:** Record the Knowledge files' names, content and verification under Decide.

**Open obligations:** The publication plan recorded — blocks the DECIDE gate.

## Owned artifacts

-   `ledger.md` - the finding this Task records, the operator's answers, and its DEFINE dialogue.
-   `probe-veto.py` - Probe: each operation under two hook designs, with the recoveries.
-   `logs/probe-veto-20261003-1925.log` - Evidence: first run, one hook, `git reset --hard HEAD` only.
-   `logs/probe-veto-20261003-1935.log` - Evidence: both hooks, `merge --abort` and `rebase --abort` added.
-   `logs/probe-veto-20261003-1945.log` - Evidence: the run of the current `probe-veto.py`, with the hook's input logged and `rebase --quit` added.

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

The findings below were added in UNDERSTAND from `probe-veto.py`; each is the Claim a Knowledge file will carry, word for word, because a promoted Knowledge Claim must match the Task Claim its Verification established.

<a id="f1-ff-merge-veto"></a>
#### `f1-ff-merge-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git merge --ff-only` exits `128` with the branch unmoved, but the index and working tree already hold the incoming commits' changes, staged, beside any uncommitted edits made before; `git reset --hard HEAD` then returns the index and tracked files to the branch's commit, discarding those earlier tracked edits and keeping untracked files, and when the hook also refuses an update that leaves the branch where it is, the reset still does this but exits `128`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused fast-forward leaves behind.
-   Basis: pending; observed in `logs/probe-veto-20261003-1945.log`.

<a id="f2-commit-veto"></a>
#### `f2-commit-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git commit` exits `128` with the branch unmoved and the index and working tree unchanged, so the changes stay staged; `git reset --hard HEAD` then returns the index and tracked files to the branch's commit, discarding the staged and the unstaged tracked changes and keeping untracked files, and when the hook also refuses an update that leaves the branch where it is, the reset still does this but exits `128`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused commit leaves behind.
-   Basis: pending; observed in `logs/probe-veto-20261003-1945.log`.

<a id="f3-merge-commit-veto"></a>
#### `f3-merge-commit-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git merge --no-ff` exits `128` with the branch unmoved, the merged changes staged beside any earlier uncommitted edits, and `MERGE_HEAD` left, so a merge stays in progress; when the hook accepts an update that leaves the branch where it is, `git merge --abort` ends the merge and restores the branch's tree while keeping the earlier uncommitted edits, and `git reset --hard HEAD` ends it while discarding them; when the hook refuses every update of the branch, `git merge --abort` exits `128` and changes nothing, and `git reset --hard HEAD` exits `128` after resetting the files, leaving `MERGE_HEAD` in place.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused merge leaves behind.
-   Basis: pending; observed in `logs/probe-veto-20261003-1945.log`.

<a id="f4-pull-veto"></a>
#### `f4-pull-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, a fast-forward `git pull` exits `128` with the branch unmoved, but the index and working tree already hold the pulled changes, staged, beside any uncommitted edits made before; `git reset --hard HEAD` then returns the index and tracked files to the branch's commit, discarding those earlier tracked edits and keeping untracked files, and when the hook also refuses an update that leaves the branch where it is, the reset still does this but exits `128`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused pull leaves behind.
-   Basis: pending; observed in `logs/probe-veto-20261003-1945.log`.

<a id="f5-rebase-veto"></a>
#### `f5-rebase-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git rebase` exits `128` with the branch unmoved, `HEAD` detached at the rebased commit and the rebase still in progress; `git rebase --abort` then exits `128` and leaves the rebase in progress, because it updates the branch with an all-zero old value, which a hook comparing old and new values treats as a change; `git reset --hard HEAD` leaves `HEAD` detached and the rebase in progress; `git rebase --quit` ends the rebase with `HEAD` still detached and moves an `--autostash` stash into the stash list.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused rebase leaves behind.
-   Basis: pending; observed in `logs/probe-veto-20261003-1945.log`.

## Understand

### Relevant context

-   git 2.56.0's `githooks` documentation: the hook runs for `preparing`, `prepared`, `committed` and `aborted`; a non-zero exit in `preparing` or `prepared` aborts the transaction; each update arrives on stdin as `<old> <new> <ref>`, with an all-zero `<old>` for a forced update. It says nothing about the working tree or index.
-   `probe-veto.py` (`logs/probe-veto-20261003-1945.log`, 20 cases) runs each operation from `main` with a pre-existing tracked edit (`notes.txt`) and an untracked file (`scratch.txt`), under two hooks: `any` refuses every update of `refs/heads/main`; `moves` refuses only when old and new differ. Earlier runs: `-1925.log` (one hook, `reset --hard` only), `-1935.log` (both hooks, no logged hook input, no `--quit`).
-   Observed: f1–f5 above; the phase-0 result `p1-veto-leaves-staged` reproduces in the `k1` cases.
-   Not covered: Windows, a hook outside `.git/hooks` (`core.hooksPath`), a hook that compares `new` with the ref's current value rather than with `old`, and `git switch main` after `rebase --quit`.

### UNDERSTAND gate

`ESTABLISHED`: the hook's documented contract is recorded, and the probe observed every operation in scope under both hook designs with the agreed recoveries. No open question remains for INVESTIGATE.

## Investigate

INVESTIGATE gate: `NOT_APPLICABLE` — skipped. The probe ran in UNDERSTAND and left no decision-relevant uncertainty; what it did not cover is listed there and becomes each Knowledge file's Limitations.

## Decide

-   Five files under `knowledge/`, one per finding, from `templates/KNOWLEDGE.md`: `ref-transaction-veto-ff-merge.md` (f1, k1), `ref-transaction-veto-commit.md` (f2, k2), `ref-transaction-veto-merge-commit.md` (f3, k3), `ref-transaction-veto-pull.md` (f4, k4), `ref-transaction-veto-rebase.md` (f5, k5).
-   Each Claim and Scope repeat f1–f5 word for word; State `VERIFIED`; Basis `history:tasks/ref-transaction-veto-knowledge/TASK.md#verification-<finding>`; evidence anchors `probe-veto.py` and `logs/probe-veto-20261003-1945.log` in the archived bundle; Derivation names the probe case and the observed lines; Limitations lists what Understand records as not covered and says gir installs no `reference-transaction` hook today (`ledger.md` A5).
-   `INDEX.md`'s Knowledge section lists the five files.
-   They are written in the terminal checkpoint, as Stewardship requires; IMPLEMENT has no repository change of its own and is skipped.
-   Verification: f1–f5 and `p1-veto-leaves-staged` from the probe log; k1–k5 by the publication in the terminal checkpoint, where `check-capsule` requires each Knowledge Basis to resolve to the matching Verification with the same Claim and Scope.
-   Rejected: one Knowledge file for all five operations (addf allows one Claim per file); prescribing a hook design (Knowledge describes, it does not prescribe).

### DECIDE gate

`ESTABLISHED`: the plan follows the operator's answers and addf's Knowledge contract, and each criterion has a check.
