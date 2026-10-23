# Evidence: Current Git config reference

## Context

The Task needs a reproducible baseline for its survey and source material for each candidate setting's effect and limits.

<a id="git-config-reference-baseline"></a>
## Observation `git-config-reference-baseline`

- The installed Git reports version `2.56.0`.
- `git help --config` prints 1,012 config variable names in this environment.
- The current online `git-config` reference identifies its latest version as `2.56.0`, updated 2026-09-28.
- `branch.sort` and `tag.sort` set the default ordering for their respective listings.
- `column.ui=auto` enables columns when output goes to a terminal.
- `diff.colorMoved` colors moved lines; `diff.mnemonicPrefix` changes diff path prefixes.
- `diff.context` changes the default number of diff context lines from three.
- `diff.renames=true`, `push.default=simple`, and `color.ui=auto` are defaults in the current manual.
- `commit.verbose` adds diff information to the commit message editor context.
- `blame.ignoreRevsFile` tells blame to ignore listed commit IDs.
- `user.useConfigOnly=true` prevents Git from guessing `user.name` and `user.email`.
- `help.autoCorrect=prompt` asks before running a suggested command after a typo.
- `rebase.missingCommitsCheck=error` stops an interactive rebase when commits are removed from the todo list; an explicit `drop` remains available.
- `rerere.autoUpdate=true` updates the index after Git reuses a recorded conflict resolution.
- `fetch.all=true` attempts to update every configured remote.
- `fetch.pruneTags=true` prunes local tags according to the active fetch refspec when pruning is enabled.
- `push.followTags=true` enables `--follow-tags` for pushes.
- `fetch.fsckObjects=true` checks fetched objects. `transfer.fsckObjects` supplies a fallback when fetch or receive-specific settings are unset.
- Failed fetch-side object checking can leave malformed objects unreferenced in the repository; the check does not quarantine them.
- `core.fsmonitor=true` enables the built-in monitor on currently supported Windows and macOS platforms. Older Git versions may treat a boolean value as a hook path.
- `core.untrackedCache=true` enables the cache and assumes filesystem modification times work correctly.
- `fetch.writeCommitGraph=true` writes a commit graph after fetches that download pack files; occasional writes can take longer when graph files merge.
- `feature.experimental=true` changes defaults that may be added or removed across releases and may interact unexpectedly.

## Source and method

-   Source: Installed Git; the [Git 2.56.0 config reference](https://git-scm.com/docs/git-config) and [diff reference](https://git-scm.com/docs/git-diff); [Julia Evans's popular Git config options](https://jvns.ca/blog/2024/02/16/popular-git-config-options/); [GitButler's Git core developer config article](https://blog.gitbutler.com/how-git-core-devs-configure-git).
-   Method: Ran `git --version` and `git help --config | wc -l`; inspected the current Git config manual entries for candidate settings.
-   Inputs and environment: Linux workspace, Git 2.56.0, observed 2026-10-23.
-   Revision or observed at: Git 2.56.0; manual page last updated 2026-09-28.
-   Raw result: [The exact config variable list](git-config-names-2.56.0.txt) from `git help --config`.

## Limitations

This observation establishes the installed version, listed variable count, and the cited manual descriptions. It does not prove behavior on other Git versions or platforms. The manual describes behavior; it does not establish that each option is useful for gir users.

## Relationships

### `../TASK.md#git-config-scope`

-   Relationship: `SUPPORTS`
-   Reason: The exact version and config variable count match the Task's premise.
-   Scope: The current Linux environment only.

### `../TASK.md#git-config-catalog`

-   Relationship: `SUPPORTS`
-   Reason: The official manual provides behavior and limitation details needed to evaluate catalog candidates.
-   Scope: Candidate descriptions must still be checked against the specified version and intended config scope.

## Reproduction

Run `git --version` and `git help --config | wc -l`. Open the Git 2.56.0 config reference and search for each setting listed above.
