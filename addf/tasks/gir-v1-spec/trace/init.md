# Trace: init

| Requirement | Test | Assertion |
|---|---|---|
| `req-init-success-status` | `init_is_idempotent_and_keeps_user_edits` | `tests/cli.rs:191` asserts the first `gir init` succeeds; line 193 asserts a clean rerun succeeds. |
| `req-init-clean-rerun-output` | `init_is_idempotent_and_keeps_user_edits` | `tests/cli.rs:194` asserts the clean rerun's stderr lacks `wrote`. |
| `req-init-conflict-status` | `init_is_idempotent_and_keeps_user_edits` | `tests/cli.rs:198` asserts exit code `1` after editing `.girconfig`. |
| `req-init-local-hook-path` | `commit_msg_hook_fixes_and_rejects_through_real_git_commit` | `tests/cli.rs:9` asserts that Git reads `core.hooksPath` as `.githooks`. |
| `req-init-accept-force` | `force_replaces_differing_files` | `tests/init.rs:99` asserts that `--force` succeeds. |
| `req-init-require-repository` | `init_requires_a_repository` | `tests/init.rs:18-19` asserts exit `2` and the repository diagnostic. |
| `req-init-root-paths` | `init_writes_files_at_the_repository_root_from_a_subdirectory` | `tests/init.rs:216-219` asserts exit `0`, root placement and absence in the current subdirectory. |
| `req-init-success-output` | `init_creates_files_and_reports_each_write` | `tests/init.rs:27-28` asserts empty stdout and the next-step message. |
| `req-init-hook-files` | `init_creates_files_and_reports_each_write` | `tests/init.rs:33-37` asserts both hook headers and Unix executable permissions. |
| `req-init-config-file` | `init_creates_files_and_reports_each_write` | `tests/init.rs:41-42` asserts every required config entry and commented example. |
| `req-init-cliff-file` | `init_generates_cliff_format_and_default_parsers` | `tests/init.rs:53-54` asserts both sections and required Git settings. |
| `req-init-cliff-format` | `init_generates_cliff_format_and_default_parsers` | `tests/init.rs:53-54` asserts version and unreleased headings, grouping, scopes, breaking markers, and trimming. |
| `req-init-cliff-parsers` | `init_generates_cliff_format_and_default_parsers` | `tests/init.rs:56-58` asserts autosquash exclusion and every default type parser. |
| `req-init-cliff-groups` | `init_generates_cliff_format_and_default_parsers` | `tests/init.rs:57-58` asserts every built-in group title. |
| `req-init-cliff-groups` | `init_uses_kept_config_for_cliff_parsers` | `tests/init.rs:70-72` asserts a custom group title and omission of an absent default type. |
| `req-init-identical-files` | `identical_crlf_files_are_left_unchanged` | `tests/init.rs:120-123` asserts success and unchanged CRLF bytes for every generated file. |
| `req-init-keep-edits` | `init_keeps_differing_files_and_processes_the_rest` | `tests/init.rs:82-83` asserts changed hook and config contents remain unchanged. |
| `req-init-kept-output` | `init_keeps_differing_files_and_processes_the_rest` | `tests/init.rs:84-85` asserts each kept-file diagnostic. |
| `req-init-force-overwrite` | `force_replaces_differing_files` | `tests/init.rs:99-104` asserts exit `0`, replacement contents, and write diagnostics. |
| `req-init-config-before-cliff` | `init_uses_kept_config_for_cliff_parsers` | `tests/init.rs:68-72` asserts the kept config and the corresponding cliff parsers. |
| `req-init-stage-hooks` | `init_stages_written_hooks_as_executable` | `tests/init.rs:133-136` asserts executable index modes and the staged-path diagnostic. |
| `req-init-stage-hooks` | `init_restages_only_hooks_with_missing_or_wrong_index_modes` | `tests/init.rs:147-150` asserts repair of missing and wrong-mode index entries. |
| `req-init-stage-hooks` | `init_reports_only_the_hook_it_stages` | `tests/init.rs:160-161` asserts that the diagnostic lists only the staged hook. |
| `req-init-no-restage` | `clean_rerun_does_not_restage_or_repeat_hook_path_message` | `tests/init.rs:172-175` asserts success despite an index lock, unchanged index state, and no staging message. |
| `req-init-no-repeat-hook-path` | `clean_rerun_does_not_restage_or_repeat_hook_path_message` | `tests/init.rs:176` asserts the hook-path message is absent. |
| `req-init-write-diagnostics` | `init_creates_files_and_reports_each_write` | `tests/init.rs:29-30` asserts a write diagnostic for every created file. |
| `req-init-git-error` | `git_staging_failure_is_reported` | `tests/init.rs:197-198` asserts exit `2` and Git's staging error. |
| `req-init-git-error` | `git_hook_path_failure_is_reported` | `tests/init.rs:206-207` asserts exit `2` and Git's config error. |
| `req-init-config-error` | `invalid_config_stops_before_cliff_and_git_setup` | `tests/init.rs:185-188` asserts exit `2`, the `.girconfig` error, no `cliff.toml` and no hook path for an unknown key and an invalid `subjectMax` |
