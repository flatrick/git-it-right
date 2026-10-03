# TASK — `fixup-unmatched agrees with git on the review's cases`

## Resume

**Contract version:** `2`

**State:** `COMPLETED`

**State path:** `DEFINE -> UNDERSTAND -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN -> COMPLETED`

**Resume at:** `NONE`

**Open obligations:** `NONE`

## Owned artifacts

-   `ledger.md` - the review findings that shaped this Task and its DEFINE dialogue.
-   `probe-git-rules.py` - Probe: the git behaviours r1–r7 depend on.
-   `logs/probe-git-rules-20261003-1722.log` - Evidence: first run, without the checked-out `HEAD` cases.
-   `logs/probe-git-rules-20261003-1726.log` - Evidence: the run of the current `probe-git-rules.py`.
-   `acceptance.py` - Probe and acceptance: follows each `try:` hint literally and checks subjects and tree; adds r1 and r7 cases to the archived Task's script.
-   `logs/acceptance-20261003-1815.log` - Evidence: the run on `e4c52d9`'s tree, 9/9.
-   `logs/test-red-20261003-1750.log` - Evidence: the new tests failing on the code before `e4c52d9`.
-   `logs/test-final-20261003-1825.log`, `logs/clippy-20261003-1825.log`, `logs/fmt-20261003-1825.log`, `logs/acceptance-final-20261003-1825.log` - Evidence: final checks on `e4c52d9`'s tree.

## Specification impact

- Current contract: `framework:spec/lint.md#req-lint-range-unsquashed`, `framework:spec/lint.md#req-lint-range-unmatched`, `framework:spec/hooks.md#req-hooks-pre-push-rejects`
- Proposed delta: `range-unsquashed` uses `--root` for a parentless base; `range-unmatched` defines subjects, the prefix chain, merges, `HEAD`/`@` and the before-the-range wording; `pre-push-rejects` keeps "already published". Text under Decide.
- Terminal publication: `framework:spec/lint.md#req-lint-range-unsquashed`, `framework:spec/lint.md#req-lint-range-unmatched`, `framework:spec/hooks.md#req-hooks-pre-push-rejects`

## Define

### Objective

`For every case the review of fix-unmatched-autosquash found, gir lint --range and pre-push predict what git rebase --autosquash does, give advice that can be followed, and stay fast on large fixups.`

DEFINE gate: `ESTABLISHED` — the operator agreed to the objective and success criteria r1–r7 (`ledger.md` Q10, A10).

### Success criteria

<a id="r1-root-in-range"></a>
#### `r1-root-in-range`

-   Claim: An unmatched fixup whose blamed target is the root commit inside the linted commits gets advice to move it below that commit, through `gir lint --range` and through pre-push on a repository's first push; when the oldest linted commit is the root, the hint says `git rebase -i --root`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the first push of every new repository gets wrong advice.
-   Basis: [Verification](#verification-r1-root-in-range).

<a id="r2-subject-rule"></a>
#### `r2-subject-rule`

-   Claim: gir reads every commit's subject as git does (the first paragraph joined with spaces) and strips the prefix chain with one or more spaces after each prefix (not tabs, which git does not skip), so its prediction matches git for wrapped fixup subjects, wrapped target titles and extra spaces. (Reworded in UNDERSTAND from "any whitespace"; `ledger.md` Q11.)
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: the loop the previous Task removed returns for these subjects.
-   Basis: [Verification](#verification-r2-subject-rule).

<a id="r3-merges-not-targets"></a>
#### `r3-merges-not-targets`

-   Claim: A merge commit in the linted commits is never a fold target, matching a plain `git rebase`, so a fixup aimed only at a merge is `fixup-unmatched`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: a fixup of a merge is told to run a rebase that leaves it.
-   Basis: [Verification](#verification-r3-merges-not-targets).

<a id="r4-blame-fast"></a>
#### `r4-blame-fast`

-   Claim: The blame step runs at most one `git blame` per hunk, and `gir lint --range` on an unmatched fixup that rewrites 3000 lines finishes in under one second.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch, a debug build.
-   Consequence if false: pre-push can take minutes on large fixups.
-   Basis: [Verification](#verification-r4-blame-fast).

<a id="r5-head-relative"></a>
#### `r5-head-relative`

-   Claim: A rev specifier that names `HEAD` or `@` resolves against the tip being linted (the pushed commit for pre-push, the newest commit for `lint --range`), not the checked-out `HEAD`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: pushing a branch other than the checked-out one misjudges such fixups.
-   Basis: [Verification](#verification-r5-head-relative).

<a id="r6-range-wording"></a>
#### `r6-range-wording`

-   Claim: `gir lint --range` describes a target outside the range as before the range, not as published; pre-push keeps "already published".
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: CI tells a user a local commit is published.
-   Basis: [Verification](#verification-r6-range-wording).

<a id="r7-amend-message"></a>
#### `r7-amend-message`

-   Claim: An `amend!` counts as carrying a new message only when it has a paragraph after its subject paragraph, so a wrapped `amend!` subject with no message is advised `fixup`.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: following the advice empties the target's message.
-   Basis: [Verification](#verification-r7-amend-message).

### Constraints

-   Same branch and worktree as the archived Task `unmatched-autosquash` (`fix-unmatched-autosquash`, `.worktrees/fix-unmatched-autosquash`); no new isolation (operator answer, `ledger.md` A9).
-   Every finding gets an end-to-end test that fails before its fix (`ledger.md` A9).
-   gir never rewrites existing commits (`framework:SPEC.md`, Boundaries).
-   Nothing under `.agents/` is committed.

### Material empirical premises

<a id="p1-review-findings"></a>
#### `p1-review-findings`

-   Claim: Findings 1–5 of the review reproduce on `8465c1d`; 6–8 are plausible.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, `8465c1d`.
-   Consequence if false: some criteria fix nothing.
-   Basis: [Verification](#verification-p1-review-findings).

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

## Implement

Commit `e4c52d9`, as decided; no deviation.

-   Tests first: `logs/test-red-20261003-1750.log` had all eight new or changed tests failing for the reviewed reasons: root target "already published" with `<base>` (lint and pre-push), a wrapped fixup subject predicted to fold, a merge predicted to fold, "already published" for `lint --range`, `fixup! HEAD~2` from another checkout unmatched, a wrapped `amend!` advised `fixup -C`, and the 3000-line fixup taking 7.82 s. That re-observes findings 1, 2, 5, 6, 7 and 8 on `8465c1d`'s code (premise `p1-review-findings`).
-   `src/cmd/lint.rs`: `subject`, `merges`, `head_at`; `folds` takes the merges and the tip; the base falls back to `--root`; the published wording depends on `pushing`; the `amend!` check looks for a paragraph after the subject.
-   `src/cmd/fixup.rs`: `blame_range` (one `git blame --root -L first,last` per hunk); `blame` is back to its code on `main`, and `blame_at` is gone.
-   `tests/lint.rs`: five new tests, a wrapped `amend!` case, the before-the-range wording, and a `lint_rev` helper; `tests/cli.rs`: a first-push test.
-   `acceptance.py`: copied from the archived Task with cases for r1 and r7; `logs/acceptance-20261003-1815.log` 9/9.

### IMPLEMENT gate

`ESTABLISHED`: the change is in `e4c52d9`; `cargo test --no-fail-fast`, clippy and `cargo fmt --check` passed on it, and acceptance 9/9.

## Verify

All on the tree committed as `e4c52d9`, Linux, git 2.56.0. `logs/test-final-20261003-1825.log`: 16 test binaries ok, none failed. `logs/acceptance-final-20261003-1825.log`: 9/9. `logs/clippy-20261003-1825.log` and `logs/fmt-20261003-1825.log`: clean. `logs/test-red-20261003-1750.log`: the same new tests failing on the code before the change.

<a id="verification-p1-review-findings"></a>
### Verification: `p1-review-findings`

- Claim: [`p1-review-findings`](#p1-review-findings).
- Method: the failing-first tests on the code before `e4c52d9`.
- Evidence considered: `logs/test-red-20261003-1750.log` shows findings 1 (root "already published", lint and pre-push), 2 (wrapped fixup predicted to fold), 5 (merge predicted to fold), 6 (7.82 s), 7 (`HEAD~2` from another checkout) and 8 (published wording; wrapped `amend!` advised `fixup -C`) failing. Finding 3 (wrapped target) and 4 (extra spaces) were confirmed by `logs/probe-git-rules-20261003-1726.log` against git; the review's "any whitespace" was corrected to spaces (`ledger.md` Q11).
- Conclusion: `VERIFIED`.
- Limitations: none that block completion.

<a id="verification-r1-root-in-range"></a>
### Verification: `r1-root-in-range`

- Claim: [`r1-root-in-range`](#r1-root-in-range).
- Method: `lint_range_names_a_root_target_and_rebases_with_root` (`tests/lint.rs`), `pre_push_names_a_root_target_on_a_first_push` (`tests/cli.rs`), and the acceptance case "r1".
- Evidence considered: both tests ok in the final log; acceptance follows `git rebase -i --root: move it below <root> chore: init, change pick to fixup` and ends with `chore: init`, `feat: two` and the tree unchanged.
- Conclusion: `VERIFIED`.
- Limitations: Linux and git 2.56.0 only.

<a id="verification-r2-subject-rule"></a>
### Verification: `r2-subject-rule`

- Claim: [`r2-subject-rule`](#r2-subject-rule).
- Method: `lint_range_reads_wrapped_subjects_and_extra_spaces_as_git_does`, whose four cases mirror the probe's observed git outcomes: wrapped fixup left, wrapped target folds, two spaces fold, a spaced chain folds.
- Evidence considered: the test ok in the final log; failing before (`logs/test-red-20261003-1750.log`); git's outcomes in `logs/probe-git-rules-20261003-1726.log`. A tab after a prefix is not an autosquash commit to gir's `cc` parser either, matching git.
- Conclusion: `VERIFIED`.
- Limitations: Linux and git 2.56.0 only.

<a id="verification-r3-merges-not-targets"></a>
### Verification: `r3-merges-not-targets`

- Claim: [`r3-merges-not-targets`](#r3-merges-not-targets).
- Method: `lint_range_never_folds_into_a_merge_commit`.
- Evidence considered: ok in the final log, failing before; git leaves such a fixup (`logs/probe-git-rules-20261003-1726.log`).
- Conclusion: `VERIFIED`.
- Limitations: `git rebase --rebase-merges` keeps merges, and gir assumes a plain rebase, as its hints run.

<a id="verification-r4-blame-fast"></a>
### Verification: `r4-blame-fast`

- Claim: [`r4-blame-fast`](#r4-blame-fast).
- Method: `lint_range_blames_a_large_unmatched_fixup_quickly` asserts the 3000-line case names its target in under one second; `fixup::blame_range` calls `git blame` once per hunk.
- Evidence considered: ok in the final log (debug build); 7.82 s before (`logs/test-red-20261003-1750.log`).
- Conclusion: `VERIFIED`.
- Limitations: the bound is asserted, not the exact time; history depth was small.

<a id="verification-r5-head-relative"></a>
### Verification: `r5-head-relative`

- Claim: [`r5-head-relative`](#r5-head-relative).
- Method: `lint_range_resolves_head_against_the_newest_commit_linted` lints `base..topic` from `main` with `fixup! HEAD~2` and `fixup! @~3`.
- Evidence considered: ok in the final log, both `fixup-unsquashed`; failing before; git folds the same specifiers with the branch checked out (`logs/probe-git-rules-20261003-1726.log`).
- Conclusion: `VERIFIED`.
- Limitations: checkout-history forms such as `@{-1}` are out of scope (`ledger.md` A10).

<a id="verification-r6-range-wording"></a>
### Verification: `r6-range-wording`

- Claim: [`r6-range-wording`](#r6-range-wording).
- Method: `lint_range_unmatched_fixup_of_a_target_before_the_range_says_to_reword_it` and `pre_push_rejects_a_fixup_of_a_pushed_commit_and_names_no_verify`.
- Evidence considered: both ok in the final log: `is before the range` for `lint --range`, `is already published` with `git push --no-verify` for pre-push.
- Conclusion: `VERIFIED`.
- Limitations: none.

<a id="verification-r7-amend-message"></a>
### Verification: `r7-amend-message`

- Claim: [`r7-amend-message`](#r7-amend-message).
- Method: the wrapped case in `lint_range_unmatched_fixup_names_the_target_blame_finds`, and the acceptance case "r7".
- Evidence considered: ok in the final log, failing before; acceptance follows `change pick to fixup` and keeps `feat: add a`.
- Conclusion: `VERIFIED`.
- Limitations: Linux and git 2.56.0 only.

### VERIFY gate

`ESTABLISHED`: r1–r7 and the premise are `VERIFIED` above; clippy and format clean on the same tree.

## Learn

### Technical

-   git's autosquash subject is the joined first paragraph; prefixes are followed by spaces, not any whitespace; merges are not targets of a plain rebase; `HEAD` means what is checked out when the rebase starts; `git blame` needs `--root` for full root IDs. All are now encoded in `src/cmd/lint.rs` and `tests/lint.rs`.

### Process

-   The previous Task's own probes and acceptance passed while a cold-context review then found eight defects: its cases were drawn from the shapes the author already had in mind (linear history, single-line subjects, no root in range, small fixups). A blind reviewer with permission to run experiments found the shapes the author did not. Disposition: no permanent change; the operator asked for this review, and it is recorded here as evidence that such a review paid off.
-   A reviewer's claim about git ("any whitespace") was itself wrong in part; probing it before encoding it avoided a new mismatch. Disposition: no permanent change; it is what Evidence and verification already asks.

### LEARN gate

`ESTABLISHED`: the learnings above are recorded with their dispositions.

## Retention and promotion

No Claim is promoted: each describes git behaviour that `tests/lint.rs` now checks against a real repository, so a change in git breaks a test rather than leaving a stale Knowledge file. No Learning is promoted.

## Archive readiness

The bundle is self-contained: `ledger.md` carries the review's findings and the operator's answers, and the probe, acceptance script and every cited log are inside it. The review's own scripts stayed in the session scratchpad and are not needed: each finding is re-observed by `logs/test-red-20261003-1750.log` or `logs/probe-git-rules-20261003-1726.log`. Specification references use `framework:`; commit IDs and source paths are supplemental anchors; the archived Task `unmatched-autosquash` is context, not required to read this one.

## Terminal record

### Summary

All eight review findings on `fix-unmatched-autosquash` are fixed in `e4c52d9`: root targets, wrapped subjects, spaces after prefixes, merges, `HEAD`/`@` specifiers, before-the-range wording, wrapped `amend!` subjects, and one blame per hunk. Every criterion has a test that failed before and passes now.

### Gate basis

r1–r7 and `p1-review-findings` are `VERIFIED` (Verify). No deferred verification remains. The specification delta is published in this checkpoint.
