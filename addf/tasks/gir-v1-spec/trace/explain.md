# Trace: explain

| Requirement | Test | Assertion |
|---|---|---|
| `req-explain-unique-topics` | `explain::tests::topics_are_unique` | `src/explain.rs:67` asserts deduplication does not reduce the topic count. |
| `req-explain-listed-pages` | `explain_every_listed_topic` | `tests/cli.rs:210` asserts each listed topic succeeds and has nonempty stdout. |
| `req-explain-known-pages` | `explain::tests::every_rule_and_type_has_a_page` | `src/explain.rs:56-57` asserts every named rule, default type, and extra topic has a nonempty page. |
| `req-explain-no-next-heading` | `explain::tests::every_rule_and_type_has_a_page` | `src/explain.rs:57` asserts a page contains no subsequent `## ` heading. |
