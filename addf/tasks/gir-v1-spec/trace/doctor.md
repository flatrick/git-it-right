# Trace: doctor

| Requirement | Test | Assertion |
|---|---|---|
| `req-doctor-fix-flag` | `doctor_fix_converges` | `tests/cli.rs:184-185` asserts that `--fix` created `.gitattributes` and set local `rebase.autoSquash=true`. |
| `req-doctor-warning-exit` | `doctor_fix_converges` | `tests/cli.rs:172,183` asserts exit `1` with warnings and `0` after fixing them. |
| `req-doctor-fix-converges` | `doctor_fix_converges` | `tests/cli.rs:182-183` asserts that the second run has no `warn ` lines and exits `0`. |
| `req-doctor-autosquash` | `doctor_fix_converges` | `tests/cli.rs:174-175,185` asserts the warning names `rebase.autoSquash` and the local setting becomes `true`. |
| `req-doctor-gitattributes-missing` | `doctor_fix_converges` | `tests/cli.rs:174-175,184` asserts the missing-file warning and that `--fix` creates `.gitattributes`. |
| `req-doctor-exec-bit` | `doctor_fix_converges` | `tests/cli.rs:174-175,182` asserts the initial `exec-bit` warning disappears after `--fix`. |
| `req-doctor-outside-repo` | `doctor_outside_git_repository_reports_error` | `tests/doctor.rs:20-21` asserts exit 2 and the exact stderr. |
| `req-doctor-report-lines` | `doctor_reports_check_lines_and_summary_counts` | `tests/doctor.rs:31-34,40` asserts each check line has a status, ID, and message, and fixed checks use `fixed`. |
| `req-doctor-summary` | `doctor_reports_check_lines_and_summary_counts` | `tests/doctor.rs:37,41-42` asserts summary counts, fix hint, ending, and absent hint after fixing. |
| `req-doctor-missing-hooks` | `doctor_missing_hooks_are_not_created_by_fix` | `tests/doctor.rs:61,63-64` asserts the warning and absent directory after `--fix`. |
| `req-doctor-hooks-path` | `doctor_repairs_inactive_hooks_path_locally` | `tests/doctor.rs:72,74-75` asserts the warning, fixed status, and local config value. |
| `req-doctor-recommended-config` | `doctor_recommends_and_sets_each_unset_git_setting` | `tests/doctor.rs:88,92-93` asserts warnings and local values for all eight settings. |
| `req-doctor-existing-config` | `doctor_preserves_nonempty_existing_recommendations` | `tests/doctor.rs:102,104-105` asserts the info line and unchanged local value. |
| `req-doctor-pull-rebase` | `doctor_respects_pull_rebase_without_setting_pull_ff` | `tests/doctor.rs:113,115-116` asserts no `pull.ff` line or local setting after `--fix`. |
| `req-doctor-windows-longpaths` | `windows: doctor_repairs_unset_windows_longpaths` | `tests/doctor.rs:124,126-127` asserts the warning, fixed status, and local value. |
| `req-doctor-user-identity` | `doctor_suggests_missing_identity_and_default_branch_without_setting_them` | `tests/doctor.rs:136,141-142` asserts each identity warning remains and neither key is set locally. |
| `req-doctor-default-branch` | `doctor_suggests_missing_identity_and_default_branch_without_setting_them` | `tests/doctor.rs:138,144-145` asserts the exact info line and unchanged local config. |
| `req-doctor-gitattributes-content` | `doctor_writes_complete_gitattributes_and_editorconfig` | `tests/doctor.rs:155,157,160` asserts the LF, CRLF, and binary rules in the created file. |
| `req-doctor-gitattributes-autocrlf` | `doctor_reports_autocrlf_when_gitattributes_is_missing` | `tests/doctor.rs:180` asserts the exact extended warning. |
| `req-doctor-gitattributes-rule` | `doctor_does_not_replace_readable_gitattributes_without_auto_rule` | `tests/doctor.rs:188,190-191` asserts the info line and unchanged file after `--fix`. |
| `req-doctor-editorconfig` | `doctor_writes_complete_gitattributes_and_editorconfig` | `tests/doctor.rs:153,163-164` asserts fixed status and the created file's required settings. |
| `req-doctor-girconfig` | `doctor_reports_invalid_girconfig_without_editing_it` | `tests/doctor.rs:199,201-202` asserts the error line and unchanged file. |
| `req-doctor-ignore-rules` | `doctor_appends_missing_ignore_rules_after_newline` | `tests/doctor.rs:212,215,217,219` asserts missing patterns, fixed status, newline, and appended rules. |
| `req-doctor-ignore-patterns` | `doctor_checks_stack_and_local_directory_ignore_patterns_when_present` | `tests/doctor.rs:230-233,238,240` asserts stack and local patterns appear when relevant and `/target/` stays absent without Cargo. |
| `req-doctor-ignore-patterns` | `doctor_appends_missing_ignore_rules_after_newline` | `tests/doctor.rs:212,219` asserts the three universal patterns and Cargo pattern. |
| `req-doctor-tracked-ignore-probes` | `doctor_does_not_warn_for_exact_tracked_ignore_probe` | `tests/doctor.rs:249-250` asserts the tracked `.env` probe is absent from ignore warnings. |
| `req-doctor-git-cliff` | `doctor_reports_missing_git_cliff_on_path` | `tests/doctor.rs:260` asserts the missing-program info line with a controlled PATH. |
| `req-doctor-case-collision` | `doctor_reports_case_collisions_without_renaming_index_entries` | `tests/doctor.rs:272-273,275-276` asserts warning, paths, and unchanged index after `--fix`. |
| `req-doctor-windows-names` | `doctor_reports_windows_unsafe_index_names_without_renaming` | `tests/doctor.rs:288,290-291` asserts warning, paths, and unchanged index after `--fix`. |
| `req-doctor-index-clean` | `doctor_counts_clean_index_as_ok_without_index_warnings` | `tests/doctor.rs:298-301` asserts no index warnings and one OK check in the summary. |
| `req-doctor-ignore-patterns` | `doctor_checks_each_stack_pattern_for_its_marker` | `tests/doctor.rs:317-319` asserts every stack marker's patterns appear without unrelated Cargo ignores. |
| `req-doctor-case-collision` | `doctor_does_not_report_the_stages_of_an_unmerged_path_as_a_case_collision` | `tests/doctor.rs:333` |
