# Verification: Git tips read-only behavior

## Claim

**Reference:** [Task claim](../TASK.md#tips-read-only)

-   Claim: `gir tips` does not modify global or repository Git config and does not require a Git executable or repository.
-   State: `VERIFIED`
-   Scope: Integrated feature branch `b049fbd`, Linux.
-   Consequence if false: A discovery command could change a developer's Git behavior without consent.
-   Basis: [Conclusion](#conclusion).

## Method

Ran `cargo test` from the feature worktree. The CLI integration test compares config files before and after running the command inside a test repository and outside one with an empty `PATH`.

## Expected observations

-   The command exits `0` outside a repository even when no Git executable is available on `PATH`.
-   The isolated global and repository config files are byte-for-byte unchanged after both runs.

## Observed results

-   `cargo test` exited `0`; `tests/tips.rs::tips_works_outside_a_repository_without_git_or_config_writes` passed.
-   The test verifies that inside and outside output matches and that both config files match their original bytes.

## Evidence considered

-   The integration test observes the built CLI with an empty `PATH`, an isolated `GIT_CONFIG_GLOBAL`, and a temporary repository. It compares the relevant config bytes before and after execution.
-   `src/main.rs` dispatches `tips` directly to the static renderer. `src/cmd/tips.rs` has no Git, repository, or config calls.

## Contradictory and inconclusive evidence

-   `NONE` within the Linux test scope.

## Conclusion

<a id="conclusion"></a>

**Result:** `VERIFIED`

The CLI exits successfully outside a repository with an empty `PATH`, and the isolated global and repository config files remain unchanged.

## Remaining uncertainty

Windows and macOS were not available for execution. The command uses no platform-specific process or file operations.
