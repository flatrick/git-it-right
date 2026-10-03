# TASK — `fixup-unmatched agrees with git on the review's cases`

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Probe git 2.56.0 for the subject rule, prefix whitespace, merges in the todo, `HEAD` during `git rebase <upstream> <branch>`, and `git blame` on root lines and line ranges.

**Open obligations:** The git behaviours r1–r7 depend on are observed — blocks the UNDERSTAND gate.

## Owned artifacts

-   `ledger.md` - the review findings that shaped this Task and its DEFINE dialogue.
-   `probe-git-rules.py` - Probe: the git behaviours r1–r7 depend on.
-   `logs/probe-git-rules-20261003-1722.log` - Evidence: first run, without the checked-out `HEAD` cases.
-   `logs/probe-git-rules-20261003-1726.log` - Evidence: the run of the current `probe-git-rules.py`.

## Specification impact

- Current contract: `framework:spec/lint.md#req-lint-range-unsquashed`, `framework:spec/lint.md#req-lint-range-unmatched`, `framework:spec/hooks.md#req-hooks-pre-push-rejects`
- Proposed delta: `PENDING` — settled in DECIDE.
- Terminal publication: `PENDING`

## Define

### Objective

`For every case the review of fix-unmatched-autosquash found, gir lint --range and pre-push predict what git rebase --autosquash does, give advice that can be followed, and stay fast on large fixups.`

DEFINE gate: `ESTABLISHED` — the operator agreed to the objective and success criteria r1–r7 (`ledger.md` Q10, A10).

### Success criteria

<a id="r1-root-in-range"></a>
#### `r1-root-in-range`

-   Claim: An unmatched fixup whose blamed target is the root commit inside the linted commits gets advice to move it below that commit, through `gir lint --range` and through pre-push on a repository's first push; when the oldest linted commit is the root, the hint says `git rebase -i --root`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the first push of every new repository gets wrong advice.
-   Basis: none yet; pending.

<a id="r2-subject-rule"></a>
#### `r2-subject-rule`

-   Claim: gir reads every commit's subject as git does (the first paragraph joined with spaces) and strips the prefix chain with one or more spaces after each prefix (not tabs, which git does not skip), so its prediction matches git for wrapped fixup subjects, wrapped target titles and extra spaces. (Reworded in UNDERSTAND from "any whitespace"; `ledger.md` Q11.)
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the loop the previous Task removed returns for these subjects.
-   Basis: none yet; pending.

<a id="r3-merges-not-targets"></a>
#### `r3-merges-not-targets`

-   Claim: A merge commit in the linted commits is never a fold target, matching a plain `git rebase`, so a fixup aimed only at a merge is `fixup-unmatched`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: a fixup of a merge is told to run a rebase that leaves it.
-   Basis: none yet; pending.

<a id="r4-blame-fast"></a>
#### `r4-blame-fast`

-   Claim: The blame step runs at most one `git blame` per hunk, and `gir lint --range` on an unmatched fixup that rewrites 3000 lines finishes in under one second.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch, a debug build.
-   Consequence if false: pre-push can take minutes on large fixups.
-   Basis: none yet; pending.

<a id="r5-head-relative"></a>
#### `r5-head-relative`

-   Claim: A rev specifier that names `HEAD` or `@` resolves against the tip being linted (the pushed commit for pre-push, the newest commit for `lint --range`), not the checked-out `HEAD`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: pushing a branch other than the checked-out one misjudges such fixups.
-   Basis: none yet; pending.

<a id="r6-range-wording"></a>
#### `r6-range-wording`

-   Claim: `gir lint --range` describes a target outside the range as before the range, not as published; pre-push keeps "already published".
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: CI tells a user a local commit is published.
-   Basis: none yet; pending.

<a id="r7-amend-message"></a>
#### `r7-amend-message`

-   Claim: An `amend!` counts as carrying a new message only when it has a paragraph after its subject paragraph, so a wrapped `amend!` subject with no message is advised `fixup`.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: following the advice empties the target's message.
-   Basis: none yet; pending.

### Constraints

-   Same branch and worktree as the archived Task `unmatched-autosquash` (`fix-unmatched-autosquash`, `.worktrees/fix-unmatched-autosquash`); no new isolation (operator answer, `ledger.md` A9).
-   Every finding gets an end-to-end test that fails before its fix (`ledger.md` A9).
-   gir never rewrites existing commits (`framework:SPEC.md`, Boundaries).
-   Nothing under `.agents/` is committed.

### Material empirical premises

<a id="p1-review-findings"></a>
#### `p1-review-findings`

-   Claim: Findings 1–5 of the review reproduce on `8465c1d`; 6–8 are plausible.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, `8465c1d`.
-   Consequence if false: some criteria fix nothing.
-   Basis: finding 1 reproduced in the parent session (`ledger.md` A1); the rest are the reviewer's runs, whose logs were not kept. Each is re-checked by the failing test written for it in IMPLEMENT.

## Understand

### Relevant context

`probe-git-rules.py`, git 2.56.0, Linux (`logs/probe-git-rules-20261003-1726.log`; `-1722.log` is the same run without the checked-out `HEAD` cases):

-   Subject: git joins the lines of the first paragraph with one space after trimming each line's trailing whitespace (`feat: one`, `  two  `, `three` gives `feat: one   two three`).
-   A wrapped fixup subject is matched on the joined subject (left when only the first line would match); a wrapped target title is matched on its joined subject (folds).
-   After a prefix git skips one or more spaces, inside a chain too (`fixup!  squash!  feat: one` folds); a tab is not skipped (`fixup!\tfeat: one` is left) — the reviewer's "any whitespace" is narrower in fact.
-   A wrapped `amend!` subject with no message paragraph folds, and git's own autosquash then leaves the target with an empty message.
-   A plain `git rebase` drops a merge commit from its todo; a fixup naming the merge is left on the linearised branch.
-   `HEAD`-relative specifiers resolve against what is checked out when the rebase starts: with the branch checked out, `fixup! HEAD~2` and `fixup! @~2` fold relative to its tip; `git rebase <base> feature` run from another branch leaves `fixup! HEAD~2`. gir's hints are `git rebase ... <base>` run on the branch being fixed, so the tip being linted is the right reference (r5 as agreed).
-   `git blame` prints a root-commit line as `^` plus 39 characters; `git blame --root` prints the full 40-character ID.
-   3000 single-line `git blame` calls took 6.04 s; one `-L 1,3000` call took 0.003 s and printed 3000 lines.
-   In gir: `lint::title` reads the first line; `folds` strips `fixup! ` with one space; `older` includes merges; `blamed` runs `fixup::blame_at` per line without `--root`; the base hint is `<base>` when the oldest commit has no parent; the published message does not depend on `pushing`; the `amend!` check counts any non-empty line after the first.

### Assumptions

-   None material beyond the probe's scope (Linux, git 2.56.0).

### Open questions

-   None: every r1–r7 mechanism is observed above.

### UNDERSTAND gate

`ESTABLISHED`: every git behaviour r1–r7 rely on is observed, one review claim is corrected (tab), and the gir code paths are known.
