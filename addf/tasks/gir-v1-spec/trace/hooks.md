# Trace: hooks

| Requirement | Test | Assertion |
|---|---|---|
| `req-hooks-commit-msg-fixes` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:14-15` asserts the commit succeeds with the fixed message recorded |
| `req-hooks-commit-msg-rejects` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:22,24` asserts the commit fails with `commit rejected` |
| `req-hooks-commit-msg-keeps-tail` | `commit_msg_hook_keeps_comments_and_verbose_diff_out_of_the_way` | `tests/cli.rs:38-39` |
| `req-hooks-commit-msg-autosquash` | `commit_msg_accepts_all_autosquash_prefixes_unchanged` | `tests/hooks.rs:39-41` asserts success, unchanged content, and no rejection for all three prefixes |
| `req-hooks-pre-push-scope` | `pre_push_lints_only_commits_missing_from_remote` | `tests/hooks.rs:58-59`, `tests/hooks.rs:63`, `tests/hooks.rs:68-77` assert remote commits are excluded and new commits are linted for existing, new, and failed-range branches |
| `req-hooks-pre-push-deletes` | `pre_push_skips_branch_deletions` | `tests/hooks.rs:85-87` asserts success and no output for an all-zero local SHA |
| `req-hooks-pre-push-rejects` | `pre_push_rejects_unsquashed_fixups_and_no_verify_commits` | `tests/cli.rs:55-56,61-62` asserts the push fails on `fixup-unsquashed` and on `push rejected [type-missing]` |
| `req-hooks-pre-push-rejects` | `pre_push_rejects_commits_that_skipped_safe_fixes` | `tests/hooks.rs:123-126` asserts `fix-pending` with the short SHA, the fixed subject, and an unmodified commit |
| `req-hooks-shim-delegates` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:15` only passes when `git commit` ran gir through `.githooks/commit-msg` |
| `req-hooks-shim-missing-warn` | `installed_hooks_warn_when_gir_is_missing` | `tests/hooks.rs:96-98` asserts both scripts skip with the named warning and exit 0 |
| `req-hooks-shim-missing-fail` | `installed_hooks_block_when_gir_is_missing_and_configured_to_fail` | `tests/hooks.rs:109-111` asserts both scripts block with the named warning and exit 1 |
| `req-hooks-commit-msg-latency` | `commit_msg_hook_stays_within_budget` | `tests/perf.rs:48` |
