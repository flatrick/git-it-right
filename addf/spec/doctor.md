# gir doctor

## Subspecifications

`NONE`.

## Invocation and result

<a id="req-doctor-fix-flag"></a>
**fix-flag.** `gir doctor` SHALL accept `--fix` and SHALL apply available fixes when it is present.

<a id="req-doctor-outside-repo"></a>
**outside-repo.** Outside a Git repository, `gir doctor` SHALL print `gir: not inside a git repository` to stderr and exit `2`.

<a id="req-doctor-report-lines"></a>
**report-lines.** `gir doctor` SHALL print each non-OK check to stdout as a line beginning `warn ` or `info `, followed by its check ID, `: `, and its message; `gir doctor --fix` SHALL use `fixed` for a check it fixes.

<a id="req-doctor-summary"></a>
**summary.** `gir doctor` SHALL finish stdout with `gir doctor: `, the OK count, ` ok, `, the remaining warning count, ` warnings`, and `   more: gir explain doctor`; without `--fix`, it SHALL add `, N fixable with: gir doctor --fix` when `N` checks have fixes.

<a id="req-doctor-warning-exit"></a>
**warning-exit.** `gir doctor` SHALL exit `1` when at least one warning remains and `0` when none remains; `info` checks SHALL NOT make the exit code `1`.

<a id="req-doctor-fix-converges"></a>
**fix-converges.** After `gir init` and `gir doctor --fix` in a repository whose only warnings have available fixes, another `gir doctor` SHALL print no `warn ` lines and exit `0`.

## Hooks and Git settings

<a id="req-doctor-missing-hooks"></a>
**missing-hooks.** If the repository root has no `.githooks` directory, `gir doctor` SHALL print `warn  hooks: no .githooks/; run: gir init`; `--fix` SHALL NOT create the directory.

<a id="req-doctor-hooks-path"></a>
**hooks-path.** If `.githooks` exists and `core.hooksPath` is not `.githooks`, `gir doctor` SHALL warn that hooks are inactive; `--fix` SHALL set local `core.hooksPath` to `.githooks`.

<a id="req-doctor-recommended-config"></a>
**recommended-config.** For each unset setting in `pull.ff=only`, `fetch.prune=true`, `push.autoSetupRemote=true`, `rerere.enabled=true`, `merge.conflictStyle=zdiff3`, `diff.algorithm=histogram`, `rebase.autoStash=true`, and `rebase.updateRefs=true`, `gir doctor` SHALL print a `warn ` line and `--fix` SHALL set that value in local Git config.

<a id="req-doctor-autosquash"></a>
**autosquash.** When `rebase.autoSquash` is unset, `gir doctor` SHALL print a warning naming it and `--fix` SHALL set local `rebase.autoSquash=true`.

<a id="req-doctor-existing-config"></a>
**existing-config.** For any recommended setting already set to a different nonempty value, `gir doctor` SHALL print an `info ` line naming that value and its recommendation; `--fix` SHALL NOT change it.

<a id="req-doctor-pull-rebase"></a>
**pull-rebase.** When `pull.rebase` has a nonempty value, `gir doctor` SHALL NOT warn about or change `pull.ff`.

<a id="req-doctor-windows-longpaths"></a>
**windows-longpaths.** On Windows, when `core.longpaths` is unset, `gir doctor` SHALL warn about paths over `260` characters; `--fix` SHALL set local `core.longpaths=true`.

<a id="req-doctor-user-identity"></a>
**user-identity.** When `user.name` or `user.email` is unset, `gir doctor` SHALL print a `warn ` line for each missing key suggesting `git config --global KEY` and `--fix` SHALL NOT set either key.

<a id="req-doctor-default-branch"></a>
**default-branch.** When `init.defaultBranch` is unset, `gir doctor` SHALL print `info  init.defaultBranch: unset; suggest: git config --global init.defaultBranch main` and `--fix` SHALL NOT set it.

## Repository files

<a id="req-doctor-gitattributes-missing"></a>
**gitattributes-missing.** When `.gitattributes` does not exist, `gir doctor` SHALL print `warn  .gitattributes: missing`; `--fix` SHALL create `.gitattributes`.

<a id="req-doctor-gitattributes-unreadable"></a>
**gitattributes-unreadable.** When `.gitattributes` exists but cannot be read, `gir doctor` SHALL print `warn  .gitattributes: cannot read: ` followed by the OS error; `--fix` SHALL NOT write the file.

<a id="req-doctor-gitattributes-content"></a>
**gitattributes-content.** The `.gitattributes` created by `gir doctor --fix` SHALL contain `* text=auto eol=lf`, CRLF rules for `*.cmd`, `*.bat`, and `*.sln`, and `binary` rules for `*.png`, `*.jpg`, `*.jpeg`, `*.gif`, `*.ico`, `*.webp`, `*.pdf`, `*.zip`, `*.gz`, `*.7z`, `*.woff`, `*.woff2`, `*.ttf`, `*.exe`, `*.dll`, `*.so`, and `*.dylib`.

<a id="req-doctor-gitattributes-autocrlf"></a>
**gitattributes-autocrlf.** When `.gitattributes` does not exist and `core.autocrlf` is set to a value other than `false`, `gir doctor` SHALL append `and core.autocrlf=VALUE, so line endings depend on each clone` to its missing-file warning.

<a id="req-doctor-gitattributes-rule"></a>
**gitattributes-rule.** An existing `.gitattributes`, valid UTF-8 or not, without a line whose first whitespace-separated word is `*` and one of whose remaining words is exactly `text=auto` SHALL produce an `info  .gitattributes:` line containing `no ` and `* text=auto`; `--fix` SHALL NOT replace that file.

<a id="req-doctor-editorconfig"></a>
**editorconfig.** When `.editorconfig` does not exist, `gir doctor` SHALL print `info  .editorconfig: missing`; `--fix` SHALL create it with `root = true`; a `[*]` section setting `charset = utf-8`, `end_of_line = lf`, `insert_final_newline = true`, `trim_trailing_whitespace = true`, `indent_style = space` and `indent_size = 4`; `trim_trailing_whitespace = false` for Markdown; `indent_size = 2` for YAML, JSON and TOML; `end_of_line = crlf` for `*.cmd`, `*.bat` and `*.sln`; and `indent_style = tab` for `Makefile`.

<a id="req-doctor-girconfig"></a>
**girconfig.** If `.girconfig` has an invalid `gir` setting, or the types file it uses is invalid, `gir doctor` SHALL print a `warn  .girconfig:` line describing the error, and one for each configuration warning; `--fix` SHALL NOT edit the file.

<a id="req-doctor-ignore-rules"></a>
**ignore-rules.** When a relevant untracked probe is not ignored, `gir doctor` SHALL print `warn  .gitignore: does not ignore: ` followed by the missing patterns; `--fix` SHALL append those patterns to `.gitignore`, adding a newline before them if needed and keeping every existing byte, including bytes that are not valid UTF-8.

<a id="req-doctor-ignore-patterns"></a>
**ignore-patterns.** `gir doctor` SHALL check `.DS_Store`, `Thumbs.db`, and `.env` in every repository, and SHALL check stack patterns only when the matching marker is present; `.claude/settings.local.json` and `.scratch/` SHALL be checked when their parent directories exist.

<a id="req-doctor-tracked-ignore-probes"></a>
**tracked-ignore-probes.** `gir doctor` SHALL NOT warn that a probe needs ignoring when that exact probe is tracked by Git.

<a id="req-doctor-git-cliff"></a>
**git-cliff.** If `git-cliff --version` cannot run successfully from `PATH`, `gir doctor` SHALL print `info  git-cliff: not installed (optional: changelog + next version from commits)`.

## Git index

<a id="req-doctor-case-collision"></a>
**case-collision.** For indexed paths that differ only in case, `gir doctor` SHALL print `warn  case-collision: paths differ only in case` with the paths; `--fix` SHALL NOT rename them. The stages of one unmerged path SHALL NOT count as a collision.

<a id="req-doctor-windows-names"></a>
**windows-names.** For indexed paths with Windows-reserved device names, trailing dots or spaces, the characters `<`, `>`, `:`, `"`, `\`, `|`, `?`, or `*`, a control character (a byte from 1 to 31), or bytes that are not valid UTF-8, `gir doctor` SHALL print `warn  windows-names: cannot be checked out on Windows` with the paths; `--fix` SHALL NOT rename them.

<a id="req-doctor-exec-bit"></a>
**exec-bit.** For an indexed `100644` path ending in `.sh` or starting with `.githooks/` and without unresolved conflict entries, `gir doctor` SHALL print `warn  exec-bit: scripts not executable in git` with the path; `--fix` SHALL set those index entries to mode `100755`, keeping their staged content and skip-worktree state, without needing their files in the working tree. On systems with an executable bit, `--fix` SHALL also make each of those files that exists in the working tree as a regular file executable, without changing its content, and SHALL NOT create a missing file or change the target of a symbolic link; on Windows it SHALL NOT change the working tree.

<a id="req-doctor-path-display"></a>
**path-display.** `gir doctor` SHALL read indexed and listed paths as Git stores them, and its report lines SHALL show each path as stored, except that a path with a control character or bytes that are not valid UTF-8 SHALL be shown C-quoted as Git quotes it.

<a id="req-doctor-index-clean"></a>
**index-clean.** When no indexed case collision, Windows-unsafe name, or missing executable bit exists, `gir doctor` SHALL count the index check as OK and SHALL print no index warning.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
