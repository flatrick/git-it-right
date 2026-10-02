# gir config

## Subspecifications

`NONE`.

## Location and syntax

<a id="req-config-location"></a>
**location.** gir SHALL read its settings from `.girconfig` at the root of the current git repository, in git-config syntax, reading only keys under `gir.`.
The only setting also read from git config SHALL be `gir.typesFile` (see types-file-location).

<a id="req-config-absent"></a>
**absent.** Outside a git repository, or when `.girconfig` does not exist, gir SHALL use the defaults below.

<a id="req-config-case-insensitive-keys"></a>
**case-insensitive-keys.** Key names SHALL be matched case-insensitively, as git config does, so `subjectMax` and `subjectmax` name the same key.

## Keys and defaults

<a id="req-config-types"></a>
**types.** `gir.types` SHALL set the allowed commit types, separated by spaces or commas.
The default SHALL be `feat fix docs style refactor perf test build ci chore revert`.

<a id="req-config-subject-max"></a>
**subject-max.** `gir.subjectMax` SHALL set the longest allowed header in characters; the default SHALL be `72`.

<a id="req-config-scopes"></a>
**scopes.** `gir.scopes` SHALL set the allowed scopes, separated by spaces or commas; empty or unset SHALL allow any scope.

<a id="req-config-scope-required"></a>
**scope-required.** `gir.scopeRequired` SHALL make a scope mandatory when true; the default SHALL be `false`.

<a id="req-config-desc-case"></a>
**desc-case.** `gir.descCase` SHALL accept `any` (the default) or `lower`.

<a id="req-config-allow-merge-revert"></a>
**allow-merge-revert.** `gir.allowMerge` and `gir.allowRevert` SHALL default to `true`.

<a id="req-config-booleans"></a>
**booleans.** Boolean keys SHALL accept `true`, `yes`, `on`, `1` and `false`, `no`, `off`, `0`.

<a id="req-config-aliases"></a>
**aliases.** Keys under `gir.alias.` SHALL map a type alias to an allowed type, overriding a default alias of the same name.
The default aliases SHALL be `feature` and `bugfix` to `feat` and `fix`, `hotfix` to `fix`, `doc` to `docs`, `tests` to `test`, `refactoring` to `refactor`, and `chores` to `chore`.
An alias whose target is not an allowed type SHALL NOT apply, so the aliased type is rejected like any type that is not allowed.

<a id="req-config-hook-missing"></a>
**hook-missing.** `gir.hookMissing` SHALL be accepted by gir and read only by the installed hook scripts (see the hooks module).

## Types file

<a id="req-config-types-file"></a>
**types-file.** `gir.typesFile` SHALL name a SourceGit Conventional Commit type definition file: a JSON array of objects, each with string fields `Type`, `Name` and `Description`.
When it names one, the allowed types SHALL be exactly its `Type` values in file order, each keeping its `Name` and `Description`; other fields, such as `PrefillShortDesc`, SHALL be ignored.

<a id="req-config-types-file-location"></a>
**types-file-location.** `gir.typesFile` SHALL be read from `.girconfig`, and otherwise from git config (system, global, repository or command line).
A relative value SHALL resolve against the directory of the file that set it, and a command-line value against the repository root; `~/` SHALL expand as git expands path values.
An empty value in `.girconfig` SHALL mean no types file, even when git config sets one.

<a id="req-config-types-file-invalid"></a>
**types-file-invalid.** A named types file that cannot be read, is not valid JSON, is not a nonempty array, has an entry that is not an object, lacks a string `Type`, `Name` or `Description`, has a `Type` that is empty or not only ASCII letters, digits and `-`, or repeats a `Type`, SHALL fail with `types file <path> (gir.typesFile in <origin>): ` followed by the reason, and exit `2`.
The reason SHALL name the entry by position and, once read, its `Type`; for a JSON syntax error it SHALL give line and column. A leading UTF-8 byte order mark SHALL be ignored.

<a id="req-config-types-file-overrides-types"></a>
**types-file-overrides-types.** When `.girconfig` sets `gir.types` and a types file is named, the types file SHALL win and gir SHALL print a warning naming both.

## Invalid settings

<a id="req-config-invalid-value"></a>
**invalid-value.** A non-boolean value for a boolean key, a non-number `subjectMax`, or a `descCase` other than `any` or `lower` SHALL fail with `.girconfig: ` followed by the key, the given value and the expected form, and exit `2`.

<a id="req-config-unknown-key"></a>
**unknown-key.** Any other key under `gir.` SHALL fail with `.girconfig: unknown key` and exit `2`.

<a id="req-config-malformed"></a>
**malformed.** A `.girconfig` that git cannot parse SHALL fail with `.girconfig: ` followed by git's error and exit `2`; a `.girconfig` with no `gir.` key SHALL mean the defaults.

<a id="req-config-explain-tolerates-invalid"></a>
**explain-tolerates-invalid.** `gir explain` SHALL fall back to the defaults when `.girconfig` or the types file it uses is invalid instead of failing.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
