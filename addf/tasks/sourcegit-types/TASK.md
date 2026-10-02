# TASK — `gir reads SourceGit's Conventional Commit type definition file`

## Resume

**Contract version:** `2`

**State:** `COMPLETED`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN -> COMPLETED`

**Resume at:** `NONE`

**Open obligations:** `NONE`

## Owned artifacts

-   `ledger.md` - the `sourcegit-types-20261002` thread, taken whole: the operator's answers that shaped this Task.
-   `probe_origin.py` - Probe: what `git config --show-origin` reports for `gir.typesFile` set globally, through an include, locally, with `-c`, empty, and from a subdirectory and a linked worktree.
-   `logs/probe-origin-20261002-1825.log` - Evidence: `probe_origin.py`, first version.
-   `logs/probe-origin-20261002-1830.log` - Evidence: `probe_origin.py` with the subdirectory and worktree cases.
-   `logs/probe-origin-20261002-1850.log` - Evidence: `probe_origin.py` with the `--type=path` cases.
-   `probe_json_size.py` - Probe: release-binary size and dependencies `serde_json` adds to a minimal crate.
-   `logs/probe-json-size-20261002-1840.log` - Evidence: `probe_json_size.py` output.
-   `acceptance.py` - Probe and acceptance: the JSON example from issue #4, unchanged, through a global `~/` `gir.typesFile`.
-   `logs/acceptance-20261002-1940.log` - Evidence: `acceptance.py` on the build of `a7da210`'s tree.
-   `logs/test-full-20261002-1936.log` - Evidence: `cargo test --no-fail-fast` on that tree.
-   `logs/test-perf-20261002-1936.log` - Evidence: the hook latency test with its timing line.
-   `logs/clippy-20261002-1935.log` - Evidence: `cargo clippy --all-targets -- -D warnings` on that tree.
-   `logs/ci-capsule-failed-37039199722-20261002.log` - Evidence: the failed `capsule` jobs of hosted run 37039199722 at `b67d564`.
-   `logs/capsule-20261002-2010.log` - Evidence: `check-capsule` locally after the index fix.
-   `logs/ci-run-37039567751-20261002.log` - Evidence: hosted run 37039567751 at `4f5cf6a`, all jobs passing.

## Specification impact

- Current contract: `framework:spec/config.md#req-config-location`, `framework:spec/config.md#req-config-types`, `framework:spec/config.md#req-config-aliases`, `framework:spec/config.md#req-config-malformed`, `framework:spec/config.md#req-config-explain-tolerates-invalid`, `framework:spec/explain.md#req-explain-types-config`, `framework:spec/explain.md#req-explain-types-summaries`, `framework:spec/init.md#req-init-cliff-parsers`, `framework:spec/init.md#req-init-config-error`, `framework:spec/doctor.md#req-doctor-girconfig`.
- Proposed delta: a new `gir.typesFile` setting, read from git config and `.girconfig`, naming a SourceGit-format JSON file whose entries become the complete list of allowed types, with their names and descriptions; an invalid file makes gir refuse with the file's path and the reason. Exact wording settled in `DECIDE`.
- Terminal publication: `framework:spec/config.md#req-config-location`, `framework:spec/config.md#req-config-aliases`, `framework:spec/config.md#req-config-types-file`, `framework:spec/config.md#req-config-types-file-location`, `framework:spec/config.md#req-config-types-file-invalid`, `framework:spec/config.md#req-config-types-file-overrides-types`, `framework:spec/config.md#req-config-explain-tolerates-invalid`, `framework:spec/explain.md#req-explain-types-config`, `framework:spec/explain.md#req-explain-types-summaries`, `framework:spec/explain.md#req-explain-types-aliases`, `framework:spec/init.md#req-init-config-file`, `framework:spec/init.md#req-init-config-error`, `framework:spec/doctor.md#req-doctor-girconfig`.

## Define

### Objective

A repository, or a user for all their repositories, can point gir at the JSON file SourceGit uses to define Conventional Commit types, and gir then allows exactly those types, so gir, git on the CLI and SourceGit agree on them (GitHub issue #4).

### Success criteria

<a id="c1-setting"></a>
#### `c1-setting`

-   Claim: `gir.typesFile` is honored when set in `.girconfig` and when set only in git config (global or repository); when both set it, the `.girconfig` value is used; a relative value resolves against the directory of the file that set it; an empty value in `.girconfig` turns off the git config value.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: a user cannot share one file across repositories, or a repository cannot override it.
-   Basis: [Verification](#verification-c1-setting).

<a id="c2-replaces"></a>
#### `c2-replaces`

-   Claim: With a valid types file, `gir lint` accepts exactly the file's `Type` values and rejects every other type, including defaults the file leaves out and aliases to them; `gir explain types` lists the file's types with their `Description`; `gir init` generates `cliff.toml` parsers for the file's types and writes a new `.girconfig`'s `types` line commented out.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: gir and SourceGit disagree on allowed types.
-   Basis: [Verification](#verification-c2-replaces).

<a id="c3-invalid-refuses"></a>
#### `c3-invalid-refuses`

-   Claim: A configured types file that is missing, unreadable, not valid JSON, not an array, has an entry missing `Type`, `Name` or `Description` or with a non-string value for one, an empty or malformed `Type`, or a duplicate `Type`, makes `gir lint`, `gir hook` and `gir init` exit `2` before doing anything, with a message naming the file's path, where the setting came from, and the reason (with line and column for a JSON syntax error); `gir explain` instead falls back to the defaults and `gir doctor` prints the error as a `warn` line.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: gir acts on a types list the user did not intend, or the user cannot find what to fix.
-   Basis: [Verification](#verification-c3-invalid-refuses).

<a id="c4-ignored-fields"></a>
#### `c4-ignored-fields`

-   Claim: `PrefillShortDesc` and unknown fields are accepted and ignored; `Name` is parsed and kept on each type in gir's configuration though nothing displays it yet.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: a SourceGit file with extra fields breaks gir, or the future type picker has no names.
-   Basis: [Verification](#verification-c4-ignored-fields).

<a id="c5-both-set-warns"></a>
#### `c5-both-set-warns`

-   Claim: When `gir.types` is set in `.girconfig` and a types file is configured, the file's types are used and gir prints a warning naming both to stderr.
-   State: `VERIFIED`
-   Scope: this branch.
-   Consequence if false: a user silently gets a different type list from the one they wrote.
-   Basis: [Verification](#verification-c5-both-set-warns).

<a id="c6-no-regression"></a>
#### `c6-no-regression`

-   Claim: Without `gir.typesFile`, gir behaves as before: `cargo test` and `cargo clippy --all-targets -- -D warnings` pass.
-   State: `VERIFIED`
-   Scope: this branch, Linux; Windows and macOS through CI.
-   Consequence if false: existing users break.
-   Basis: [Verification](#verification-c6-no-regression).

### Constraints

-   Isolate: the operator chose a new worktree and branch. Worktree `.claude/worktrees/sourcegit-types`, branch `feat/sourcegit-types`, from `8692ce2`.
-   Code behaves the same on Windows, Linux and macOS (`rules/os-agnostic-code.md`), including path handling for `gir.typesFile`.
-   Aliases stay in gir's own configuration; the JSON format has none.
    An alias whose target the file leaves out stops applying (ledger A10).
-   A relative `gir.typesFile` resolves against the directory of the file that set it; `~/` expands to the home directory; an empty value in `.girconfig` turns off a value from git config (ledger A11).
-   `gir explain` falls back to the defaults and `gir doctor` prints a `warn` line on an invalid types file (ledger A12).
-   With a types file configured, `gir init` writes the `types` line of a new `.girconfig` commented out (ledger A13).
-   A valid `Type` is nonempty and only ASCII letters, digits and `-` (ledger A14).
-   No other active Task; no overlap.

### Material empirical premises

<a id="p1-format"></a>
#### `p1-format`

-   Claim: SourceGit's type definition file is a JSON array of objects with string fields `Name`, `Type`, `Description` and `PrefillShortDesc`, as in the example in issue #4.
-   State: `VERIFIED`
-   Scope: SourceGit `master` at `a4ae633c51c922150ea113913c682a8213308560`, read 2026-10-02.
-   Consequence if false: gir rejects files SourceGit writes, or accepts ones it does not.
-   Basis: [Verification](#verification-p1-format).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective and success criteria are right, with the answers recorded in `ledger.md` (A15).

## Understand

### Relevant context

`src/config.rs` reads only `.girconfig`, through `git config --file`; types are a `Vec<String>`, with changelog group titles hardcoded in `DEFAULT_TYPES`.
`gir explain types` takes descriptions from `CHEATSHEET.md`.
No JSON parser is a dependency yet.

Config consumers: `src/main.rs` (`lint` and `hook` propagate `Config::load()` errors, `explain` uses `unwrap_or_default`), `fixup` (through `lint`'s rules), `src/cmd/init.rs` (`load_from` on the written `.girconfig`, then `cliff.toml`), `src/cmd/doctor.rs` (`load_from` error becomes a `warn` line).
`Config::load_from` takes a `.girconfig` path; `gir.typesFile` from git config needs a second read that `load_from` does not do today.

SourceGit (see `p1-format`): `ConventionalCommitType` has string properties `Name`, `Type`, `Description`, `PrefillShortDesc`, read with System.Text.Json defaults, so names are case-sensitive, unknown fields are ignored, and comments and trailing commas are syntax errors.
SourceGit keeps the file's path in its own per-repository settings (`ConventionalTypesOverride`), not in git config, and on any read error silently uses its built-in types.

`git config --show-origin --show-scope -z --get gir.typesFile` prints scope, origin and value, NUL-separated (`logs/probe-origin-20261002-1825.log`, `logs/probe-origin-20261002-1830.log`, git 2.56.0):

-   A global or included value's origin is `file:` and the absolute path of the file that set it, including a file reached through `include.path`.
-   A repository value's origin is `file:.git/config`, relative to the top level even when run from a subdirectory; from a linked worktree it is absolute.
-   A `git -c` value's origin is `command line:`, with no file.
-   `--type=path` expands `~/`; without it the value is returned verbatim.
-   An empty value is returned as set, distinct from unset (exit `1`).

### Assumptions

-   NONE.

### Open questions

-   Which JSON parser: a dependency (`serde_json`) or hand-written. Resolved in Investigate.
-   What a relative value set with `git -c` resolves against, since it has no file. Resolved in Investigate.

### Deferred verification

-   NONE.

### UNDERSTAND gate

`ESTABLISHED`: `p1-format` is verified, every consumer of `Config` is identified, and git's origin reporting, which the relative-path rule depends on, is observed.

## Investigate

-   JSON parser: resolved. `serde_json` adds `itoa`, `memchr`, `serde_core` and `zmij`, and 38,224 bytes to a stripped LTO release build of a minimal crate; its errors carry line and column (`logs/probe-json-size-20261002-1840.log`). The operator chose it, without `serde_derive` (ledger A16).
-   `git -c` values: resolved. A relative one resolves against the repository root (ledger A17).
-   `--type=path`: resolved. It expands `~/` for values from git config and from `git config --file`, and returns an empty value as empty (`logs/probe-origin-20261002-1850.log`).
-   `gir fixup` does not load `Config` today (`src/main.rs`, `src/cmd/fixup.rs`), so an invalid configuration never stopped it. `c3-invalid-refuses` listed it by mistake; it is narrowed to `lint`, `hook` and `init`, matching where `.girconfig` errors already refuse.
-   Aliases: `fix_header` in `src/cc/mod.rs` applies an alias whatever its target, then reports the target as not allowed. Ledger A10 requires the alias to stop applying instead.
-   Hook latency: one more `git config` call per hook run; `tests/perf.rs` budgets 750 ms median, so the extra spawn is checked there, not assumed.

### INVESTIGATE gate

`ESTABLISHED`: every open question has a disposition above; nothing decision-relevant remains unprobed.

## Decide

**Data.** `Config` keeps `types: Vec<String>` as the allowed list every consumer already reads, and gains `types_file: Option<TypesFile>`, where `TypesFile` holds the resolved path, where the setting came from, and `defs: Vec<TypeDef>`, each `TypeDef` holding `ty`, `name` and `description`. With a types file, `types` is the file's `Type` values in file order. `Config` also gains `warnings: Vec<String>`.

**Loading.** `Config::load_at(root)` replaces `load_from(path)` for its three callers and the tests; `load()` calls it with the top level.
1. Read `.girconfig` as today, also accepting `gir.typesFile`; when present, read it again with `--type=path`, relative to `root`.
2. Otherwise, `git config --show-origin -z --type=path --get gir.typesFile`, from `root`. A `file:` origin makes a relative value relative to that file's directory (a relative origin is relative to `root`); any other origin, such as `command line:`, makes it relative to `root`.
3. An empty value means no types file. Otherwise read and validate the file; any failure is `Err("types file <path> (gir.typesFile in <origin>): <reason>")`.
4. With a types file, a `gir.types` from `.girconfig` is replaced and a warning is added naming both.

**Validation**, in `config.rs` on a `serde_json::Value`: the top level is an array; each entry is an object whose `Type`, `Name` and `Description` are present strings; `Type` is nonempty ASCII letters, digits and `-`; no `Type` repeats. Errors name the entry by 1-based position and, when known, its `Type`. Other fields are ignored.

**Consumers.**
-   `alias_for` returns an alias only when its target is in `types`, with or without a types file.
-   `gir explain types` heads the list `Allowed types (from <path>):` with a types file and prints each `Description` as the summary; it lists only aliases that apply.
-   `main` prints each warning as `gir: warning: ...` for `lint`, `hook` and `explain`; `init` prints them; `doctor` reports a load error or warning as a `warn  .girconfig:` line, as today.
-   `gir init` writes `# types = ...` in a new `.girconfig` when git config already names a types file, and generates `cliff.toml` from the effective types.

**Rejected.**
-   Changing `types` to `Vec<TypeDef>`: every consumer would change for data only `explain` reads.
-   Printing warnings inside `load`: `doctor` could not turn them into report lines.
-   Expanding `~/` in gir: git's `--type=path` already does it, the same way as for git's own path settings.

**Verification strategy.** Integration tests in `tests/config.rs` and `tests/lint.rs` style, each in a temporary repository with `GIT_CONFIG_GLOBAL` pointing at a temporary file, covering c1 to c5; `cargo test` and `cargo clippy --all-targets -- -D warnings` for c6, with `tests/perf.rs` for hook latency.

**Specification delta** (published at completion): `config.md` gains `types-file`, `types-file-location`, `types-file-invalid` and `types-file-overrides-types`; `config.md#req-config-aliases` adds that an alias whose target is not an allowed type does not apply; `explain.md#req-explain-types-config` and `#req-explain-types-summaries` cover a types file; `init.md#req-init-config-file` covers the commented `types` line.

### DECIDE gate

`ESTABLISHED`: each choice traces to an operator answer or a probe above, and every success criterion has a planned test.

## Implement

`a7da210` implements Decide in `src/config.rs`, `src/main.rs`, `src/explain.rs`, `src/explain.md` (`config` page), `src/cmd/init.rs` and `src/cmd/doctor.rs`, with seven tests in `tests/config.rs`.

Deviations from Decide:

-   `load_from(path)` stays, reading `.girconfig` and a types file it names but not git config; `load_at(root)` adds the git config fallback and is what `load`, `init` and `doctor` use. Reason: `tests/config.rs` calls `load_from` in-process, where reading git config would read the developer's own `~/.gitconfig`.
-   An empty array is invalid (`defines no types`): it would reject every commit. Not asked for by the operator; strictness per ledger A4.
-   A leading UTF-8 byte order mark is skipped, since SourceGit reads the file with `File.ReadAllText`, which skips it.
-   `init` decides the commented `types` line from git config at every run, so a re-run after the global setting changes reports `.girconfig` as differing from the template, as for any other edit.

Checkpoints:

-   Mutation checks: making `alias_for` ignore its target, and resolving git config values against the root, each fail their test (`.scratch/mutation-alias-20261002-1928.log`, `.scratch/mutation-base-20261002-1930.log`, gitignored).
-   `cargo test --no-fail-fast` and `cargo clippy --all-targets -- -D warnings` pass on the committed tree (`logs/test-full-20261002-1936.log`, `logs/clippy-20261002-1935.log`); hook median 6 ms against a 750 ms budget (`logs/test-perf-20261002-1936.log`).
-   `acceptance.py` with the issue's own example file through a global `~/` value: 14 PASS, 0 FAIL (`logs/acceptance-20261002-1940.log`).

### IMPLEMENT gate

`ESTABLISHED`: the change is committed and every success criterion has a test that runs.

## Verify

<a id="verification-p1-format"></a>
### Verification: `p1-format`

- Claim: [p1-format](#p1-format)
- Method: read `src/Models/ConventionalCommitType.cs`, `src/App.JsonCodeGen.cs`, `src/Models/RepositorySettings.cs`, `src/ViewModels/RepositoryConfigure.cs` and `src/Views/CommitMessageToolBox.axaml.cs` in `sourcegit-scm/sourcegit` at `a4ae633` through the GitHub API.
- Evidence considered: the model declares exactly the four `string` properties; it is deserialized as `List<ConventionalCommitType>` with no naming-policy or case-insensitivity option.
- Conclusion: `VERIFIED`; the issue's example matches SourceGit's model.
- Limitations: later SourceGit versions may add fields; c4 ignores unknown fields for that reason.

<a id="verification-c1-setting"></a>
### Verification: `c1-setting`

- Claim: [c1-setting](#c1-setting)
- Method: `types_file_from_git_config_resolves_against_its_file_and_girconfig_overrides_it` and `types_file_from_command_line_config_resolves_against_repo_root` in `tests/config.rs`; `acceptance.py` with a global `~/` value.
- Evidence considered: a global relative value resolves next to the global config; a repository value resolves next to `.git/config`, run from a subdirectory; `.girconfig` wins; an empty `.girconfig` value restores the defaults; a `-c` value resolves against the root. Mutating the base to the root fails the test (`.scratch/mutation-base-20261002-1930.log`, not committed). `acceptance.py` 14 PASS (`logs/acceptance-20261002-1940.log`).
- Conclusion: `VERIFIED`.
- Limitations: tests run on Linux locally and on the three CI platforms; GUI clients were not exercised.

<a id="verification-c2-replaces"></a>
### Verification: `c2-replaces`

- Claim: [c2-replaces](#c2-replaces)
- Method: `types_file_replaces_types_keeps_names_and_drops_aliases_to_left_out_types` and `init_comments_out_types_when_git_config_names_a_types_file`; `acceptance.py` with the issue's file.
- Evidence considered: only the file's types lint; `fix` and `hotfix` are rejected; `explain types` shows the file and its descriptions; `init` writes `# types` and `cliff.toml` parsers for the file's types. Removing the alias filter fails the test (`.scratch/mutation-alias-20261002-1928.log`, not committed).
- Conclusion: `VERIFIED`.
- Limitations: tests run on Linux locally and on the three CI platforms; GUI clients were not exercised.

<a id="verification-c3-invalid-refuses"></a>
### Verification: `c3-invalid-refuses`

- Claim: [c3-invalid-refuses](#c3-invalid-refuses)
- Method: `invalid_types_file_refuses_naming_file_origin_and_reason`: a missing file and eleven malformed contents through `lint` and `hook commit-msg`, with `explain` and `doctor` on each; `init` refusal through `Config::load_at`, shared with `lint`.
- Evidence considered: each exits `2` with `gir: types file <path> (gir.typesFile in .girconfig): <reason>`, including line and column for a syntax error; `explain` prints the defaults; `doctor` prints the reason. `acceptance.py` shows the same for a value from the global config.
- Conclusion: `VERIFIED`.
- Limitations: tests run on Linux locally and on the three CI platforms; GUI clients were not exercised.

<a id="verification-c4-ignored-fields"></a>
### Verification: `c4-ignored-fields`

- Claim: [c4-ignored-fields](#c4-ignored-fields)
- Method: `TYPES_JSON` in `tests/config.rs` carries `PrefillShortDesc` and an unknown `Extra` field; the issue's file carries `PrefillShortDesc` throughout.
- Evidence considered: both load; `Name` values are read back from `Config::types_file`.
- Conclusion: `VERIFIED`.
- Limitations: tests run on Linux locally and on the three CI platforms; GUI clients were not exercised.

<a id="verification-c5-both-set-warns"></a>
### Verification: `c5-both-set-warns`

- Claim: [c5-both-set-warns](#c5-both-set-warns)
- Method: `types_file_overrides_girconfig_types_with_a_warning`.
- Evidence considered: the file's `wip` lints, `.girconfig`'s `alpha` is rejected, and stderr carries `gir: warning: gir.types in .girconfig is ignored; types come from ...`.
- Conclusion: `VERIFIED`.
- Limitations: tests run on Linux locally and on the three CI platforms; GUI clients were not exercised.

<a id="verification-c6-no-regression"></a>
### Verification: `c6-no-regression`

- Claim: [c6-no-regression](#c6-no-regression)
- Method: `cargo test --no-fail-fast` and `cargo clippy --all-targets -- -D warnings` locally; hosted CI run [37039567751](https://github.com/flatrick/git-it-right/actions/runs/37039567751) at `4f5cf6a` (`logs/ci-run-37039567751-20261002.log`).
- Evidence considered: all test binaries pass (`logs/test-full-20261002-1936.log` on Linux; hook median 6 ms against 750 ms, `logs/test-perf-20261002-1936.log`); clippy is clean (`logs/clippy-20261002-1935.log`); CI passes `test` on Ubuntu, macOS and Windows and `capsule` on Ubuntu and Windows. The previous run, at `b67d564`, passed `test` on all three and failed `capsule` on an index entry the checker could not read (`logs/ci-capsule-failed-37039199722-20261002.log`), fixed in `4f5cf6a` (`logs/capsule-20261002-2010.log`).
- Conclusion: `VERIFIED`.
- Limitations: tests run on Linux locally and on the three CI platforms; GUI clients were not exercised.

### VERIFY gate

`ESTABLISHED`: c1 to c6 are `VERIFIED` through the Verifications above, and the operator accepted the Implement deviations and the c3 narrowing (`ledger.md` A18).

## Learn

### Technical

SourceGit silently falls back to its built-in types on any read error; gir refuses instead, as the operator asked, so the two can disagree only while the file is invalid.
`git config --show-origin` reports a repository value's origin relative to the top level; that, and the other origin facts in Understand, live in the published `config.md` requirements and this bundle. No further destination.

### Process

`scripts/check-capsule` was not run before the first push, and the hosted `capsule` job caught an index entry form that `skills/work-control.md` does not state.
Logged as friction in `SELF-IMPROVEMENT/20261002T172622Z-index-state-form-only-in-checker.md`; deciding a fix is the operator's call under `rules/self-improvement-log.md`.

### LEARN gate

`ESTABLISHED`: both learnings have a disposition above.

## Retention and promotion

### Promotion: `p1-format`

-   Claim: [p1-format](#p1-format)
-   Will this Claim's validity outlive this Task and inform a future decision? `no`: the accepted file format is now stated in `config.md`, and a future type picker reads it from there; SourceGit's own source is the authority for later versions.
-   Disposition: not promoted — Task-scoped only.

### Promotion: `c1-setting` to `c6-no-regression`

-   Claim: [c1-setting](#c1-setting) through [c6-no-regression](#c6-no-regression)
-   Will this Claim's validity outlive this Task and inform a future decision? `no`: they describe this change's behavior, which the published specification states as requirements.
-   Disposition: not promoted — Task-scoped only.

No Claim is carried forward: every Claim is `VERIFIED`.

## Archive readiness

The bundle holds its ledger, probes, acceptance script and logs; internal references are relative.
References outside it (`src/`, `tests/`, commits, PR #9, CI runs, SourceGit's repository) are supplemental evidence anchors.
The mutation logs under `.scratch/` were not kept; their outcome is recorded in Implement.

## Terminal record

### Summary

`COMPLETED`: gir reads SourceGit's type definition file named by `gir.typesFile`, from `.girconfig` or git config, and refuses on an invalid one; implemented in `a7da210`, draft PR #9.

### Gate basis

c1 to c6 are `VERIFIED` through their Verifications, including hosted CI run 37039567751 on Ubuntu, macOS and Windows; the operator accepted every deviation (`ledger.md` A18).
No deferred obligation remains. The specification delta is published in this checkpoint (Specification impact).

## Stop record

`NONE`.
