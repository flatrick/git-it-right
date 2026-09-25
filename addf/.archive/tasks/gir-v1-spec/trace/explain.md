# Trace: explain

| Requirement | Test | Assertion |
|---|---|---|
| `req-explain-list-format` | `explain_lists_topics_on_one_line` | `tests/explain.rs:16-21` asserts exit 0, the prefix, one line, a final newline, and single-space separators. |
| `req-explain-unique-topics` | `explain::tests::topics_are_unique` | `src/explain.rs:67` asserts deduplication does not reduce the topic count. |
| `req-explain-listed-pages` | `explain_every_listed_topic` | `tests/cli.rs:210` asserts each listed topic succeeds and has nonempty stdout. |
| `req-explain-config-page-aliases` | `config_page_shows_the_init_aliases` | `tests/explain.rs:32-35` asserts both sample aliases match the generated config. |
| `req-explain-type-examples-valid` | `explain::tests::cheatsheet_examples_pass_the_linter_untouched` | `src/explain.rs:76-80` asserts each type has two examples and each example starts with its type and passes lint without fixes. |
| `req-explain-single-section` | `topic_pages_contain_only_their_section_and_one_final_newline` | `tests/explain.rs:51-52` asserts each page equals its section body plus one newline. |
| `req-explain-no-next-heading` | `explain::tests::every_rule_and_type_has_a_page` | `src/explain.rs:57` asserts a page contains no subsequent `## ` heading. |
| `req-explain-case-sensitive` | `topic_names_are_case_sensitive` | `tests/explain.rs:61-64` asserts `feat` succeeds and `Feat` is rejected by name. |
| `req-explain-unknown-topic` | `unknown_topic_reports_name_and_available_topics` | `tests/explain.rs:72-75` asserts exit 2, the named error and topics on stderr, and empty stdout. |
| `req-explain-types-config` | `types_page_uses_configured_order_and_defaults_without_usable_config` | `tests/explain.rs:83-89` asserts the heading, configured order, and default order after invalid config. |
| `req-explain-types-summaries` | `types_page_uses_cheatsheet_summaries_and_group_fallback` | `tests/explain.rs:102-104` asserts all default summaries and a custom type's group fallback. |
| `req-explain-types-aliases` | `types_page_prints_alias_mappings_and_footer` | `tests/explain.rs:113,117` asserts default and configured aliases appear as mappings. |
| `req-explain-types-footer` | `types_page_prints_alias_mappings_and_footer` | `tests/explain.rs:114` asserts the exact final footer. |
| `req-explain-read-only` | `explain_does_not_change_repository_files_or_git_config` | `tests/explain.rs:148-151` asserts repository files, global config, index and config remain unchanged. |
| `req-explain-known-pages` | `explain::tests::every_rule_and_type_has_a_page` | `src/explain.rs:55-57` panics on a missing page and asserts each listed topic, including `fix-pending`, is nonempty |
