# TASK — `git tips for useful Git configuration`

## Resume

**Contract version:** `2`

**State:** `DEFINE`

**State path:** `DEFINE`

**Resume at:** Read the CLI, Doctor, and Explain specifications, then define the candidate-screening criteria and catalog contents from the current Git config reference.

**Open obligations:** Screen current Git configuration options and classify candidates at INVESTIGATE; settle the tip catalog and specification delta at DECIDE; implement and verify at their named states.

State must equal the final entry in State path. Append an accepted
transition's destination before beginning work there. The path records
only which transitions occurred; retain material reasons and evidence
in the applicable Task section or owned artifact.

State is the authoritative execution cursor, not proof that a preceding
gate was valid. Do not infer State from artifact presence.

## Owned artifacts

-   `ledger.md` — user decisions and constraints carried from the exploration thread.
-   `decision.md` — selected tip catalog and rationale, created at DECIDE.
-   `verification.md` — acceptance evidence, created at VERIFY.

## Specification impact

- Current contract: `framework:SPEC.md`; `framework:spec/cli.md`; `framework:spec/doctor.md`; `framework:spec/explain.md`
- Proposed delta: Add `gir tips` as a read-only command that lists curated Git settings with safe settings first and states behavioral effects and use cases. Add `diff.colorMoved=plain` to the repo-local Git recommendations checked and fixed by `gir doctor`.
- Terminal publication: `PENDING`

## Define

### Objective

Survey Git configuration options and add a useful, read-only `gir tips` command. Add `diff.colorMoved=plain` to `gir doctor`'s recommended local settings.

### Success criteria

<a id="git-config-catalog"></a>
#### `git-config-catalog`

-   Claim: `gir tips` presents curated, broadly useful developer settings, lists settings that only affect presentation or readability first, and explains the effect and use case for every setting that changes Git behavior.
-   State: `UNVERIFIED`
-   Scope: The implemented CLI and its user-facing documentation.
-   Consequence if false: Developers may apply a setting without understanding its effect or find the catalog hard to use.
-   Basis: Pending implementation and CLI verification.

<a id="doctor-diff-color-moved"></a>
#### `doctor-diff-color-moved`

-   Claim: `gir doctor` recommends `diff.colorMoved=plain` and `gir doctor --fix` sets it in local repository config when unset, while preserving an existing value.
-   State: `UNVERIFIED`
-   Scope: Doctor behavior in a Git repository.
-   Consequence if false: The confirmed doctor recommendation is absent or overwrites user config.
-   Basis: Pending implementation and doctor verification.

<a id="tips-read-only"></a>
#### `tips-read-only`

-   Claim: Running `gir tips` does not modify global or repository Git config.
-   State: `UNVERIFIED`
-   Scope: The `gir tips` command.
-   Consequence if false: A discovery command changes a developer's Git behavior without consent.
-   Basis: Pending implementation and CLI verification.

### Constraints

-   The Task uses worktree `.worktrees/git-config-tips` on branch `feat/git-config-tips`.
-   Any git worktree created for this work must live under `.worktrees/`.
-   `gir tips` only teaches settings; it does not apply them.
-   List settings that only affect presentation or readability before settings that change behavior.
-   For behavior-changing settings, state the effect and when a developer might want them.
-   Follow `addf/CORE.md` and record every lifecycle transition in its own commit.

### Material empirical premises

<a id="git-config-scope"></a>
#### `git-config-scope`

-   Claim: The installed Git is version 2.56.0 and its `git help --config` output lists 1,012 config variable names.
-   State: `UNVERIFIED`
-   Scope: The current execution environment.
-   Consequence if false: The proposed survey baseline may not match this environment.
-   Basis: Observed before Task creation; verify again during INVESTIGATE.

## Understand

### Relevant context

`gir doctor` already recommends repo-local config settings. `gir init` installs gir files and activates hooks. `gir explain` serves pages from Markdown sections. Git config options include personal preferences, shared repo behavior, specialized server settings, and options that are already defaults.

### Assumptions

-   The catalog uses global commands for optional personal settings. Source: accepted plan. Verification state: unverified. Existing doctor local recommendations can still override these values in a repository.
-   The survey uses the current Git config reference available during implementation. Source: accepted plan. Verification state: unverified. Version-sensitive tips must disclose relevant limits.

### Open questions

-   Which candidate settings merit inclusion, and which useful groupings emerge from the survey? Resolve at INVESTIGATE and DECIDE.

### Deferred verification

-   Windows and macOS behavior of any included version- or platform-sensitive setting; settle during VERIFY with documentation and available platform evidence, recording limits where a platform cannot be tested.

## Investigate

No investigation has been performed under this Task yet.

## Decide

Pending the current config-option survey.

## Implement

Not started.

## Verify

Not started.

## Learn

Not started.

## Retention and promotion

No claims have been considered for promotion yet.

## Archive readiness

Pending completion.

## Terminal record

Not terminal.

## Stop record

Not stopped.
