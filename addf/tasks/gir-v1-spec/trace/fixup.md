# Trace: fixup

| Requirement | Test | Assertion |
|---|---|---|
| `req-fixup-staged-line-target` | `fixup_finds_the_target_from_staged_lines_and_autosquash_folds_it` | `tests/cli.rs:77` asserts the created subject names `feat: add a`, the commit that last changed the staged line. |
| `req-fixup-base-limit` | `fixup_refuses_ambiguous_and_base_branch_targets` | `tests/cli.rs:151` asserts `already on the base branch` for a staged base-branch line. |
| `req-fixup-multiple-targets` | `fixup_refuses_ambiguous_and_base_branch_targets` | `tests/cli.rs:144` asserts `several commits` for staged changes from two commits. |
| `req-fixup-create-commit` | `fixup_finds_the_target_from_staged_lines_and_autosquash_folds_it` | `tests/cli.rs:77` asserts the new `fixup! feat: add a` commit subject; `tests/cli.rs:81` asserts its staged content folds into the target. |
| `req-fixup-rebase-left-to-user` | `fixup_finds_the_target_from_staged_lines_and_autosquash_folds_it` | `tests/cli.rs:77` asserts `fixup! feat: add a` is still HEAD before the test runs rebase. |
