# Trace: lint

| Requirement | Test | Assertion |
|---|---|---|
| `req-lint-strip-comments` | `message::tests::strips_comments_and_everything_after_scissors` | `src/message.rs:53` asserts the text excludes the comment, scissors and diff |
| `req-lint-strip-comments` | `commit_msg_hook_keeps_comments_and_verbose_diff_out_of_the_way` | `tests/cli.rs:36` asserts a diff line shaped like a header does not cause a rejection |
| `req-lint-comment-string` | `message::tests::honours_custom_comment_char_and_crlf` | `src/message.rs:61-62` asserts `;` lines are set aside and `#` lines kept |
| `req-lint-crlf` | `message::tests::honours_custom_comment_char_and_crlf` | `src/message.rs:61` asserts the text has LF only |
| `req-lint-normalize` | `cc::tests::safe_fixes_are_applied_and_reported` | `src/cc/tests.rs:44-45` on the case at line 40 asserts trailing spaces and blank runs vanish with only `header-spacing` reported |
| `req-lint-autosquash-exempt` | `autosquash_subjects_pass_file_and_stdin_without_checks` | `tests/lint.rs:21-25` asserts all three prefixes pass file and stdin lint without diagnostics |
| `req-lint-merge` | `cc::tests::special_subjects` | `src/cc/tests.rs:103` asserts `merge-commit` when `allowMerge` is false |
| `req-lint-revert` | `cc::tests::special_subjects` | `src/cc/tests.rs:104` asserts `revert-commit` when `allowRevert` is false |
| `req-lint-empty` | `cc::tests::empty_message_is_rejected_after_normalization` | `src/cc/tests.rs:121-122` asserts normalized empty input has rule `empty` and fails |
| `req-lint-merge` | `merge_and_revert_are_allowed_by_default_and_can_be_disabled` | `tests/config.rs:77` asserts a default merge subject passes |
| `req-lint-revert` | `merge_and_revert_are_allowed_by_default_and_can_be_disabled` | `tests/config.rs:77` asserts a default revert subject passes |
| `req-lint-type-missing` | `cc::tests::unfixable_messages_are_rejected_with_a_hint` | `src/cc/tests.rs:68-69` asserts rule and hint prefix |
| `req-lint-type-unknown` | `cc::tests::unfixable_messages_are_rejected_with_a_hint` | `src/cc/tests.rs:72-74` asserts rule, `did you mean`, and hint header |
| `req-lint-scope-required` | `cc::tests::scope_rules_follow_config` | `src/cc/tests.rs:83` |
| `req-lint-scope-unknown` | `cc::tests::scope_rules_follow_config` | `src/cc/tests.rs:84-85` asserts unknown rejected and listed accepted |
| `req-lint-desc-empty` | `cc::tests::unfixable_messages_are_rejected_with_a_hint` | `src/cc/tests.rs:76` |
| `req-lint-desc-empty` | `cc::tests::description_left_empty_by_period_fix_is_desc_empty` | `src/cc/tests.rs:135` |
| `req-lint-header-length` | `cc::tests::unfixable_messages_are_rejected_with_a_hint` | `src/cc/tests.rs:77` |
| `req-lint-spec` | `cc::tests::invalid_conventional_scope_is_rejected_as_spec` | `src/cc/tests.rs:128-129` asserts an otherwise accepted header with nested scope syntax fails with only rule `spec` |
| `req-lint-valid-unchanged` | `cc::tests::spec_examples_pass_untouched` | `src/cc/tests.rs:24-25` asserts no fixes, no violations, same text |
| `req-lint-header-spacing` | `cc::tests::safe_fixes_are_applied_and_reported` | `src/cc/tests.rs:44-45` on the case at line 33 |
| `req-lint-header-spacing` | `cc::header::tests::render_is_canonical` | `src/cc/header.rs:78` |
| `req-lint-type-case` | `cc::tests::safe_fixes_are_applied_and_reported` | `src/cc/tests.rs:44-45` on the case at line 32,35 |
| `req-lint-type-alias` | `cc::tests::safe_fixes_are_applied_and_reported` | `src/cc/tests.rs:44-45` on the case at line 34-35 |
| `req-lint-scope-empty` | `cc::tests::safe_fixes_are_applied_and_reported` | `src/cc/tests.rs:44-45` on the case at line 36 |
| `req-lint-desc-period` | `cc::tests::safe_fixes_are_applied_and_reported` | `src/cc/tests.rs:44-45` on the case at line 34 removes the period |
| `req-lint-desc-case` | `cc::tests::desc_case_lower_is_opt_in` | `src/cc/tests.rs:90-93` asserts `any` untouched, `lower` lowercases, `API` kept |
| `req-lint-body-separator` | `cc::tests::safe_fixes_are_applied_and_reported` | `src/cc/tests.rs:44-45` on the case at line 37 |
| `req-lint-breaking-footer` | `cc::tests::safe_fixes_are_applied_and_reported` | `src/cc/tests.rs:44-45` on the case at line 38-39 |
| `req-lint-breaking-footer` | `cc::tests::breaking_change_hyphen_form_is_left_alone` | `src/cc/tests.rs:62` |
| `req-lint-fix-idempotent` | `cc::tests::fixes_are_idempotent` | `src/cc/tests.rs:54-55` |
| `req-lint-fixes-do-not-fail` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:14` asserts the commit with four fixes succeeds |
| `req-lint-fix-line` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:17` asserts `gir: fixed [type-alias]` |
| `req-lint-rejection-lines` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:24-27` asserts the three lines and the three-line bound |
| `req-lint-json` | `lint_range_reports_json_per_commit` | `tests/cli.rs:162-163` asserts the array opens with a `commit` key and carries the rule |
| `req-lint-json-escaping` | `report::tests::json_escapes` | `src/report.rs:81` |
| `req-lint-file` | `lint_file_rejects_without_changing_file` | `tests/lint.rs:34-36` asserts exit 1, `rejected` label, and unchanged file |
| `req-lint-file-fix` | `lint_file_fix_rewrites_message_and_preserves_comments` | `tests/lint.rs:44-47` asserts exit 0, fixed file with set-aside comment, and both fix lines |
| `req-lint-stdin` | `lint_reads_stdin_with_or_without_dash_and_prints_fixed_message` | `tests/lint.rs:55-61` asserts both stdin spellings print fixed text and JSON mode does not print plain text |
| `req-lint-range` | `lint_range_reports_json_per_commit` | `tests/cli.rs:160` asserts exit `1` for a bad commit in range |
| `req-lint-range-unsquashed` | `lint_range_rejects_each_unsquashed_autosquash_subject` | `tests/lint.rs:72-74` asserts exit 1, SHA label, rule `fixup-unsquashed`, and autosquash hint for each prefix |
| `req-lint-range-fix-pending` | `lint_range_rejects_every_pending_safe_fix_and_shows_fixed_subject` | `tests/lint.rs:84-90` asserts exit 1, rule `fix-pending`, all four pending rules, and the fixed subject hint |
| `req-lint-range-no-fix` | `lint_range_refuses_fix` | `tests/lint.rs:98-99` |
| `req-lint-range` | `lint_range_accepts_any_revision_git_log_takes` | `tests/lint.rs:108-109` asserts a bare revision lints its whole history |
