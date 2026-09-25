# gir config

## Subspecifications

`NONE`.

## Location and syntax

<a id="req-config-location"></a>
**location.** gir SHALL read its settings from `.girconfig` at the root of the current git repository, in git-config syntax, reading only keys under `gir.`.

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

<a id="req-config-hook-missing"></a>
**hook-missing.** `gir.hookMissing` SHALL be accepted by gir and read only by the installed hook scripts (see the hooks module).

## Invalid settings

<a id="req-config-invalid-value"></a>
**invalid-value.** A non-boolean value for a boolean key, a non-number `subjectMax`, or a `descCase` other than `any` or `lower` SHALL fail with `.girconfig: ` followed by the key, the given value and the expected form, and exit `2`.

<a id="req-config-unknown-key"></a>
**unknown-key.** Any other key under `gir.` SHALL fail with `.girconfig: unknown key` and exit `2`.

<a id="req-config-malformed"></a>
**malformed.** A `.girconfig` that git cannot parse SHALL fail with `.girconfig: ` followed by git's error and exit `2`; a `.girconfig` with no `gir.` key SHALL mean the defaults.

<a id="req-config-explain-tolerates-invalid"></a>
**explain-tolerates-invalid.** `gir explain` SHALL fall back to the defaults when `.girconfig` is invalid instead of failing.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
