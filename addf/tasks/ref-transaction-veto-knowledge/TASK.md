# TASK — `record as Knowledge what a reference-transaction veto leaves behind`

## Resume

**Contract version:** `2`

**State:** `LEARN`

**State path:** `DEFINE -> UNDERSTAND -> DECIDE -> VERIFY -> LEARN`

**Resume at:** Write Learn and Retention and promotion, then the terminal checkpoint with the five Knowledge files.

**Open obligations:** Learn and Retention and promotion recorded — blocks the LEARN gate. Five Knowledge files and their index entries — terminal checkpoint.

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
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: the finding that started this Task stays unrecorded.
-   Basis: [Verification](#verification-k1-ff-merge).

<a id="k2-commit"></a>
#### `k2-commit`

-   Claim: The criterion above holds for a direct `git commit`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: a guard design misjudges vetoed commits.
-   Basis: [Verification](#verification-k2-commit).

<a id="k3-merge-commit"></a>
#### `k3-merge-commit`

-   Claim: The criterion above holds for `git merge --no-ff`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: a guard design misjudges vetoed merges.
-   Basis: [Verification](#verification-k3-merge-commit).

<a id="k4-pull"></a>
#### `k4-pull`

-   Claim: The criterion above holds for `git pull` from a remote whose branch is ahead.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: a guard design misjudges vetoed pulls.
-   Basis: [Verification](#verification-k4-pull).

<a id="k5-rebase"></a>
#### `k5-rebase`

-   Claim: The criterion above holds for `git rebase` of the checked-out branch.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: a guard design misjudges vetoed rebases.
-   Basis: [Verification](#verification-k5-rebase).

### Constraints

-   Isolated: worktree `.worktrees/knowledge-ref-transaction-veto`, branch `knowledge-ref-transaction-veto`, from `3e10963` (`ledger.md` A3).
-   No change to gir's code or specification.
-   Nothing under `.agents/` is committed.

### Material empirical premises

<a id="p1-veto-leaves-staged"></a>
#### `p1-veto-leaves-staged`

-   Claim: With git 2.56.0, a `reference-transaction` hook that exits non-zero in `prepared` for `refs/heads/main` makes `git merge --ff-only` fail with `main` unmoved and the incoming changes staged in the index and present in the working tree.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook.
-   Consequence if false: there is nothing to record, or a different behaviour to record.
-   Basis: [Verification](#verification-p1-veto-leaves-staged).

The findings below were added in UNDERSTAND from `probe-veto.py`; each is the Claim a Knowledge file will carry, word for word, because a promoted Knowledge Claim must match the Task Claim its Verification established.

<a id="f1-ff-merge-veto"></a>
#### `f1-ff-merge-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git merge --ff-only` exits `128` with the branch unmoved, but the index and working tree already hold the incoming commits' changes, staged, beside any uncommitted edits made before; `git reset --hard HEAD` then returns the index and tracked files to the branch's commit, discarding those earlier tracked edits and keeping untracked files, and when the hook also refuses an update that leaves the branch where it is, the reset still does this but exits `128`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused fast-forward leaves behind.
-   Basis: [Verification](#verification-f1-ff-merge-veto).

<a id="f2-commit-veto"></a>
#### `f2-commit-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git commit` exits `128` with the branch unmoved and the index and working tree unchanged, so the changes stay staged; `git reset --hard HEAD` then returns the index and tracked files to the branch's commit, discarding the staged and the unstaged tracked changes and keeping untracked files, and when the hook also refuses an update that leaves the branch where it is, the reset still does this but exits `128`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused commit leaves behind.
-   Basis: [Verification](#verification-f2-commit-veto).

<a id="f3-merge-commit-veto"></a>
#### `f3-merge-commit-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git merge --no-ff` exits `128` with the branch unmoved, the merged changes staged beside any earlier uncommitted edits, and `MERGE_HEAD` left, so a merge stays in progress; when the hook accepts an update that leaves the branch where it is, `git merge --abort` ends the merge and restores the branch's tree while keeping the earlier uncommitted edits, and `git reset --hard HEAD` ends it while discarding them; when the hook refuses every update of the branch, both still restore the files as described but exit `128` and leave `MERGE_HEAD` in place.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused merge leaves behind.
-   Basis: [Verification](#verification-f3-merge-commit-veto).

<a id="f4-pull-veto"></a>
#### `f4-pull-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, a fast-forward `git pull` exits `128` with the branch unmoved, but the index and working tree already hold the pulled changes, staged, beside any uncommitted edits made before; `git reset --hard HEAD` then returns the index and tracked files to the branch's commit, discarding those earlier tracked edits and keeping untracked files, and when the hook also refuses an update that leaves the branch where it is, the reset still does this but exits `128`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused pull leaves behind.
-   Basis: [Verification](#verification-f4-pull-veto).

<a id="f5-rebase-veto"></a>
#### `f5-rebase-veto`

-   Claim: When a `reference-transaction` hook exits non-zero in the `prepared` state for the update of the checked-out branch, `git rebase` exits `128` with the branch unmoved, `HEAD` detached at the rebased commit and the rebase still in progress; `git rebase --abort` then returns the working tree and index to the branch's tree but exits `128` with `HEAD` still detached and the rebase in progress, because it updates the branch with an all-zero old value, which a hook comparing old and new values treats as a change; `git reset --hard HEAD` leaves `HEAD` detached and the rebase in progress; `git rebase --quit` ends the rebase with `HEAD` still detached and moves an `--autostash` stash into the stash list.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, a shell hook in `.git/hooks`.
-   Consequence if false: a guard design misjudges what a refused rebase leaves behind.
-   Basis: [Verification](#verification-f5-rebase-veto).

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

## Implement

IMPLEMENT gate: `NOT_APPLICABLE` — skipped. The deliverable is five Knowledge files, which Stewardship publishes only in the terminal checkpoint; nothing else in the repository changes.

## Verify

Every Verification below rests on `logs/probe-veto-20261003-1945.log` (git 2.56.0, Linux): 20 cases, each listing the operation's and the recovery's exit code and output, the branch and `HEAD`, `git status`, the files, the state files under `.git`, the stash list, and the content of the pre-existing tracked edit (`notes.txt`) and untracked file (`scratch.txt`). Hook `any` refuses every update of `refs/heads/main`; hook `moves` refuses only one whose old and new values differ.

<a id="verification-p1-veto-leaves-staged"></a>
### Verification: `p1-veto-leaves-staged`

- Claim: [`p1-veto-leaves-staged`](#p1-veto-leaves-staged).
- Method: the two `k1: git merge --ff-only` cases.
- Evidence considered: under both hooks, `exit 128` with `fatal: in 'prepared' phase, update aborted by the reference-transaction hook`, `main` unchanged, status `A  incoming.txt` beside ` M notes.txt`.
- Conclusion: `VERIFIED`; the phase-0 result reproduces.
- Limitations: Linux, git 2.56.0, a shell hook in `.git/hooks`.

<a id="verification-f1-ff-merge-veto"></a>
### Verification: `f1-ff-merge-veto`

- Claim: [`f1-ff-merge-veto`](#f1-ff-merge-veto).
- Method: the `k1` cases, followed by `git reset --hard HEAD`.
- Evidence considered: after the veto, `main` unmoved, `A  incoming.txt` staged and ` M notes.txt` kept. After the reset, status `?? scratch.txt` only, `notes.txt` back to `notes`, `incoming.txt` gone; exit `0` under `moves`, exit `128` with the same file result under `any`.
- Conclusion: `VERIFIED`.
- Limitations: as Understand lists.

<a id="verification-f2-commit-veto"></a>
### Verification: `f2-commit-veto`

- Claim: [`f2-commit-veto`](#f2-commit-veto).
- Method: the `k2` cases (a staged new file `new.txt`), followed by `git reset --hard HEAD`.
- Evidence considered: after the veto, status unchanged from before (`A  new.txt`, ` M notes.txt`, `?? scratch.txt`). After the reset, `?? scratch.txt` only, `new.txt` gone, `notes.txt` back to `notes`; exit `0` under `moves`, `128` under `any`.
- Conclusion: `VERIFIED`.
- Limitations: as Understand lists.

<a id="verification-f3-merge-commit-veto"></a>
### Verification: `f3-merge-commit-veto`

- Claim: [`f3-merge-commit-veto`](#f3-merge-commit-veto).
- Method: the four `k3` cases: each hook, then `git reset --hard HEAD` or `git merge --abort`.
- Evidence considered: after the veto, `main` unmoved, `A  incoming.txt` beside ` M notes.txt`, state files `MERGE_HEAD`, `AUTO_MERGE`, `ORIG_HEAD`. Under `moves`: `merge --abort` exit `0`, status ` M notes.txt`, `?? scratch.txt`, only `ORIG_HEAD` left; `reset --hard` exit `0`, `?? scratch.txt` only, `notes.txt` back to `notes`, only `ORIG_HEAD` left. Under `any`: both exit `128` with the same files as under `moves`, and `MERGE_HEAD` and `AUTO_MERGE` still present.
- Conclusion: `VERIFIED`. An earlier draft of this Claim said `merge --abort` under `any` changed nothing; the log contradicted it and the Claim was corrected before this Verification.
- Limitations: as Understand lists.

<a id="verification-f4-pull-veto"></a>
### Verification: `f4-pull-veto`

- Claim: [`f4-pull-veto`](#f4-pull-veto).
- Method: the `k4` cases (a clone whose remote `main` is one commit ahead; `git pull --ff-only`), followed by `git reset --hard HEAD`.
- Evidence considered: the same observations as `f1` in every line: veto exit `128`, `main` unmoved, `A  incoming.txt` beside ` M notes.txt`; reset exit `0` under `moves`, `128` under `any`, with `?? scratch.txt` only afterwards.
- Conclusion: `VERIFIED`.
- Limitations: only a fast-forward pull was probed.

<a id="verification-f5-rebase-veto"></a>
### Verification: `f5-rebase-veto`

- Claim: [`f5-rebase-veto`](#f5-rebase-veto).
- Method: the ten `k5` cases: a clean tree and a dirty tree with `--autostash`, each hook, then `git reset --hard HEAD`, `git rebase --abort` or (dirty tree) `git rebase --quit`.
- Evidence considered: after the veto, `main` unmoved, `HEAD=detached` at the rebased commit, state files `rebase-merge`, `AUTO_MERGE`, `ORIG_HEAD`. `rebase --abort`: exit `128` under both hooks; the hook received `0000000000000000000000000000000000000000 <main's current ID> refs/heads/main`; afterwards `HEAD` still detached, `rebase-merge` still present, status `D  incoming.txt`. `reset --hard`: exit `0`, `HEAD` detached, `rebase-merge` present. `rebase --quit`: exit `0`, `Autostash exists; creating a new stash entry.`, `HEAD` detached, `rebase-merge` gone, stash list `stash@{0}: autostash`.
- Conclusion: `VERIFIED`. An earlier draft said `rebase --abort` left the files untouched; the log shows it resets them, and the Claim was corrected before this Verification.
- Limitations: as Understand lists, including what `git switch main` does afterwards.

<a id="verification-k1-ff-merge"></a>
### Verification: `k1-ff-merge`

- Claim: [`k1-ff-merge`](#k1-ff-merge).
- Method: [Verification of f1](#verification-f1-ff-merge-veto), and publication of `knowledge/ref-transaction-veto-ff-merge.md` in the terminal checkpoint with f1's Claim and Scope and a Basis to that Verification; `scripts/check-capsule` rejects a Knowledge Basis whose Claim or Scope differ.
- Evidence considered: f1's Verification; the terminal checkpoint's `check-capsule` run.
- Conclusion: `VERIFIED`, with its publication in the terminal checkpoint.
- Limitations: none beyond f1's.

<a id="verification-k2-commit"></a>
### Verification: `k2-commit`

- Claim: [`k2-commit`](#k2-commit).
- Method: as for k1, with f2 and `knowledge/ref-transaction-veto-commit.md`.
- Evidence considered: f2's Verification; the terminal checkpoint's `check-capsule` run.
- Conclusion: `VERIFIED`, with its publication in the terminal checkpoint.
- Limitations: none beyond f2's.

<a id="verification-k3-merge-commit"></a>
### Verification: `k3-merge-commit`

- Claim: [`k3-merge-commit`](#k3-merge-commit).
- Method: as for k1, with f3 and `knowledge/ref-transaction-veto-merge-commit.md`.
- Evidence considered: f3's Verification; the terminal checkpoint's `check-capsule` run.
- Conclusion: `VERIFIED`, with its publication in the terminal checkpoint.
- Limitations: none beyond f3's.

<a id="verification-k4-pull"></a>
### Verification: `k4-pull`

- Claim: [`k4-pull`](#k4-pull).
- Method: as for k1, with f4 and `knowledge/ref-transaction-veto-pull.md`.
- Evidence considered: f4's Verification; the terminal checkpoint's `check-capsule` run.
- Conclusion: `VERIFIED`, with its publication in the terminal checkpoint.
- Limitations: none beyond f4's.

<a id="verification-k5-rebase"></a>
### Verification: `k5-rebase`

- Claim: [`k5-rebase`](#k5-rebase).
- Method: as for k1, with f5 and `knowledge/ref-transaction-veto-rebase.md`.
- Evidence considered: f5's Verification; the terminal checkpoint's `check-capsule` run.
- Conclusion: `VERIFIED`, with its publication in the terminal checkpoint.
- Limitations: none beyond f5's.

### VERIFY gate

`ESTABLISHED`: the premise and f1–f5 are `VERIFIED` from the probe log; k1–k5 are `VERIFIED` with their publication in the terminal checkpoint, which `check-capsule` validates.

## Learn

### Technical

-   A `reference-transaction` veto is not atomic with the working tree: every operation probed except `git commit` had already rewritten the index and working tree when the ref update was refused, and the usual recoveries update the same ref, so the hook can refuse them too. `git rebase --abort` sends an all-zero old value, so a guard that compares old and new values refuses it even when the branch would not move.

### Process

-   Two Claims (f3, f5) were first drafted from a summary of the log and were wrong about what `merge --abort` and `rebase --abort` do under the stricter hook; rereading the raw log before the Verification caught both. Disposition: no permanent change; Evidence and verification already asks for Claims established from observations, which is what caught them.
-   addf friction around Knowledge as a Task's deliverable: `SELF-IMPROVEMENT/20261003T133924Z-knowledge-as-a-success-criterion.md`.

### LEARN gate

`ESTABLISHED`: the learnings above are recorded with their dispositions.

## Retention and promotion

### Promotion: `f1-ff-merge-veto`

-   Claim: [`f1-ff-merge-veto`](#f1-ff-merge-veto).
-   Will this Claim's validity outlive this Task and inform a future decision? `yes`: it is what any design that guards a branch with a `reference-transaction` hook must handle for a fast-forward.
-   Disposition: promoted to `knowledge/ref-transaction-veto-ff-merge.md`.

### Promotion: `f2-commit-veto`

-   Claim: [`f2-commit-veto`](#f2-commit-veto).
-   Will this Claim's validity outlive this Task and inform a future decision? `yes`, as for f1, for commits.
-   Disposition: promoted to `knowledge/ref-transaction-veto-commit.md`.

### Promotion: `f3-merge-commit-veto`

-   Claim: [`f3-merge-commit-veto`](#f3-merge-commit-veto).
-   Will this Claim's validity outlive this Task and inform a future decision? `yes`, as for f1, for merge commits.
-   Disposition: promoted to `knowledge/ref-transaction-veto-merge-commit.md`.

### Promotion: `f4-pull-veto`

-   Claim: [`f4-pull-veto`](#f4-pull-veto).
-   Will this Claim's validity outlive this Task and inform a future decision? `yes`, as for f1, for pulls.
-   Disposition: promoted to `knowledge/ref-transaction-veto-pull.md`.

### Promotion: `f5-rebase-veto`

-   Claim: [`f5-rebase-veto`](#f5-rebase-veto).
-   Will this Claim's validity outlive this Task and inform a future decision? `yes`, as for f1, for rebases.
-   Disposition: promoted to `knowledge/ref-transaction-veto-rebase.md`.

### Promotion: `p1-veto-leaves-staged`

-   Claim: [`p1-veto-leaves-staged`](#p1-veto-leaves-staged).
-   Will this Claim's validity outlive this Task and inform a future decision? `no`: f1 states it in full.
-   Disposition: not promoted — covered by f1.
