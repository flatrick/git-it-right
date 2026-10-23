# Verification: Git config survey baseline

## Claim

**Reference:** [Task claim](../TASK.md#git-config-scope)

## Method

Read the installed Git version, count the saved `git help --config` output, and compare the saved list with a fresh invocation.

## Expected observations

- `git --version` reports `2.56.0`.
- The saved config list contains 1,012 lines and matches the fresh output byte for byte.

## Observed results

### `current-git-config-list`

-   Fact: Git reports version `2.56.0`; the saved list has 1,012 lines; `cmp` reports no difference between the saved list and fresh `git help --config` output.
-   Source and method: `git --version`; `wc -l < addf/tasks/git-config-tips/evidence/git-config-names-2.56.0.txt`; `cmp` against fresh `git help --config` output.
-   Context: Linux, worktree `.worktrees/git-config-tips`, observed 2026-10-23.
-   Limitations: This verifies the installed environment only. It does not establish the config variable list for other Git releases.

## Evidence considered

-   [`git-config-reference.md`](../evidence/git-config-reference.md#git-config-reference-baseline) supports the version and count with direct command output and the official reference.

## Contradictory and inconclusive evidence

-   `NONE`

## Conclusion

<a id="conclusion"></a>

**Result:** `VERIFIED`

The installed Git version and complete config variable list match the Task's stated baseline.

## Remaining uncertainty

Git config options can change in later releases. The survey is explicitly scoped to Git 2.56.0 and the current manual.
