# Trace: init

| Requirement | Test | Assertion |
|---|---|---|
| `req-init-success-status` | `init_is_idempotent_and_keeps_user_edits` | `tests/cli.rs:191` asserts the first `gir init` succeeds; line 193 asserts a clean rerun succeeds. |
| `req-init-clean-rerun-output` | `init_is_idempotent_and_keeps_user_edits` | `tests/cli.rs:194` asserts the clean rerun's stderr lacks `wrote`. |
| `req-init-conflict-status` | `init_is_idempotent_and_keeps_user_edits` | `tests/cli.rs:198` asserts exit code `1` after editing `.girconfig`. |
| `req-init-local-hook-path` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:9` asserts that Git reads `core.hooksPath` as `.githooks`. |
