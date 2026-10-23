# Verification: doctor diff.colorMoved recommendation

## Claim

**Reference:** [Task claim](../TASK.md#doctor-diff-color-moved)

-   Claim: Doctor recommends `diff.colorMoved=plain`, fixes it in local config when unset, and preserves a different existing effective value.
-   State: `VERIFIED`
-   Scope: Integrated feature branch `b049fbd`, Linux.
-   Consequence if false: Doctor could omit the recommendation or overwrite a developer's choice.
-   Basis: [Conclusion](#conclusion).

## Method

Ran `cargo test` from the feature worktree. The doctor integration tests inspect its report and query isolated local and global Git config after `--fix`.

## Expected observations

-   An unset value appears as a warning, and `--fix` writes local `plain`.
-   A different existing local value is reported and preserved.
-   A different inherited global value is reported and is not copied into local config.

## Observed results

-   `cargo test` exited `0`; `tests/doctor.rs::doctor_recommends_and_sets_each_unset_git_setting` passed and verified the local value is `plain` after `--fix`.
-   `tests/doctor.rs::doctor_preserves_nonempty_existing_recommendations` passed and verified local `zebra` remains unchanged.
-   `tests/doctor.rs::doctor_respects_inherited_diff_color_moved_without_local_copy` passed and verified the global `zebra` value remains global with no local value.

## Evidence considered

-   The doctor tests observe warning, fix, and preservation behavior through the CLI and query the resulting Git config.
-   The added `LOCAL_CONFIG` entry uses the existing check and local fix path.

## Contradictory and inconclusive evidence

-   `NONE` within the Linux test scope.

## Conclusion

<a id="conclusion"></a>

**Result:** `VERIFIED`

Doctor recommends and sets the unset local value. It reports and preserves an existing local value and does not copy an inherited global value locally.

## Remaining uncertainty

Windows and macOS were not available for execution. The added recommendation uses the existing cross-platform config path.
