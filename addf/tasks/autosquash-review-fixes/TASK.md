# TASK — `fixup-unmatched agrees with git on the review's cases`

## Resume

**Contract version:** `2`

**State:** `DECIDE`

**State path:** `DEFINE -> UNDERSTAND -> DECIDE`

**Resume at:** Record the approach for r1–r7, its spec delta and verification strategy under Decide.

**Open obligations:** Approach, spec delta and verification strategy recorded — blocks the DECIDE gate.

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

## Investigate

INVESTIGATE gate: `NOT_APPLICABLE` — skipped. Understand left no assumption or open question; the review and `probe-git-rules.py` already settled every uncertainty r1–r7 depend on.

## Decide

### Approach

All in `src/cmd/lint.rs` unless named.

-   r2, r7: one `subject(body)` that joins the first paragraph's lines, each trimmed at the end, with one space; `folds` uses it for the fixup and for every older commit, and strips each prefix only when one or more spaces follow it, then those spaces. The `amend!` check asks whether a non-empty paragraph follows the subject paragraph.
-   r3: `lint_recorded` asks git which listed commits are merges, with one `git rev-list --merges --no-walk --stdin` fed their IDs, and only when the list holds an autosquash commit; merges are left out of the fold targets.
-   r5: a specifier that is `HEAD` or `@`, or starts with `HEAD~`, `HEAD^`, `@~` or `@^`, has that leading name replaced by the newest listed commit (the tip being linted or pushed) before `git rev-parse`.
-   r1, r4: a new `fixup::blame_range(rev, path, lines)` runs one `git blame --root -l -s -L <first>,<last>` per hunk and reads the ID on every line; `lint::blamed` uses it. `fixup::blame` goes back to its code on `main`, and the `blame_at` added by the previous Task goes away, so `gir fixup` is byte-for-byte unchanged against `main`.
-   r1: when the oldest listed commit has no parent, the base is `--root`, so hints read `git rebase --autosquash --root` and `git rebase -i --root`.
-   r6: the published-target message says `is already published` when pushing and `is before the range` for `gir lint --range`.

### Proposed specification delta

-   `lint.md`, `range-unsquashed`: `<base>` becomes the 10-character ID of the first parent of the oldest commit linted, "or `--root` when it has none".
-   `lint.md`, `range-unmatched`: define the subject as git's (first paragraph joined with spaces), the prefix chain as each prefix followed by one or more spaces, exclude merge commits from the targets, resolve a specifier starting with `HEAD` or `@` against the newest commit linted, and word a target outside the range as "before the range".
-   `hooks.md`, `pre-push-rejects`: a target outside the pushed commits is worded "already published".

### Rejected alternatives

-   Fetching parents for every commit through `lint::commits`' format: changes a function both callers share, for data needed only when an autosquash commit exists.
-   Adding `--root` and ranged blame to `fixup::blame` as well: it would change `gir fixup`, which no criterion covers.

### Verification strategy

-   For each of r1–r7, an end-to-end test in `tests/lint.rs` or `tests/cli.rs` written first and seen failing on the current code, then passing.
-   r4: a test that lints an unmatched fixup rewriting 3000 lines and asserts it finishes in under one second, plus a timing log.
-   `acceptance.py` from the archived Task, run again, with cases for a root target and a wrapped `amend!`.
-   `cargo test --no-fail-fast`, clippy, `cargo fmt --check`.

### DECIDE gate

`ESTABLISHED`: each criterion maps to a change grounded in the Understand observations, and each has a failing-first test.
