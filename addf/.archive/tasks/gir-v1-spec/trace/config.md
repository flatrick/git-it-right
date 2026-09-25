# Trace: config

| Requirement | Test | Assertion |
|---|---|---|
| `req-config-scopes` | `cc::tests::scope_rules_follow_config` | `src/cc/tests.rs:84-85` |
| `req-config-scope-required` | `cc::tests::scope_rules_follow_config` | `src/cc/tests.rs:83` |
| `req-config-desc-case` | `cc::tests::desc_case_lower_is_opt_in` | `src/cc/tests.rs:90-92` |
| `req-config-location` | `config_comes_from_repo_root_and_only_gir_keys` | `tests/config.rs:22-25` |
| `req-config-absent` | `missing_config_and_non_repo_use_defaults` | `tests/config.rs:33-36` |
| `req-config-case-insensitive-keys` | `config_key_names_are_case_insensitive` | `tests/config.rs:43-44` |
| `req-config-types` | `types_default_and_custom_lists_accept_spaces_and_commas` | `tests/config.rs:51-53` |
| `req-config-subject-max` | `subject_max_defaults_to_72_and_counts_characters` | `tests/config.rs:60-68` |
| `req-config-allow-merge-revert` | `merge_and_revert_are_allowed_by_default_and_can_be_disabled` | `tests/config.rs:77-81` |
| `req-config-booleans` | `boolean_keys_accept_all_documented_spellings` | `tests/config.rs:91-93` |
| `req-config-aliases` | `default_aliases_and_configured_override_map_to_types` | `tests/config.rs:102-107` |
| `req-config-hook-missing` | `hook_missing_is_accepted_without_changing_config_behavior` | `tests/config.rs:116-117` |
| `req-config-invalid-value` | `invalid_values_report_key_value_expected_form_and_exit_two` | `tests/config.rs:126-131` |
| `req-config-unknown-key` | `unknown_gir_key_reports_error_while_other_sections_are_ignored` | `tests/config.rs:140-141` |
| `req-config-explain-tolerates-invalid` | `explain_uses_defaults_when_config_is_invalid` | `tests/config.rs:150-152` |
| `req-config-malformed` | `malformed_config_fails_and_config_without_gir_keys_uses_defaults` | `tests/config.rs:161-165` |
