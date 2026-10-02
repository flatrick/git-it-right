# TASK — `gir reads SourceGit's Conventional Commit type definition file`

## Resume

**Contract version:** `2`

**State:** `VERIFY`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY`

**Resume at:** Write a Verification for each of c1 to c6 from the Implement checkpoints; c6 on Windows and macOS needs hosted CI, which needs the operator to push.

**Open obligations:** `c6-no-regression` on Windows and macOS (blocks `VERIFY`). Operator review of the Implement deviations (blocks `VERIFY`).

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

## Specification impact

- Current contract: `framework:spec/config.md#req-config-location`, `framework:spec/config.md#req-config-types`, `framework:spec/config.md#req-config-aliases`, `framework:spec/config.md#req-config-malformed`, `framework:spec/config.md#req-config-explain-tolerates-invalid`, `framework:spec/explain.md#req-explain-types-config`, `framework:spec/explain.md#req-explain-types-summaries`, `framework:spec/init.md#req-init-cliff-parsers`, `framework:spec/init.md#req-init-config-error`, `framework:spec/doctor.md#req-doctor-girconfig`.
- Proposed delta: a new `gir.typesFile` setting, read from git config and `.girconfig`, naming a SourceGit-format JSON file whose entries become the complete list of allowed types, with their names and descriptions; an invalid file makes gir refuse with the file's path and the reason. Exact wording settled in `DECIDE`.
- Terminal publication: `PENDING`

## Define

### Objective

A repository, or a user for all their repositories, can point gir at the JSON file SourceGit uses to define Conventional Commit types, and gir then allows exactly those types, so gir, git on the CLI and SourceGit agree on them (GitHub issue #4).

### Success criteria

<a id="c1-setting"></a>
#### `c1-setting`

-   Claim: `gir.typesFile` is honored when set in `.girconfig` and when set only in git config (global or repository); when both set it, the `.girconfig` value is used; a relative value resolves against the directory of the file that set it; an empty value in `.girconfig` turns off the git config value.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: a user cannot share one file across repositories, or a repository cannot override it.
-   Basis: pending check.

<a id="c2-replaces"></a>
#### `c2-replaces`

-   Claim: With a valid types file, `gir lint` accepts exactly the file's `Type` values and rejects every other type, including defaults the file leaves out and aliases to them; `gir explain types` lists the file's types with their `Description`; `gir init` generates `cliff.toml` parsers for the file's types and writes a new `.girconfig`'s `types` line commented out.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: gir and SourceGit disagree on allowed types.
-   Basis: pending check.

<a id="c3-invalid-refuses"></a>
#### `c3-invalid-refuses`

-   Claim: A configured types file that is missing, unreadable, not valid JSON, not an array, has an entry missing `Type`, `Name` or `Description` or with a non-string value for one, an empty or malformed `Type`, or a duplicate `Type`, makes `gir lint`, `gir hook` and `gir init` exit `2` before doing anything, with a message naming the file's path, where the setting came from, and the reason (with line and column for a JSON syntax error); `gir explain` instead falls back to the defaults and `gir doctor` prints the error as a `warn` line.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: gir acts on a types list the user did not intend, or the user cannot find what to fix.
-   Basis: pending check.

<a id="c4-ignored-fields"></a>
#### `c4-ignored-fields`

-   Claim: `PrefillShortDesc` and unknown fields are accepted and ignored; `Name` is parsed and kept on each type in gir's configuration though nothing displays it yet.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: a SourceGit file with extra fields breaks gir, or the future type picker has no names.
-   Basis: pending check.

<a id="c5-both-set-warns"></a>
#### `c5-both-set-warns`

-   Claim: When `gir.types` is set in `.girconfig` and a types file is configured, the file's types are used and gir prints a warning naming both to stderr.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: a user silently gets a different type list from the one they wrote.
-   Basis: pending check.

<a id="c6-no-regression"></a>
#### `c6-no-regression`

-   Claim: Without `gir.typesFile`, gir behaves as before: `cargo test` and `cargo clippy --all-targets -- -D warnings` pass.
-   State: `UNVERIFIED`
-   Scope: this branch, Linux; Windows and macOS through CI.
-   Consequence if false: existing users break.
-   Basis: pending check.

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

## Learn

### Technical

`NONE` yet.

### Process

`NONE` yet.

## Retention and promotion

`NONE` yet.

## Archive readiness

`NONE` yet.

## Terminal record

`NONE` yet.

## Stop record

`NONE`.
