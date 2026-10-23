# Verification — Git tips and doctor recommendation

## Catalog claim

**Reference:** [Task claim](TASK.md#git-config-catalog)

### `git-config-catalog`

-   Claim: `gir tips` prints the selected settings in presentation-first order and explains the effect and use case for every behavior-changing setting.
-   State: `VERIFIED`
-   Scope: Integrated feature branch `b049fbd`, Linux, built with the repository's Rust toolchain.
-   Consequence if false: Developers may apply settings without understanding their effects or find the catalog hard to use.
-   Basis: [Conclusion](#catalog-conclusion).

## Read-only claim

**Reference:** [Task claim](TASK.md#tips-read-only)

### `tips-read-only`

-   Claim: Running `gir tips` does not modify global or repository Git config and does not require a Git executable or repository.
-   State: `VERIFIED`
-   Scope: Integrated feature branch `b049fbd`, Linux.
-   Consequence if false: A discovery command could change a developer's Git behavior without consent.
-   Basis: [Conclusion](#read-only-conclusion).

## Doctor claim

**Reference:** [Task claim](TASK.md#doctor-diff-color-moved)

### `doctor-diff-color-moved`

-   Claim: Doctor recommends `diff.colorMoved=plain`, fixes it in local config when unset, and preserves a different existing effective value.
-   State: `VERIFIED`
-   Scope: Integrated feature branch `b049fbd`, Linux.
-   Consequence if false: Doctor could omit the recommendation or overwrite a developer's choice.
-   Basis: [Conclusion](#doctor-conclusion).

## Method

Ran `cargo test` and `cargo clippy --all-targets -- -D warnings` from the feature worktree. The tests exercise the built CLI and use isolated temporary repositories and config files.

## Observed results

-   `cargo test` exited `0`. Unit, integration, and doc tests passed. `tests/tips.rs::tips_lists_safe_settings_first_with_effects_and_use_cases` checks all twelve commands, their exact order, both group headings, and effect/use-case lines for every item. It checks the five behavior entries for the disclosed effects.
-   `tests/tips.rs::tips_works_outside_a_repository_without_git_or_config_writes` runs tips from outside a repository with an empty `PATH`, compares output with an in-repository run, and verifies that both the isolated global config and repository config are byte-for-byte unchanged.
-   `tests/doctor.rs::doctor_recommends_and_sets_each_unset_git_setting` checks that doctor warns for unset `diff.colorMoved` and that `--fix` sets local `plain`.
-   `tests/doctor.rs::doctor_preserves_nonempty_existing_recommendations` checks that a local `zebra` value is reported and preserved by `--fix`.
-   `tests/doctor.rs::doctor_respects_inherited_diff_color_moved_without_local_copy` checks that a global `zebra` value is reported and remains global after `--fix`.
-   `cargo clippy --all-targets -- -D warnings` exited `0` with no warnings.
-   The implementation owner also ran the built binary outside a repository with Git absent from `PATH`; it exited `0` and did not create the isolated global config file.

## Evidence considered

-   The CLI integration tests directly observe ordering, disclosures, repository independence, missing-Git behavior, config immutability, exit codes, and doctor report/fix behavior.
-   The static command implementation in `src/cmd/tips.rs` formats a fixed catalog and has no Git or config calls. Its dispatcher path in `src/main.rs` only renders and writes the page.
-   Clippy checks the integrated target set for compiler lints; it does not establish product behavior.

## Contradictory and inconclusive evidence

-   `NONE` for the three claims within the Linux test scope.
-   Windows and macOS were not available for execution. The tips command has no platform-specific branches, but cross-platform output is not independently tested here.

## Conclusions

<a id="catalog-conclusion"></a>
### Catalog

**Result:** `VERIFIED`

The CLI test observes all twelve settings in the agreed order and checks each entry's effect and useful situation. It checks the exact operational disclosures for all five behavior-changing entries.

<a id="read-only-conclusion"></a>
### Read-only behavior

**Result:** `VERIFIED`

The CLI succeeds outside a repository with an empty `PATH`, and the isolated local and global config files remain byte-for-byte unchanged after both runs.

<a id="doctor-conclusion"></a>
### Doctor recommendation

**Result:** `VERIFIED`

Doctor recommends and fixes the unset local value. It reports and preserves an existing local or inherited global value.

## Remaining uncertainty

The command and tests were run on Linux only. No setting is automatically applied by `gir tips`; users choose whether to run each printed command.
