# Trace: lint

| Requirement | Test | Assertion |
|---|---|---|
| `req-lint-strip-comments` | `message::tests::strips_comments_and_everything_after_scissors` | `src/message.rs:53` asserts the text excludes the comment, scissors and diff |
| `req-lint-strip-comments` | `commit_msg_hook_keeps_comments_and_verbose_diff_out_of_the_way` | `tests/cli.rs:108` asserts a diff line shaped like a header does not cause a rejection |
| `req-lint-comment-string` | `message::tests::honours_custom_comment_char_and_crlf` | `src/message.rs:61-62` asserts `;` lines are set aside and `#` lines kept |
| `req-lint-crlf` | `message::tests::honours_custom_comment_char_and_crlf` | `src/message.rs:61` asserts the text has LF only |
| `req-lint-normalize` | `cc::tests::safe_fixes_are_applied_and_reported` | `src/cc/tests.rs:44-45` on the case at line 40 asserts trailing spaces and blank runs vanish with only `header-spacing` reported |
| `req-lint-merge` | `cc::tests::special_subjects` | `src/cc/tests.rs:103` asserts `merge-commit` when `allowMerge` is false |
| `req-lint-revert` | `cc::tests::special_subjects` | `src/cc/tests.rs:104` asserts `revert-commit` when `allowRevert` is false |
| `req-lint-type-missing` | `cc::tests::unfixable_messages_are_rejected_with_a_hint` | `src/cc/tests.rs:68-69` asserts rule and hint prefix |
| `req-lint-type-unknown` | `cc::tests::unfixable_messages_are_rejected_with_a_hint` | `src/cc/tests.rs:72-74` asserts rule, `did you mean`, and hint header |
| `req-lint-scope-required` | `cc::tests::scope_rules_follow_config` | `src/cc/tests.rs:83` |
| `req-lint-scope-unknown` | `cc::tests::scope_rules_follow_config` | `src/cc/tests.rs:84-85` asserts unknown rejected and listed accepted |
| `req-lint-desc-empty` | `cc::tests::unfixable_messages_are_rejected_with_a_hint` | `src/cc/tests.rs:76` |
| `req-lint-header-length` | `cc::tests::unfixable_messages_are_rejected_with_a_hint` | `src/cc/tests.rs:77` |
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
| `req-lint-fixes-do-not-fail` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:86` asserts the commit with four fixes succeeds |
| `req-lint-fix-line` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:89` asserts `gir: fixed [type-alias]` |
| `req-lint-rejection-lines` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:96-99` asserts the three lines and the three-line bound |
| `req-lint-json` | `lint_range_reports_json_per_commit` | `tests/cli.rs:234-235` asserts the array opens with a `commit` key and carries the rule |
| `req-lint-json-escaping` | `report::tests::json_escapes` | `src/report.rs:81` |
| `req-lint-range` | `lint_range_reports_json_per_commit` | `tests/cli.rs:232` asserts exit `1` for a bad commit in range |
