# gir init

## Subspecifications

`NONE`.

## Invocation and status

<a id="req-init-accept-force"></a>
**accept-force.** `gir init` SHALL accept the optional `--force` flag.

<a id="req-init-accept-optional"></a>
**accept-optional.** `gir init` SHALL accept the optional `--optional` flag.

<a id="req-init-require-repository"></a>
**require-repository.** Outside a Git repository, `gir init` SHALL exit `2` and print `gir: not inside a git repository` to stderr.

<a id="req-init-root-paths"></a>
**root-paths.** From any directory inside a Git repository, `gir init` SHALL write its files at that repository's top level.

<a id="req-init-success-status"></a>
**success-status.** When setup succeeds without differing existing files, `gir init` SHALL exit `0`.

<a id="req-init-success-output"></a>
**success-output.** Whenever `gir init` completes setup, including when it exits `1` for kept files, it SHALL print `gir: next: gir doctor` as its last stderr line and print nothing to stdout.

## Generated files

<a id="req-init-file-kinds"></a>
**file-kinds.** The required files are `.githooks/commit-msg`, `.githooks/pre-push` and `.girconfig`.
The optional files are `cliff.toml` and `GIT-IT-RIGHT.md`.

<a id="req-init-adopted"></a>
**adopted.** A repository is adopted when `.girconfig` is in its index.

<a id="req-init-required-files"></a>
**required-files.** `gir init` SHALL create each missing required file, in any repository.

<a id="req-init-optional-files"></a>
**optional-files.** `gir init` SHALL create a missing optional file only when the repository is not adopted, or `--optional` or `--force` is given.
Otherwise it SHALL leave the file absent and print nothing about it.

<a id="req-init-optional-no-overwrite"></a>
**optional-no-overwrite.** `--optional` SHALL NOT change how existing files are treated; the existing-file requirements below apply.

<a id="req-init-hook-files"></a>
**hook-files.** `gir init` SHALL create `.githooks/commit-msg` and `.githooks/pre-push` with a `#!/bin/sh` header and executable permissions on Unix.

<a id="req-init-config-file"></a>
**config-file.** `gir init` SHALL create `.girconfig` with `[gir]` entries `types = feat fix docs style refactor perf test build ci chore revert` and `subjectMax = 72`, a `[gir "alias"]` section with `feature = feat` and `bugfix = fix`, and commented examples for `scopes`, `scopeRequired`, `descCase`, and `hookMissing`.
When git config already names a types file (see the config module), the `types` entry SHALL be written commented out.

<a id="req-init-cliff-file"></a>
**cliff-file.** When [optional-files](#req-init-optional-files) permits, `gir init` SHALL create `cliff.toml` with `[changelog]` and `[git]` sections, `conventional_commits = true`, `filter_unconventional = true`, `protect_breaking_commits = true`, and `sort_commits = "oldest"`.

<a id="req-init-git-it-right-file"></a>
**git-it-right-file.** When [optional-files](#req-init-optional-files) permits, `gir init` SHALL create `GIT-IT-RIGHT.md` with the same content regardless of `.girconfig`.
It SHALL explain what gir is and why the repository uses it; the per-clone contributor setup (install gir, run `gir init`); `gir explain types` and `gir explain <rule>`; checking a contributor's branch with `gir lint --range` without CI and in CI; that `cliff.toml` is an optional git-cliff configuration with its two commands; and that both optional files may be deleted and are restored by `gir init --optional`.

<a id="req-init-cliff-format"></a>
**cliff-format.** Generated `cliff.toml` SHALL format versioned and unreleased headings, group commits, show scopes and breaking markers, and set `trim = true`.

<a id="req-init-cliff-parsers"></a>
**cliff-parsers.** Generated `cliff.toml` SHALL skip messages beginning `fixup!`, `squash!`, or `amend!` and SHALL include a commit parser for each type in the effective `.girconfig` `gir.types` list.

<a id="req-init-cliff-groups"></a>
**cliff-groups.** Generated `cliff.toml` SHALL use the built-in titles `Features`, `Bug Fixes`, `Documentation`, `Styling`, `Refactor`, `Performance`, `Testing`, `Build`, `CI`, `Miscellaneous`, and `Reverts` for the corresponding default types; other types SHALL use their name with the first character capitalized.

## Existing files

<a id="req-init-identical-files"></a>
**identical-files.** `gir init` SHALL leave an existing file unchanged when its content matches the template, treating CRLF line endings as equivalent to LF.

<a id="req-init-clean-rerun-output"></a>
**clean-rerun-output.** On a rerun with files matching their templates, `gir init` SHALL NOT print `gir: wrote` to stderr.

<a id="req-init-keep-edits"></a>
**keep-edits.** Without `--force`, `gir init` SHALL leave each existing file with different content unchanged, including a file that is not valid UTF-8.

<a id="req-init-kept-output"></a>
**kept-output.** For each differing file kept without `--force`, `gir init` SHALL print `gir: kept ` followed by its path and `(differs from the template; --force overwrites)` to stderr.

<a id="req-init-conflict-status"></a>
**conflict-status.** If one or more differing files are kept, `gir init` SHALL exit `1` after processing the remaining files and Git setup.

<a id="req-init-force-overwrite"></a>
**force-overwrite.** With `--force`, `gir init` SHALL replace each existing file whose content differs from its template, print `gir: wrote ` followed by its path to stderr, and exit `0` if setup otherwise succeeds.

<a id="req-init-config-before-cliff"></a>
**config-before-cliff.** `gir init` SHALL generate `cliff.toml` from the effective `.girconfig` after deciding whether to create, keep, or replace that file, so a kept custom `gir.types` list controls the generated parsers.

## Git setup and diagnostics

<a id="req-init-stage-hooks"></a>
**stage-hooks.** `gir init` SHALL stage with mode `100755` each hook file it wrote in this run, and each hook file whose index entry is missing or not `100755`, and print `gir: staged ` followed by those paths and ` as executable` on stderr.

<a id="req-init-no-restage"></a>
**no-restage.** `gir init` SHALL NOT stage a hook file it did not write and whose index entry is already `100755`, so a rerun leaves the index unchanged.

<a id="req-init-local-hook-path"></a>
**local-hook-path.** Unless the effective `core.hooksPath` is already `.githooks`, `gir init` SHALL run `git config --local core.hooksPath .githooks` and print `gir: set core.hooksPath=.githooks (this clone)` to stderr.

<a id="req-init-no-repeat-hook-path"></a>
**no-repeat-hook-path.** When the effective `core.hooksPath` is `.githooks`, `gir init` SHALL NOT print `gir: set core.hooksPath=.githooks (this clone)`.

<a id="req-init-write-diagnostics"></a>
**write-diagnostics.** For every created or replaced file, `gir init` SHALL print `gir: wrote ` followed by its repository-relative path to stderr.

<a id="req-init-config-error"></a>
**config-error.** If the effective `.girconfig` has an unknown `gir.` key or a value that the config module's invalid-value requirement rejects, `gir init` SHALL exit `2` and print an error beginning `gir: .girconfig:` to stderr before generating `cliff.toml` or running Git setup.
An invalid types file SHALL do the same, with the error the config module's types-file-invalid requirement names.

<a id="req-init-git-error"></a>
**git-error.** If staging the hooks or setting the local hook path fails, `gir init` SHALL exit `2` and print `gir: ` followed by Git's error to stderr.

## Generated hook behavior

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
