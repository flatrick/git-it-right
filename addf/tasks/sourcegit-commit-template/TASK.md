# TASK — align and generate the SourceGit type template

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** finish mapping init's template ownership and existing types-file behavior.

**Open obligations:** `NONE`.

## Owned artifacts

- `ledger.md` — accepted scope and decisions transferred from the exploration thread.
- `decision.md` — implementation choice and verification approach.

## Specification impact

- Current contract: `framework:spec/init.md#req-init-file-kinds`, `#req-init-required-files`, `#req-init-identical-files`, `#req-init-keep-edits`, `#req-init-force-overwrite`, and `framework:spec/config.md#req-config-types-file`.
- Proposed delta: require `gir init` to generate `conventional-commits.json` in SourceGit's supported schema, with gir's built-in types and order, independent of project-specific types.
- Terminal publication: `PENDING`.

## Define

### Objective

Align the existing SourceGit-format `conventional-commits.json` with gir's built-in types and make `gir init` generate the same file as a required artifact.

### Success criteria

<a id="sourcegit-template-conformance"></a>
#### SourceGit template and init conformance

- Claim: The checked-in JSON and the file created by `gir init` use the SourceGit-compatible `Name`, `Type`, and `Description` format and represent gir's 11 built-in types in order, while init follows the existing required-file, idempotence, and edited-file behavior.
- State: `UNVERIFIED`
- Scope: Built-in configuration and `gir init` behavior in this Task's implementation.
- Consequence if false: Human-facing GUI commit helpers can suggest messages that do not match gir's default accepted types, or repositories will not receive the helper template during setup.
- Basis: Pending implementation and verification.

DEFINE gate: `ESTABLISHED` — the user explicitly approved the objective, built-in-only behavior, and implementation in an isolated worktree.

### Constraints

- Implement in worktree `/tmp/gir-sourcegit-commit-template` on branch `task/sourcegit-commit-template`; leave the original checkout's untracked `conventional-commits.json` untouched.
- Always generate the built-in default types, regardless of custom `.girconfig` types.
- Keep the existing SourceGit JSON schema; `PrefillShortDesc` is optional and must not contain placeholder text.
- Follow the `addf/CORE.md` lifecycle, one commit per State transition.

### Material empirical premises

<a id="existing-template-state"></a>
#### Existing template and parser contract

- Claim: The source checkout's untracked JSON already uses SourceGit's field format and includes the built-in type choices, but has placeholder `PrefillShortDesc` text; gir parses `Type`, `Name`, and `Description` and its built-in list has 11 ordered entries.
- State: `UNVERIFIED`
- Scope: Source checkout at `d487b8078eed99aeae7614e62f4f4287dd4208f0` and current source files inspected during DEFINE.
- Consequence if false: The implementation baseline or proposed output shape may be incomplete.
- Basis: Source checkout inspection and `src/config.rs`; confirm while mapping implementation in UNDERSTAND.

## Understand

### Relevant context

`gir init` creates repository-level files through `src/cmd/init.rs`'s generic idempotent `write` helper. `Config::DEFAULT_TYPES` supplies the built-in order. `src/config.rs` accepts SourceGit arrays with string `Type`, `Name`, and `Description` fields; `PrefillShortDesc` is ignored by gir and optional in SourceGit's documented example.

### Assumptions

- Keep the conventional JSON as an init-required file; this follows the accepted plan and the user's request that `gir init` produce it.
- User-approved built-in list is stable when custom repository types are configured.

### Open questions

`NONE`.

### Deferred verification

`NONE`.

## Investigate

No additional probe is required unless implementation evidence contradicts the accepted design.

## Decide

Pending completion of the subsystem map and selection of the smallest template ownership shape. The generated JSON must be exactly the same content as the checked-in template and must remain independent from custom configured types.

## Implement

Pending.

## Verify

Pending.

## Learn

### Technical

Pending.

### Process

Pending.

## Retention and promotion

No Task-scoped Claim is expected to outlive this Task; assess again before terminalization.

## Archive readiness

Pending.

## Terminal record

### Summary

Pending.

### Gate basis

Pending.

## Stop record

Not applicable unless the Task stops before completion.
