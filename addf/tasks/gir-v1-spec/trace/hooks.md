# Trace: hooks

| Requirement | Test | Assertion |
|---|---|---|
| `req-hooks-commit-msg-fixes` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:14-15` asserts the commit succeeds with the fixed message recorded |
| `req-hooks-commit-msg-rejects` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:22,24` asserts the commit fails with `commit rejected` |
| `req-hooks-commit-msg-keeps-tail` | `commit_msg_hook_keeps_comments_and_verbose_diff_out_of_the_way` | `tests/cli.rs:38-39` |
| `req-hooks-pre-push-rejects` | `pre_push_rejects_unsquashed_fixups_and_no_verify_commits` | `tests/cli.rs:55-56,61-62` asserts the push fails on `fixup-unsquashed` and on `push rejected [type-missing]` |
| `req-hooks-shim-delegates` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:15` only passes when `git commit` ran gir through `.githooks/commit-msg` |
| `req-hooks-commit-msg-latency` | `commit_msg_hook_stays_within_budget` | `tests/perf.rs:48` |
