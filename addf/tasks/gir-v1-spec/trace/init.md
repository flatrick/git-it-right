# Trace: init

| Requirement | Test | Assertion |
|---|---|---|
| `req-init-success-status` | `init_is_idempotent_and_keeps_user_edits` | `tests/cli.rs:263` asserts the first `gir init` succeeds; line 265 asserts a clean rerun succeeds. |
| `req-init-clean-rerun-output` | `init_is_idempotent_and_keeps_user_edits` | `tests/cli.rs:266` asserts the clean rerun's stderr lacks `wrote`. |
| `req-init-conflict-status` | `init_is_idempotent_and_keeps_user_edits` | `tests/cli.rs:270` asserts exit code `1` after editing `.girconfig`. |
| `req-init-local-hook-path` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:81` asserts that Git reads `core.hooksPath` as `.githooks`. |
