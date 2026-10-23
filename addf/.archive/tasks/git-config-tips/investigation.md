# Git config option survey

## Screening method

The survey screened the 1,012 variable names printed by Git 2.56.0's `git help --config` against one question: does this setting offer a broadly useful improvement to a developer's everyday Git experience? Candidate descriptions were checked against the current Git reference. The user-facing catalog should prioritize settings that only change output or editor context. Settings that change Git operations remain eligible only when the effect and useful situation can be stated plainly.

The survey excludes server administration, transport and credential setup, email workflows, external-tool wiring, per-remote or per-branch configuration, repository-format knobs, low-level pack and index tuning, obsolete settings, and options that are already Git defaults. Settings already checked by `gir doctor` remain useful but need a clear reason to repeat them in `gir tips`.

## Presentation and editor settings

| Setting | Effect | Useful when | Survey disposition |
|---|---|---|---|
| `diff.colorMoved=plain` | Colors lines Git detects as moved in diffs. | You want to distinguish moved code from new or deleted lines. | Include. Add to doctor as requested. |
| `branch.sort=-committerdate` | Lists branches by newest commit date first. | A repository has many branches and recent activity is the best way to find one. | Include. |
| `tag.sort=version:refname` | Sorts version-like tag names numerically by their components. | You often inspect release tags such as `v1.9` and `v1.10`. | Include. |
| `column.ui=auto` | Formats supported listings in columns only for terminal output. | You want compact branch, tag, and status listings without changing piped output. | Include. |
| `diff.mnemonicPrefix=true` | Uses prefixes such as `i/` and `w/` to identify diff sides. | You often compare the index with the working tree. | Include if the final list remains concise. |
| `diff.context=10` | Shows ten context lines around diff hunks instead of the default three. | You want more surrounding code while reviewing changes. | Include as a display preference; state that it makes diffs longer. |
| `commit.verbose=true` | Shows the diff in the editor while writing a commit message; Git removes that context from the committed message. | You want to review the staged changes while describing a commit. | Include. Explain that it changes editor context only. |
| `log.date=iso` | Shows log dates in a numeric ISO-like format. | You want dates that are easier to compare or copy. | Optional. It is a display preference with less impact than the other candidates. |
| `blame.ignoreRevsFile=.git-blame-ignore-revs` | Omits listed commits from blame attribution. | A repository has mechanical formatting commits listed in a tracked ignore file. | Candidate for repository-level documentation; requires a coordinated file, so not a global one-line tip. |
| `diff.algorithm=histogram` | Changes the diff algorithm, not repository contents. | Reordered functions or repeated lines produce confusing diffs. | Already recommended by doctor; avoid duplicating in tips unless the catalog also explains doctor-managed settings. |

## Workflow and history settings

| Setting | Effect | Useful when | Survey disposition |
|---|---|---|---|
| `help.autoCorrect=prompt` | Prompts before running a uniquely suggested command after a typo. | You want typo suggestions with a confirmation step. | Include; state that accepting the prompt runs a command. |
| `rebase.missingCommitsCheck=error` | Stops an interactive rebase if a commit disappears from its todo list. An explicit `drop` still works. | You edit rebase todo lists and want protection against accidental line deletion. | Include; it intentionally blocks some rebases. |
| `pull.rebase=true` | Rebases local commits after fetching instead of merging them into the upstream branch. | Your team prefers linear history and you rebase private local commits. | Candidate. Must explain conflict and history implications, and that merge commits are not created by pull. |
| `user.useConfigOnly=true` | Prevents Git from guessing author and committer identity values. | You use different email addresses across repositories and want a missing identity to fail before a commit. | Include; state that commits fail until a repository or higher-scope identity is configured. |
| `rerere.autoUpdate=true` | Stages a previously recorded conflict resolution automatically. | You repeatedly resolve the same conflicts and trust the recorded resolution. | Candidate with a strong staging caveat; possibly too surprising for the initial catalog. |
| `rebase.autoSquash=true` | Folds `fixup!` and `squash!` commits during interactive rebase by default. | You use fixup commits to clean up a branch. | Already recommended by doctor; avoid duplicating unless doctor-managed settings are included. |
| `rebase.autoStash=true` | Stashes dirty work before rebase and reapplies it afterward. Reapplying can conflict. | You intentionally rebase with unrelated uncommitted changes. | Already recommended by doctor; caveat makes it a behavior-changing recommendation. |
| `rebase.updateRefs=true` | Moves other branch refs that point into the rebased range. | You maintain stacked branches and want descendants updated together. | Already recommended by doctor; explain that multiple local refs move if included. |

## Fetch and push settings

| Setting | Effect | Useful when | Survey disposition |
|---|---|---|---|
| `fetch.all=true` | A bare `git fetch` attempts to update every configured remote. | You regularly use multiple remotes and want all of them refreshed. | Include only with the all-remotes effect and possible extra time stated. |
| `fetch.pruneTags=true` | With pruning active, removes local tags absent from the fetch refspec. | A remote is authoritative for the tags in a repository. | Candidate, but warn that local-only tags may be deleted. |
| `push.followTags=true` | A push also sends eligible annotated tags reachable from the pushed commits. | You publish release tags with the commits they describe. | Candidate, but state that tags may be published automatically. |
| `fetch.prune=true` | Removes remote-tracking refs that disappeared from the remote. | You want local remote branch listings to match the server. | Already recommended by doctor; explain deletion if included. |
| `push.autoSetupRemote=true` | First push of a branch sets its upstream. | You use same-name local and remote branches and want later pull/push commands to use that tracking branch. | Already recommended by doctor; mention possible same-name collision if included. |

## Integrity and performance settings

| Setting | Effect | Useful when | Survey disposition |
|---|---|---|---|
| `fetch.fsckObjects=true` | Checks fetched objects and rejects malformed objects or broken links. A failed fetch can leave unreferenced objects. | You want Git to validate incoming objects, accepting that a repository with legacy object problems may fail to fetch. | Candidate for an advanced integrity section; not a universal safe default. |
| `transfer.fsckObjects=true` | Supplies the default fsck check for fetch and receive when their specific settings are unset. | You administer repositories that receive pushes as well as fetch from remotes. | Exclude from a general personal-settings list; receiving-side configuration belongs on the receiving repository. |
| `core.fsmonitor=true` | Starts Git's built-in filesystem monitor to speed commands such as status in large repositories. Built-in support is currently Windows and macOS; old Git versions may misread boolean values. | You use a large repository, a supported platform, and consistent modern Git versions across clients. | Situational tip only if platform/version caveats fit. |
| `core.untrackedCache=true` | Caches information to speed checks for untracked files and assumes filesystem mtimes are reliable. | Large repositories have slow status scans and the filesystem reports mtimes correctly. | Situational tip; explain the mtime assumption. |
| `fetch.writeCommitGraph=true` | Writes commit-graph data after fetches that download packs. A graph merge can make some fetches take longer. | Large repositories benefit from faster graph and ancestry operations. | Situational tip; likely exclude from the first concise catalog. |

## Exclusions and defaults

- `feature.experimental=true` changes a release-dependent group of defaults and may introduce unintended interactions. It is for users who want to test new behavior, not a general DX recommendation.
- `fetch.all`, `fetch.pruneTags`, and `push.followTags` are not unsafe in every workflow, but they change which refs are fetched, removed, or published. Include them only if the effect and the matching use case appear beside the command.
- `push.default=simple`, `diff.renames=true`, and `color.ui=auto` are already Git defaults in the current manual; recommending them adds no behavior for most users.
- `init.defaultBranch=main` is a personal naming choice already suggested by doctor. It is not a safe universal default for every team.
- Pager, editor, credential helper, signing key, aliases, URL rewrites, and includeIf settings depend on a developer's local tools or identity.
- Submodule recursion and submodule summaries help only repositories that use submodules.
- Server, email, protocol, HTTP transport, GUI, filesystem compatibility, and low-level performance options are outside a beginner-facing tips list unless a later user need selects them.

## Candidate grouping

The survey supports one read-only catalog with two top-level effect groups: output/editor context first, then settings that change workflow, refs, or performance. Within the second group, short labels such as `workflow`, `fetch/push`, and `large repositories` can show when each setting fits without making developers choose a profile before seeing the options.
