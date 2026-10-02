# TASK — `gir reads SourceGit's Conventional Commit type definition file`

## Resume

**Contract version:** `2`

**State:** `DEFINE`

**State path:** `DEFINE`

**Resume at:** Get the operator's explicit agreement to the Objective, Success criteria and the Open questions' proposed answers; then gate DEFINE.

**Open obligations:** Operator agreement on DEFINE (blocks `DEFINE`). Open questions below (block `DEFINE`).

## Owned artifacts

-   `ledger.md` - the `sourcegit-types-20261002` thread, taken whole: the operator's answers that shaped this Task.

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

-   Claim: `gir.typesFile` is honored when set in `.girconfig` and when set only in git config (global or repository); when both set it, the `.girconfig` value is used.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: a user cannot share one file across repositories, or a repository cannot override it.
-   Basis: pending check.

<a id="c2-replaces"></a>
#### `c2-replaces`

-   Claim: With a valid types file, `gir lint` accepts exactly the file's `Type` values and rejects every other type, including defaults the file leaves out; `gir explain types` lists the file's types with their `Description`; `gir init` generates `cliff.toml` parsers for the file's types.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: gir and SourceGit disagree on allowed types.
-   Basis: pending check.

<a id="c3-invalid-refuses"></a>
#### `c3-invalid-refuses`

-   Claim: A configured types file that is missing, unreadable, not valid JSON, not an array, has an entry missing `Type`, `Name` or `Description` or with a non-string value for one, an empty or malformed `Type`, or a duplicate `Type`, makes `gir lint`, `gir hook`, `gir fixup` and `gir init` exit `2` before doing anything, with a message naming the file's path, where the setting came from, and the reason (with line and column for a JSON syntax error).
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
-   No other active Task; no overlap.

### Material empirical premises

<a id="p1-format"></a>
#### `p1-format`

-   Claim: SourceGit's type definition file is a JSON array of objects with string fields `Name`, `Type`, `Description` and `PrefillShortDesc`, as in the example in issue #4.
-   State: `UNVERIFIED`
-   Scope: the issue's example; SourceGit's own source not yet read.
-   Consequence if false: gir rejects files SourceGit writes, or accepts ones it does not.
-   Basis: pending check against SourceGit's source in `INVESTIGATE`.

## Understand

### Relevant context

`src/config.rs` reads only `.girconfig`, through `git config --file`; types are a `Vec<String>`, with changelog group titles hardcoded in `DEFAULT_TYPES`.
`gir explain types` takes descriptions from `CHEATSHEET.md`.
No JSON parser is a dependency yet.

### Assumptions

-   NONE yet.

### Open questions

Proposed answers, awaiting the operator:

-   Q8: an alias whose target the file leaves out stops applying, so the aliased type is rejected like any unknown type.
-   A relative `gir.typesFile` resolves against the repository root, wherever it was set; `~/` expands to the home directory.
-   An empty `gir.typesFile =` in `.girconfig` turns off a value inherited from git config.
-   `gir explain` keeps falling back to the defaults on an invalid configuration, including an invalid types file; `gir doctor` reports an invalid types file as a `warn` line, as it does `.girconfig` errors.
-   `gir init` writes `types = ...` into a new `.girconfig`; with a global types file that triggers the c5 warning in every new repository. Proposed: when a types file is configured, `gir init` writes the `types` line commented out.
-   A `Type` is valid when it is nonempty and contains only ASCII letters, digits and `-`.

### Deferred verification

-   NONE.

## Investigate

`NONE` yet.

## Decide

`NONE` yet.

## Implement

`NONE` yet.

## Verify

`NONE` yet.

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
