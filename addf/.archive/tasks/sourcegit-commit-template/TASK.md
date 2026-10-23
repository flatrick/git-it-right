# TASK — align and generate the SourceGit type template

## Resume

**Contract version:** `2`

**State:** `COMPLETED`

**State path:** `DEFINE -> UNDERSTAND -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN -> COMPLETED`

**Resume at:** `NONE`

**Open obligations:** `NONE`.

## Owned artifacts

- `ledger.md` — accepted scope and decisions transferred from the exploration thread.
- `decision.md` — implementation choice and verification approach.

## Specification impact

- Current contract: `framework:spec/init.md#req-init-file-kinds`, `framework:spec/init.md#req-init-required-files`, `framework:spec/init.md#req-init-identical-files`, `framework:spec/init.md#req-init-keep-edits`, `framework:spec/init.md#req-init-force-overwrite`, and `framework:spec/config.md#req-config-types-file`.
- Proposed delta: require `gir init` to generate `conventional-commits.json` in SourceGit's supported schema, with gir's built-in types and order, independent of project-specific types.
- Terminal publication: `framework:spec/init.md#req-init-sourcegit-file`, `framework:spec/init.md#req-init-sourcegit-before-config`.

## Define

### Objective

Align the existing SourceGit-format `conventional-commits.json` with gir's built-in types and make `gir init` generate the same file as a required artifact.

### Success criteria

<a id="sourcegit-template-conformance"></a>
#### SourceGit template and init conformance

- Claim: The checked-in JSON and the file created by `gir init` use the SourceGit-compatible `Name`, `Type`, and `Description` format and represent gir's 11 built-in types in order, while init follows the existing required-file, idempotence, and edited-file behavior.
- State: `VERIFIED`
- Scope: Built-in configuration and `gir init` behavior in this Task's implementation.
- Consequence if false: Human-facing GUI commit helpers can suggest messages that do not match gir's default accepted types, or repositories will not receive the helper template during setup.
- Basis: `TASK.md#verification-sourcegit-template-conformance-conclusion`.

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
- State: `VERIFIED`
- Scope: Source checkout at `d487b8078eed99aeae7614e62f4f4287dd4208f0` and current source files inspected during DEFINE.
- Consequence if false: The implementation baseline or proposed output shape may be incomplete.
- Basis: `TASK.md#verification-existing-template-state-conclusion`.

## Understand

### Relevant context

`gir init` writes root-level files with the generic idempotent writer in `src/cmd/init.rs`. Required files are written before `Config::load_at`; optional `cliff.toml` and `GIT-IT-RIGHT.md` are handled after config loads. `src/cmd/templates.rs` owns generated contents. `Config::DEFAULT_TYPES` provides the built-in order, while gir's SourceGit reader requires `Type`, `Name`, and `Description` and ignores optional fields such as `PrefillShortDesc`.

`Cargo.toml` explicitly limits packaged files, so embedding the root JSON with `include_str!` also requires adding that JSON to the package include list.

### Evidence and interpretation

- Observation: The source checkout's JSON parses to 11 entries, starts with `build`, and ends with `test`; gir's `DEFAULT_TYPES` starts with `feat` and ends with `revert`. Its `feat` entry has `PrefillShortDesc: "this is a test"`.
- Observation: `read_types_file` requires string `Type`, `Name`, and `Description` fields and permits additional JSON fields. The targeted existing test `cargo test --test config types_file_replaces_types_keeps_names_and_drops_aliases_to_left_out_types` passed in this worktree.
- Observation: `src/cmd/init.rs` writes `.girconfig` before `Config::load_at`. A required `conventional-commits.json` should be written before that load so a configured `gir.typesFile` can refer to the generated path.
- Observation: `Cargo.toml` has an explicit `include` list that does not currently name the root JSON.
- SourceGit's documented example uses the same `Name`, `Type`, and `Description` fields; `PrefillShortDesc` is optional. [SourceGit README](https://github.com/sourcegit-scm/sourcegit/blob/master/README.md?plain=1)
- Interpretation: Keep current human labels and descriptions, reorder entries to `DEFAULT_TYPES`, omit placeholder prefill text, and use the JSON itself as the single canonical template source.

### Assumptions

- Keep the conventional JSON as an init-required file; this follows the accepted plan and the user's request that `gir init` produce it.
- User-approved built-in list is stable when custom repository types are configured.

### Open questions

`NONE`.

### Deferred verification

`NONE`.

## Investigate

Investigation gate: `NOT_APPLICABLE` — the implementation ownership, schema, built-in order, and package inclusion constraint are established by current source and the existing parser test. No remaining uncertainty could change the selected template shape.

## Decide

See [decision.md](decision.md). The root JSON is the one canonical template; init writes it before config loading through the existing required-file writer.

<a id="verification-existing-template-state-conclusion"></a>
Verification of `existing-template-state`: `VERIFIED` — source inspection, parsing the untracked file, and the existing gir types-file test established the current field format, type set, ordering mismatch, placeholder, and parser compatibility. Scope is the original checkout's `conventional-commits.json` at the initial source revision and the gir source and test at this Task's UNDERSTAND checkpoint. The SourceGit schema claim is supported by the [SourceGit README](https://github.com/sourcegit-scm/sourcegit/blob/master/README.md?plain=1).

UNDERSTAND gate: `ESTABLISHED` — the init write path, template owner, type source, schema, test coverage pattern, and package file boundary are understood well enough to decide the implementation.

## Implement

Added the root JSON template in gir's built-in order, embedded it with `include_str!`, included it in Cargo packages, and made init write it as a required file before loading config. Updated init specs, CLI usage, README, and generated onboarding docs. Added init behavior coverage for type order and fields, configured `typesFile` loading, keep/force behavior, CRLF reruns, and restoration after adoption.

The first configured-types-file test used a relative path in local Git config, which resolves from `.git`. Replaced it with `.girconfig`'s repository-root-relative path; the corrected test passes.

Implementation checks so far: `cargo test` passed all suites; `cargo fmt -- --check`, `git diff --check`, and `cargo package --list --allow-dirty` passed, with the package list including `conventional-commits.json`. Final success claim remains `UNVERIFIED` until VERIFY records direct artifact evidence.

IMPLEMENT gate: `ESTABLISHED` — the selected behavior and documentation changes are implemented in commit `6e1d4f4`; verification remains the next state.

## Verify

### Verification: sourcegit-template-conformance

#### Method

Run the full Rust suite, check formatting and whitespace, inspect the package file list, and parse the committed JSON against the expected built-in order and SourceGit fields. Init integration tests exercise the generated file through `gir init` and read its output from temporary repositories.

#### Evidence considered

- `cargo test --quiet` exited `0`; all Rust test suites passed, including the 26 init tests and 12 CLI argument tests.
- Init tests confirmed exact output equality with the canonical JSON, the built-in type order, required fields, no `PrefillShortDesc`, independence from custom `.girconfig` types, edit preservation, `--force`, CRLF idempotence, adopted-repository restoration, and loading when `.girconfig` names the generated file as `gir.typesFile`.
- `cargo fmt -- --check` and `git diff --check` exited `0`.
- `cargo package --list --allow-dirty` included `conventional-commits.json`.
- A JSON parse of the committed file returned `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert` in order, with string `Name`, `Type`, and `Description` fields and no `PrefillShortDesc`.
- SourceGit's documented example uses `Name`, `Type`, and `Description`; its example makes `PrefillShortDesc` optional. [SourceGit README](https://github.com/sourcegit-scm/sourcegit/blob/master/README.md?plain=1)

<a id="verification-sourcegit-template-conformance-conclusion"></a>
#### Conclusion

**Result:** `VERIFIED`

The committed template matches gir's built-in type order and accepted SourceGit type-file schema. `gir init` writes it as a required file and follows the existing edit, force, and rerun behavior. The crate package contains the embedded source file.

#### Remaining uncertainty

The SourceGit application itself was not launched. Compatibility follows SourceGit's published schema example and gir's parser tests. This does not block the claim about the documented JSON format.

VERIFY gate: `ESTABLISHED` — the success criteria are supported by integration tests, full-suite results, package inclusion, and direct inspection of the committed JSON.

## Learn

### Technical

No new reusable technical fact emerged. The work confirmed the existing contract that relative `gir.typesFile` paths in `.girconfig` resolve from the repository root; the configuration spec and existing tests already cover that behavior.

### Process

The managed workspace denied the first write to Git metadata, then allowed it after explicit escalation. The friction is recorded in `addf/SELF-IMPROVEMENT/20261023T175956Z-git-metadata-write-access.md`. No framework change is justified by this environment-specific event.

## Retention and promotion

### Promotion: sourcegit-template-conformance

- Claim: [SourceGit template and init conformance](TASK.md#sourcegit-template-conformance)
- Will this Claim's validity outlive this Task and inform a future decision? `no`; the accepted behavior now lives in the current init specification and tests.
- Disposition: not promoted — Task-scoped only.

### Promotion: existing-template-state

- Claim: [Existing template and parser contract](TASK.md#existing-template-state)
- Will this Claim's validity outlive this Task and inform a future decision? `no`; it describes the initial untracked file and the parser behavior already documented in current source and tests.
- Disposition: not promoted — Task-scoped only.

No additional Learning artifact qualifies for promotion. The path-resolution fact was already covered by the configuration contract. The managed-workspace friction is recorded in the addf self-improvement log, with no broader rule proposed.

## Archive readiness

The bundle contains its Ledger handoff and implementation decision. Its internal link to `decision.md` is relative and remains valid after archival. The SourceGit README and current framework specifications are supplemental references; the Task's decision, tests, and verification record preserve the basis for its outcome. No current consumer depends on the Task directory.

## Terminal record

### Summary

Updated the SourceGit JSON template to match gir's built-in types and made `gir init` create it as a required file. Documented the file, added init behavior coverage, and published the completed init contract.

### Gate basis

The success Claim `sourcegit-template-conformance` and material premise `existing-template-state` are `VERIFIED` through their Task verification anchors. The full Rust suite passed, formatting and whitespace checks passed, Cargo packaging includes the JSON, and the committed JSON was parsed and checked against the expected order. No open obligation or deferred verification remains.

## Stop record

Not applicable unless the Task stops before completion.
