# Trace: cli

| Requirement | Test | Assertion |
|---|---|---|
| `req-cli-version` | `version_flags_print_crate_version` | `tests/args.rs:10-14` |
| `req-cli-help` | `help_flags_print_usage_for_every_subcommand` | `tests/args.rs:23-25`, `tests/args.rs:43-48` |
| `req-cli-usage-lists-commands` | `usage_lists_commands_with_their_arguments_and_flags` | `tests/args.rs:56`, `tests/args.rs:68` |
| `req-cli-unknown-long-option` | `unaccepted_long_flags_report_the_command_and_exit_two` | `tests/args.rs:84-98` |
| `req-cli-range-lint-only` | `range_is_rejected_outside_lint_and_accepted_by_lint` | `tests/args.rs:108-117` |
| `req-cli-runtime-error` | `command_errors_print_gir_prefix_and_exit_two` | `tests/args.rs:124-130`, `tests/args.rs:134-140`, `tests/args.rs:144-153` |
| `req-cli-unknown-short-option` | `unknown_short_flags_and_bad_arguments_exit_two` | `tests/args.rs:161-165` asserts both short-flag messages and exit `2` |
| `req-cli-bad-arguments` | `unknown_short_flags_and_bad_arguments_exit_two` | `tests/args.rs:169-171` asserts exit `2`, the message and the usage text |
| `req-cli-lint-arguments` | `unknown_short_flags_and_bad_arguments_exit_two` | `tests/args.rs:176-177` |
| `req-cli-hook-arguments` | `hook_takes_its_documented_arguments` | `tests/args.rs:186-194` asserts `unknown hook` for bad hook names and counts, bad arguments for a bare `gir hook`, and extra pre-push arguments accepted |
| `req-cli-repository-root` | `repository_commands_work_from_a_subdirectory` | `tests/args.rs:210-221` asserts doctor sees root files, fixup finds its target, and init stages both hooks from a subdirectory |
| `req-cli-runtime-error` | `commit_failure_reports_git_command_and_target` | `tests/fixup.rs:239-240` asserts git's own output precedes gir's single final message |
| `req-cli-version` | `version_and_help_ignore_trailing_arguments` | `tests/args.rs:228-229` |
| `req-cli-help` | `version_and_help_ignore_trailing_arguments` | `tests/args.rs:231-232` |
| `req-cli-help` | `unknown_short_option_before_help_exits_two` | `tests/args.rs:239-241` asserts exit 2, exact unknown-option stderr and empty stdout. |
