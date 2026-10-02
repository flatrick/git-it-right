# TASK — `gir amend, reword and squash, with a terminal picker and --split`

## Resume

**Contract version:** `2`

**State:** `COMPLETED`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN -> COMPLETED`

**Resume at:** `NONE`

**Open obligations:** `NONE`

## Owned artifacts

-   `ledger.md` - pre-Task and DEFINE questions with the operator's answers.
-   `logs/probe-premises-20260927.log` - Probe: empty `--fixup=amend:` and `--fixup=reword:` with staged changes (git 2.55.0, Linux).
-   `logs/probe-split-20260927.log` - Probe: the split loop on two targets in one file, including an insertion.
-   `logs/test-red-20260927.log` - Evidence: the new integration tests against unchanged `src/` (19 of 20 fail).
-   `logs/split-order-loop-20260927.log` - Evidence: the split tests repeated 8 times.
-   `logs/tty-manual-20260927.log` - Evidence: the built gir under a real pseudo-terminal (`script`), and with a pipe.
-   `logs/refusal-diff-14d3bd5-20260927.log` - Evidence: every `gir fixup` refusal from `main` (`14d3bd5`) and this branch, stdin a pipe, diffed.
-   `logs/clippy-20260927.log` - Evidence: `cargo clippy --all-targets -- -D warnings` on the final code.
-   `logs/test-final-20260927.log` - Evidence: `cargo test` on the final revision (stdout).

## Specification impact

- Current contract: `framework:spec/fixup.md`, `framework:spec/cli.md#req-cli-usage-lists-commands`, `framework:spec/cli.md#req-cli-unknown-long-option`, `framework:spec/cli.md#req-cli-repository-root`, `framework:spec/explain.md#req-explain-known-pages`
- Proposed delta: changes to the `fixup` and `cli` modules.
  - fixup: the module covers four subcommands. `gir amend`, `gir reword` and `gir squash` create `amend!`, `amend!` (message only) and `squash!` commits through `git commit --fixup=amend:`, `--fixup=reword:` and `--squash=`. Existing requirements apply to each subcommand, with `gir <subcommand>` and the subcommand's prefix in their messages.
  - fixup: `gir reword` does not require staged changes and ignores them. Without a commit argument it has no lines to trace.
  - fixup: add *interactive* — stdin and stderr are terminals, or `GIR_INTERACTIVE=1`; `GIR_INTERACTIVE=0` disables it. When interactive and a target cannot be selected automatically, gir asks with a numbered prompt on stderr instead of refusing; `--dry-run` never asks.
  - fixup: the prompt for several targets also offers splitting; an empty answer, `q` or end of input cancels with exit `1` and no commit.
  - fixup: add `--split` for fixup, amend and squash — one autosquash commit per target, each holding only that target's hunks, working tree untouched; refused before any commit when a hunk spans several commits or a file cannot be traced; a failure part-way restores `HEAD` and the index.
  - fixup: the non-interactive several-targets refusal adds `or: gir <subcommand> --split`.
  - cli: usage lists `amend`, `reword` and `squash`; accepted flags gain `amend --dry-run --split`, `reword --dry-run`, `squash --dry-run --split`, `fixup --dry-run --split`; repository-root covers the new subcommands.
- Terminal publication: `framework:SPEC.md#purpose`, `framework:spec/fixup.md#req-fixup-subcommands`, `framework:spec/fixup.md#req-fixup-subcommand-scope`, `framework:spec/fixup.md#req-fixup-arguments`, `framework:spec/fixup.md#req-fixup-staged-required`, `framework:spec/fixup.md#req-fixup-amend-staged-required`, `framework:spec/fixup.md#req-fixup-reword-ignores-index`, `framework:spec/fixup.md#req-fixup-reword-target`, `framework:spec/fixup.md#req-fixup-staged-line-target`, `framework:spec/fixup.md#req-fixup-split-flag-hint`, `framework:spec/fixup.md#req-fixup-absorb-hint`, `framework:spec/fixup.md#req-fixup-interactive`, `framework:spec/fixup.md#req-fixup-ask-several`, `framework:spec/fixup.md#req-fixup-ask-branch-commit`, `framework:spec/fixup.md#req-fixup-ask-answers`, `framework:spec/fixup.md#req-fixup-ask-cancel`, `framework:spec/fixup.md#req-fixup-split`, `framework:spec/fixup.md#req-fixup-split-refusals`, `framework:spec/fixup.md#req-fixup-split-with-target`, `framework:spec/fixup.md#req-fixup-split-rollback`, `framework:spec/fixup.md#req-fixup-split-output`, `framework:spec/fixup.md#req-fixup-commit-failure`, `framework:spec/cli.md#req-cli-usage-lists-commands`, `framework:spec/cli.md#req-cli-unknown-long-option`, `framework:spec/cli.md#req-cli-runtime-error`, `framework:spec/cli.md#req-cli-repository-root`

## Define

### Objective

A developer can create any of git's autosquash commits (`fixup!`, `amend!` with content, `amend!` message-only, `squash!`) with gir's target selection and base checks.
When gir cannot pick one target and a person is at a terminal, gir asks instead of refusing.
Staged changes that belong to several commits can become one autosquash commit per target.
Callers without a terminal (GUI clients, CI, agents) keep today's refusals.

### Success criteria

<a id="modes-create-commits"></a>
#### `modes-create-commits`

-   Claim: `gir amend`, `gir reword` and `gir squash` with an explicit or traced target create a commit whose subject is `amend! <subject>`, `amend! <subject>` and `squash! <subject>` respectively; the `gir reword` commit's tree equals its parent's even with changes staged.
-   State: `VERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: the new subcommands create wrong or no commits.
-   Basis: [Verification](#verification-modes-create-commits).

<a id="reword-needs-target"></a>
#### `reword-needs-target`

-   Claim: Without a terminal and without a commit argument, `gir reword` exits `2` naming `gir reword <commit>` and creates no commit.
-   State: `VERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: reword guesses a target silently.
-   Basis: [Verification](#verification-reword-needs-target).

<a id="non-interactive-unchanged"></a>
#### `non-interactive-unchanged`

-   Claim: Without a terminal, every existing `gir fixup` refusal prints the same stderr as before this Task, except the several-targets refusal, which ends with the added `or: gir fixup --split` line.
-   State: `VERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: GUI clients and scripts see changed behaviour or hang on a prompt.
-   Basis: [Verification](#verification-non-interactive-unchanged).

<a id="split-per-target"></a>
#### `split-per-target`

-   Claim: `gir fixup --split` with hunks traced to N commits creates N `fixup!` commits, each changing only its target's hunks; afterwards `HEAD`'s tree equals the tree staged before the command and the working tree is unchanged. A hunk spanning several commits is refused before any commit, leaving `HEAD` and the index unchanged.
-   State: `VERIFIED`
-   Scope: Linux, this branch.
-   Consequence if false: a split loses, duplicates or misattributes changes.
-   Basis: [Verification](#verification-split-per-target).

<a id="picker-choices"></a>
#### `picker-choices`

-   Claim: With `GIR_INTERACTIVE=1` and several targets, answering `2` creates one commit for the second listed target from all staged changes, `s` splits as in `split-per-target`, and an empty answer exits `1` with no commit.
-   State: `VERIFIED`
-   Scope: Linux, this branch, piped stdin.
-   Consequence if false: the prompt commits to the wrong target.
-   Basis: [Verification](#verification-picker-choices).

<a id="gates-green"></a>
#### `gates-green`

-   Claim: `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision.
-   State: `VERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: regressions ship.
-   Basis: [Verification](#verification-gates-green).

### Constraints

-   Work in worktree `.worktrees/feat-fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   No new crate dependency for the picker.
-   `addf/spec/*` changes only by publication at completion.

### Material empirical premises

<a id="amend-needs-staged"></a>
#### `amend-needs-staged`

-   Claim: `git commit --fixup=amend:<commit>` with nothing staged fails, while `--fixup=reword:<commit>` succeeds and ignores staged changes.
-   State: `REFUTED`
-   Scope: git 2.55.0, Linux.
-   Consequence if false: the staged-required rule for `gir amend` is wrong.
-   Basis: [Verification](#verification-amend-needs-staged).

<a id="unidiff-zero-apply"></a>
#### `unidiff-zero-apply`

-   Claim: After one target's hunks are committed, `git diff -U0 HEAD <staged-tree>` yields hunks for the remaining targets that `git apply --cached --unidiff-zero` applies to an index reset to `HEAD`, and blame at the new `HEAD` still attributes their lines to the original commits.
-   State: `VERIFIED`
-   Scope: git 2.55.0, Linux.
-   Consequence if false: the split algorithm needs another design.
-   Basis: [Verification](#verification-unidiff-zero-apply).

<a id="gui-clients-have-no-tty"></a>
#### `gui-clients-have-no-tty`

-   Claim: GUI git clients' custom actions (SourceTree, SourceGit, Visual Studio) run gir with stdin or stderr not a terminal, so no prompt appears.
-   State: `UNVERIFIED`
-   Scope: those clients on Windows and macOS.
-   Consequence if false: such a client could wait on a prompt; end of input or `GIR_INTERACTIVE=0` still ends it.
-   Basis: none; the clients are not available here.

### DEFINE gate

`ESTABLISHED`: on 2026-09-27 the operator approved the plan and explicitly agreed that the objective, scope and success criteria are right (`ledger.md`).

## Understand

### Relevant context

-   `src/cmd/fixup.rs` holds target selection (`auto_target`, `parse_hunks`, `blame`, `find_base`); `src/main.rs` dispatches subcommands and gates flags.
-   `src/cc/structure.rs` and `addf/spec/hooks.md#req-hooks-commit-msg-autosquash` already accept `amend!` and `squash!`; hooks need no change.
-   `git::passthrough` inherits stdio, so git's editor works for amend, reword and squash.

### Assumptions

-   NONE beyond the premises above.

### Open questions

-   NONE.

### UNDERSTAND gate

`ESTABLISHED`: the change is confined to `src/cmd/fixup.rs`, `src/main.rs`, a new picker module, docs and tests; the two unknowns that could change the design are the premises `amend-needs-staged` and `unidiff-zero-apply`.

### Deferred verification

-   `gui-clients-have-no-tty`: the clients are not installed here; earliest checkpoint `VERIFY`; settling observation: run a custom action calling `gir fixup` in one client; consequence if false as stated; blocks nothing, because end of input cancels.

## Investigate

-   `amend-needs-staged`: refuted. git creates an empty `amend! feat: a` commit (exit `0`). A git rule does not force gir's staged requirement for `gir amend`; it is now a design choice (see Decide).
-   `unidiff-zero-apply`: verified. Disposition: resolved.
-   `gui-clients-have-no-tty`: `DEFERRED_VERIFICATION` (see Understand).

INVESTIGATE gate: `ESTABLISHED`; each decision-relevant uncertainty above has a disposition.

## Decide

-   Approach: the approved plan (`/plan` output, restated in Specification impact): a `Mode` enum in `src/cmd/fixup.rs` shared by four subcommands; a dependency-free picker in `src/pick.rs` gated on `IsTerminal` and `GIR_INTERACTIVE`; `--split` implemented as the probed loop (diff `HEAD` against the staged tree, attribute by blame, `read-tree HEAD`, `apply --cached --unidiff-zero`, commit, repeat), with rollback to the original `HEAD` and staged tree on failure.
-   `gir amend` keeps requiring staged changes although git would accept none: an empty `amend!` and a `reword` produce the same result, so the message-only path stays `gir reword` and the refusal names it. Automatic target selection needs staged lines anyway.
-   Rejected: flags on `gir fixup` (operator chose subcommands); an arrow-key crate (operator chose no dependency); `--fixup=squash:` (not a git option).
-   Residual uncertainty: `gui-clients-have-no-tty`; end of input cancels the prompt, so a client that does attach a terminal cannot hang indefinitely without also holding stdin open.
-   Verification strategy: integration tests in `tests/fixup.rs` per success criterion with `GIT_EDITOR=true` and `GIR_INTERACTIVE`; unit tests for the prompt; `cargo test` and clippy.

DECIDE gate: `ESTABLISHED`.

## Implement

-   `src/cmd/fixup.rs`: `Mode` (name, prefix, commit option), `run` replacing `fixup`, `Trace`/`Traced` returned by tracing instead of formatted errors, the picker hooks, `several_targets` with the `--split` hint.
-   `src/cmd/fixup/split.rs`: the split, rollback and patch builder. `src/pick.rs`: `interactive` and `choose`. `src/main.rs`: the three subcommands, `--split`, usage.
-   `tests/fixup_modes.rs` (22 tests), `tests/common/mod.rs` (`gir_with`; `GIR_INTERACTIVE` cleared), `CHEATSHEET.md` (the `gir explain fixup` page) and `README.md`.
-   Deviation from Decide: the split does not re-trace after each commit. Re-blaming an insertion next to a line an earlier split commit changed would name that new commit. Each round instead resets the index to the original `HEAD` and applies the hunks of this and all earlier targets from the one original trace.
-   Checkpoint: a deterministic test (`split_handles_alternating_targets_in_one_file`, insertions alternating between targets) failed on the deviation: `git apply --unidiff-zero` places a zero-context hunk by its new-side start, which assumes every earlier hunk was applied, so skipped hunks misplaced later ones. The final tree still matched, so only per-commit assertions caught it. Fix: the patch builder recomputes each new-side start from the hunks it includes (`renumbers_the_new_side_from_the_hunks_included`). The premise probe `unidiff-zero-apply` covered the re-trace design only; it is not evidence for the shipped design, which the tests cover.
-   Checkpoint: under a real terminal the reword prompt read "pick the commit it belongs to"; changed to "pick the commit to reword:".

IMPLEMENT gate: `ESTABLISHED`.

## Verify

<a id="verification-amend-needs-staged"></a>
### Verification: `amend-needs-staged`

- Claim: [amend-needs-staged](#amend-needs-staged)
- Method: in a fresh repository with nothing staged, ran `git commit --fixup=amend:<A>`; then staged a change and ran `git commit --fixup=reword:<A>`.
- Evidence considered: `logs/probe-premises-20260927.log`: the amend exited `0` creating `amend! feat: a`; the reword exited `0`, its tree equals its parent's, and `a` stayed staged.
- Conclusion: `REFUTED`; the amend half is false, the reword half holds.
- Limitations: git 2.55.0 on Linux only; not blocking, since gir's rule is now its own choice.

<a id="verification-unidiff-zero-apply"></a>
### Verification: `unidiff-zero-apply`

- Claim: [unidiff-zero-apply](#unidiff-zero-apply)
- Method: one file with lines from commits A and B; staged a change to an A line, an insertion between A lines, and a change to a B line; ran the loop by hand for two rounds.
- Evidence considered: `logs/probe-split-20260927.log`: round 2's diff was recomputed with shifted line numbers (`@@ -8 +8 @@`); blame at the new `HEAD` still named B; both applies and commits exited `0`; the final tree equals the staged tree; `git status --short` was empty. `logs/probe-premises-20260927.log` has an earlier two-file run whose second target variable was captured wrongly; its mechanics matched but it is not the basis.
- Conclusion: `VERIFIED`.
- Limitations: git 2.55.0 on Linux; renames and binary files are outside the split, which refuses untraceable files. It verifies the re-trace design; the shipped design differs (see Implement).

<a id="verification-modes-create-commits"></a>
### Verification: `modes-create-commits`

- Claim: [modes-create-commits](#modes-create-commits)
- Method: integration tests with `GIT_EDITOR=true`; manual runs under a pseudo-terminal.
- Evidence considered: `logs/test-final-20260927.log`: `amend_creates_an_amend_commit_with_the_staged_changes`, `squash_creates_a_squash_commit_for_the_traced_target` (traced targets), `reword_changes_no_content_and_leaves_the_index_alone` (explicit target, tree equals parent, `a.txt` still staged), `reword_needs_nothing_staged`, `amend_and_squash_accept_an_explicit_target`, `explicit_target_checks_apply_to_every_subcommand`, `dry_run_prints_the_target_for_every_subcommand` pass. `logs/tty-manual-20260927.log` sections 5 and 5b show `amend! feat: add b` / `amend! feat: add a` subjects. `logs/test-red-20260927.log` shows these tests failing before the change.
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.55.0; the editor is replaced by `true`, so message editing itself is git's behavior, not gir's.

<a id="verification-reword-needs-target"></a>
### Verification: `reword-needs-target`

- Claim: [reword-needs-target](#reword-needs-target)
- Method: `gir reword` with stdin a pipe and no `GIR_INTERACTIVE`.
- Evidence considered: `reword_without_a_target_needs_one_when_not_interactive` passes in `logs/test-final-20260927.log`: exit `2`, exact stderr, `HEAD` unchanged, although `1` was on stdin.
- Conclusion: `VERIFIED`.
- Limitations: none material.

<a id="verification-non-interactive-unchanged"></a>
### Verification: `non-interactive-unchanged`

- Claim: [non-interactive-unchanged](#non-interactive-unchanged)
- Method: built `main` (`14d3bd5`) from `git archive`; ran it and this branch's gir through 11 refusal scenarios with stdin a pipe, diffing stderr and exit codes; the existing `tests/fixup.rs` ran unmodified.
- Evidence considered: `logs/refusal-diff-14d3bd5-20260927.log`: 10 scenarios identical (nothing staged, with and without a commit; not a commit; not on the branch; on base; new file; new binary; binary change; unattributed line; line on the base branch), all exit `2` with `HEAD` unchanged; several-targets differs by exactly `+  or: gir fixup --split creates one fixup! per commit`. Two earlier runs of this script were wrong (a literal `$O` argument, then clobbered setup) and were overwritten; the committed log is the corrected run, and each scenario's message shows it reached its intended refusal. `logs/tty-manual-20260927.log` sections 3 and 4 show no prompt with a pipe, or with `GIR_INTERACTIVE=0` under a terminal.
- Conclusion: `VERIFIED`.
- Limitations: `git-absorb` was not installed, so the absorb line was not compared; `multiple_targets_suggest_installed_git_absorb` still passes.

<a id="verification-split-per-target"></a>
### Verification: `split-per-target`

- Claim: [split-per-target](#split-per-target)
- Method: integration tests; one test alternates insertions B, A, B in one file so every order applies a hunk after a skipped, line-shifting one; per-commit `--numstat` assertions; manual split plus `git rebase -i --autosquash`.
- Evidence considered: `logs/test-final-20260927.log`: `split_creates_one_commit_per_target`, `split_handles_alternating_targets_in_one_file`, `split_refuses_a_hunk_spanning_several_commits_before_committing` (`HEAD` and `write-tree` unchanged), `split_restores_head_and_index_when_a_commit_fails`, `split_is_refused_with_a_commit_argument_and_for_reword`, and unit test `renumbers_the_new_side_from_the_hunks_included` pass. `logs/split-order-loop-20260927.log`: 8 repeated runs, all pass. `logs/tty-manual-20260927.log` section 6: after the split and autosquash, `main..` is `feat: add b`, `feat: add a` with both changes folded in. Contradicting evidence, since resolved: the first alternating-test run failed (Implement).
- Conclusion: `VERIFIED`.
- Limitations: Linux, git 2.55.0; Windows not run for this Task.

<a id="verification-picker-choices"></a>
### Verification: `picker-choices`

- Claim: [picker-choices](#picker-choices)
- Method: integration tests with `GIR_INTERACTIVE=1` and piped answers; the same choices under a real pseudo-terminal without `GIR_INTERACTIVE`.
- Evidence considered: `logs/test-final-20260927.log`: `picker_number_puts_all_staged_changes_in_that_commit` (`9` asks again, `2` commits all staged changes to the second target), `picker_split_creates_one_commit_per_target`, `picker_cancel_creates_no_commit` (empty, `q`, end of input: exit `1`, `HEAD` unchanged), `picker_offers_branch_commits_for_a_new_file`, `picker_offers_branch_commits_for_reword`, `picker_can_be_disabled_and_never_runs_for_dry_run`, and the `pick` unit tests pass. `logs/tty-manual-20260927.log` sections 1 and 2 show the prompt, then `s` and `1` acting as specified with terminal detection alone.
- Conclusion: `VERIFIED`.
- Limitations: Linux terminal only; Windows console detection (`IsTerminal`) not observed.

<a id="verification-gates-green"></a>
### Verification: `gates-green`

- Claim: [gates-green](#gates-green)
- Method: `cargo clippy --all-targets -- -D warnings` and `cargo test` on the final code.
- Evidence considered: `logs/clippy-20260927.log` finishes without warnings (exit `0`); `logs/test-final-20260927.log` reports 193 passed, 0 failed.
- Conclusion: `VERIFIED`.
- Limitations: Linux only.

VERIFY gate: `ESTABLISHED`; every success Claim is `VERIFIED`. `gui-clients-have-no-tty` stays `UNVERIFIED` and is carried forward.

## Learn

### Technical

`git apply --unidiff-zero` places a zero-context hunk by its new-side start, so any subset of `-U0` hunks must be renumbered before applying. A check that only compares the final tree cannot see misplaced intermediate commits; the per-commit assertions in `split_handles_alternating_targets_in_one_file` and the unit test `renumbers_the_new_side_from_the_hunks_included` now enforce this.

### Process

A premise probe verifies only the design it probed; when implementation deviated from the probed split design, the new assumption went unprobed until a deterministic test exposed it. Test ordering that depends on random commit IDs hid the risky order; forcing it made the test decisive.
A comparison script produced plausible "identical" results twice while testing the wrong path; reading each scenario's actual message, not just the diff verdict, caught it.
Neither is addf friction; no framework change.

LEARN gate: `ESTABLISHED`.

## Retention and promotion

The technical Learning is enforced by tests (tooling), not a document; no other permanent change.

### Promotion: success Claims

-   Claims: `modes-create-commits`, `reword-needs-target`, `non-interactive-unchanged`, `split-per-target`, `picker-choices`, `gates-green`.
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; the behavior is now specified in `spec/fixup.md` and covered by tests.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `unidiff-zero-apply`

-   Claim: [unidiff-zero-apply](#unidiff-zero-apply)
-   Will this Claim's validity outlive this Task and inform a future decision? `no`; it concerns a design that was not shipped.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `gui-clients-have-no-tty`

-   Claim: [gui-clients-have-no-tty](#gui-clients-have-no-tty)
-   Will this Claim's validity outlive this Task and inform a future decision? `yes`; whether a GUI client can meet the prompt matters to future picker work and support.
-   Disposition: carried forward to `framework:open-claims/gui-clients-have-no-tty.md`.

No Claim promoted to Knowledge.

## Archive readiness

The bundle holds its ledger and every log it cites under `logs/`; internal links are relative.
External references (`framework:` spec and the open Claim) are supplemental.

## Terminal record

### Summary

gir now has `gir amend`, `gir reword` and `gir squash` next to `gir fixup`, a numbered prompt when a person is at a terminal and gir cannot pick one target, and `--split` for one autosquash commit per target.
Callers without a terminal get the old refusals, plus one `--split` hint line.
The spec delta is published in `SPEC.md`, `spec/fixup.md` and `spec/cli.md`.

### Gate basis

All six success Claims are `VERIFIED` through the Verifications above, on Linux.
`amend-needs-staged` is `REFUTED` and shaped a Decision; `gui-clients-have-no-tty` is `UNVERIFIED` and carried forward.

## Stop record
