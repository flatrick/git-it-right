# Trace: fixup

| Requirement | Test | Assertion |
|---|---|---|
| `req-fixup-staged-line-target` | `fixup_finds_the_target_from_staged_lines_and_autosquash_folds_it` | `tests/cli.rs:77` asserts the created subject names `feat: add a`, the commit that last changed the staged line. |
| `req-fixup-base-limit` | `fixup_refuses_ambiguous_and_base_branch_targets` | `tests/cli.rs:151` asserts `already on the base branch` for a staged base-branch line. |
| `req-fixup-multiple-targets` | `fixup_refuses_ambiguous_and_base_branch_targets` | `tests/cli.rs:144` asserts `several commits` for staged changes from two commits. |
| `req-fixup-create-commit` | `fixup_finds_the_target_from_staged_lines_and_autosquash_folds_it` | `tests/cli.rs:77` asserts the new `fixup! feat: add a` commit subject; `tests/cli.rs:81` asserts its staged content folds into the target. |
| `req-fixup-rebase-left-to-user` | `fixup_finds_the_target_from_staged_lines_and_autosquash_folds_it` | `tests/cli.rs:77` asserts `fixup! feat: add a` is still HEAD before the test runs rebase. |
| `req-fixup-arguments` | `accepts_optional_target_and_dry_run_in_either_order` | `tests/fixup.rs:25-26` asserts both argument orders and automatic selection succeed with the expected target. |
| `req-fixup-staged-required` | `requires_staged_changes_even_with_explicit_target` | `tests/fixup.rs:36-37` asserts exit 2 and the exact diagnostic. |
| `req-fixup-explicit-target` | `explicit_target_ignores_line_origin_and_accepts_new_files` | `tests/fixup.rs:48-51` asserts the named target receives changes from another commit and a new file. |
| `req-fixup-explicit-target-on-branch` | `explicit_target_must_be_in_current_branch_history` | `tests/fixup.rs:64-67` asserts exit 2, the exact diagnostic, unchanged HEAD and staged index. |
| `req-fixup-explicit-target-after-base` | `explicit_target_must_be_after_base` | `tests/fixup.rs:76-79` asserts exit 2, the exact diagnostic, unchanged HEAD and staged index. |
| `req-fixup-invalid-target` | `rejects_invalid_explicit_commit` | `tests/fixup.rs:87-88` asserts exit 2 and the exact diagnostic. |
| `req-fixup-insertion-target` | `insertion_uses_adjacent_head_lines` | `tests/fixup.rs:97-98` asserts the insertion selects the adjacent line's commit. |
| `req-fixup-base-lookup` | `base_lookup_prefers_origin_head_over_main_and_limits_automatic_target` | `tests/fixup.rs:109-110` asserts origin/HEAD is selected before main by observing the base refusal. |
| `req-fixup-no-base-limit` | `automatic_target_has_no_base_limit_when_no_base_exists` | `tests/fixup.rs:146-147` asserts the existing commit is selected without a base. |
| `req-fixup-new-file` | `automatic_target_refuses_a_new_file` | `tests/fixup.rs:155-156` asserts exit 2 and the exact diagnostic. |
| `req-fixup-unattributed-lines` | `automatic_target_reports_unattributed_lines` | `tests/fixup.rs:165-167` asserts exit 2 and both required diagnostic fragments. |
| `req-fixup-multiple-target-details` | `automatic_target_reports_multiple_targets_and_split_hint` | `tests/fixup.rs:182-183` asserts both 10-character IDs, subjects and file locations. |
| `req-fixup-split-hint` | `automatic_target_reports_multiple_targets_and_split_hint` | `tests/fixup.rs:184` asserts the exact split hint. |
| `req-fixup-absorb-hint` | `multiple_targets_suggest_installed_git_absorb` | `tests/fixup.rs:201-202` asserts exit 2 and the installed git-absorb hint. |
| `req-fixup-dry-run` | `accepts_optional_target_and_dry_run_in_either_order` | `tests/fixup.rs:25-29` asserts exact stdout, exit 0, unchanged HEAD and staged index. |
| `req-fixup-success-message` | `successful_fixup_prints_target_and_base_and_preserves_config` | `tests/fixup.rs:214-215` asserts exit 0 and both exact stderr lines with short target and base IDs. |
| `req-fixup-success-message` | `success_without_base_prints_base_placeholder` | `tests/fixup.rs:227-228` asserts exit 0 and the `<base>` fallback. |
| `req-fixup-config-unchanged` | `successful_fixup_prints_target_and_base_and_preserves_config` | `tests/fixup.rs:217-218` asserts global and local Git configuration bytes are unchanged. |
| `req-fixup-commit-failure` | `commit_failure_reports_git_command_and_target` | `tests/fixup.rs:237-239` asserts exit 2, the exact failed command fragment and unchanged HEAD. |
| `req-fixup-base-lookup` | `base_lookup_falls_back_to_master` | `tests/fixup.rs:122-123` asserts master supplies the base when main is absent. |
| `req-fixup-base-lookup` | `base_lookup_falls_back_to_upstream` | `tests/fixup.rs:135-136` asserts upstream supplies the base when named branches are absent. |
