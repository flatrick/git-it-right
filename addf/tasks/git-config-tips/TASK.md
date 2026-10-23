# TASK — `git tips for useful Git configuration`

## Resume

**Contract version:** `2`

**State:** `LEARN`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY -> LEARN`

**Resume at:** Record the learning disposition, then publish the verified CLI and doctor specifications and terminalize the Task.

**Open obligations:** Record learning and promotion decisions; publish the verified specification delta and terminalize the Task.

State must equal the final entry in State path. Append an accepted
transition's destination before beginning work there. The path records
only which transitions occurred; retain material reasons and evidence
in the applicable Task section or owned artifact.

State is the authoritative execution cursor, not proof that a preceding
gate was valid. Do not infer State from artifact presence.

## Owned artifacts

-   `ledger.md` — user decisions and constraints carried from the exploration thread.
-   `investigation.md` — screened config settings, candidate effects, and preliminary dispositions.
-   `grounding.md` — existing CLI, doctor, and explain architecture.
-   `evidence/git-config-reference.md` — installed reference baseline and official setting descriptions.
-   `evidence/git-config-names-2.56.0.txt` — exact output from `git help --config`.
-   `verifications/git-config-scope.md` — verifies the installed version and config variable count.
-   `decision.md` — selected tip catalog and rationale, created at DECIDE.
-   `verification.md` — acceptance evidence, created at VERIFY.
-   `.audit/git-config-tips.todo.md` — pstack architect phases and feature throughput checkpoint, local to this worktree.

## Specification impact

- Current contract: `framework:SPEC.md`; `framework:spec/cli.md`; `framework:spec/doctor.md`
- Proposed delta: Add `gir tips` as a read-only, repository-independent command that lists curated Git settings with presentation settings first and states each behavior effect and useful situation. Add a tips child specification and list it in `SPEC.md`. Add `diff.colorMoved=plain` to the repo-local Git recommendations checked and fixed by `gir doctor`.
- Terminal publication: `PENDING`

## Define

### Objective

Survey Git configuration options and add a useful, read-only `gir tips` command. Add `diff.colorMoved=plain` to `gir doctor`'s recommended local settings.

### Success criteria

<a id="git-config-catalog"></a>
#### `git-config-catalog`

-   Claim: `gir tips` presents curated, broadly useful developer settings, lists settings that only affect presentation or readability first, and explains the effect and use case for every setting that changes Git behavior.
-   State: `VERIFIED`
-   Scope: The implemented CLI and its user-facing documentation.
-   Consequence if false: Developers may apply a setting without understanding its effect or find the catalog hard to use.
-   Basis: [Catalog conclusion](verification.md#catalog-conclusion).

<a id="doctor-diff-color-moved"></a>
#### `doctor-diff-color-moved`

-   Claim: `gir doctor` recommends `diff.colorMoved=plain` and `gir doctor --fix` sets it in local repository config when unset, while preserving an existing value.
-   State: `VERIFIED`
-   Scope: Doctor behavior in a Git repository.
-   Consequence if false: The confirmed doctor recommendation is absent or overwrites user config.
-   Basis: [Doctor conclusion](verification.md#doctor-conclusion).

<a id="tips-read-only"></a>
#### `tips-read-only`

-   Claim: Running `gir tips` does not modify global or repository Git config.
-   State: `VERIFIED`
-   Scope: The `gir tips` command.
-   Consequence if false: A discovery command changes a developer's Git behavior without consent.
-   Basis: [Read-only conclusion](verification.md#read-only-conclusion).

### Constraints

-   The Task uses worktree `.worktrees/git-config-tips` on branch `feat/git-config-tips`.
-   Any git worktree created for this work must live under `.worktrees/`.
-   `gir tips` only teaches settings; it does not apply them.
-   List settings that only affect presentation or readability before settings that change behavior.
-   For behavior-changing settings, state the effect and when a developer might want them.
-   Follow `addf/CORE.md` and record every lifecycle transition in its own commit.

### DEFINE exit claim

-   Outcome and bounds can guide investigation and verification: `ESTABLISHED`.
-   Basis: The operator explicitly said “Implement the plan” after reviewing the proposed scope, including the Git settings survey, `gir tips`, `diff.colorMoved=plain`, behavior disclosures, and validation criteria. The current CLI, configuration, doctor, and explain specs were read. The worktree and branch constraints are recorded above.

### Material empirical premises

<a id="git-config-scope"></a>
#### `git-config-scope`

-   Claim: The installed Git is version 2.56.0 and its `git help --config` output lists 1,012 config variable names.
-   State: `VERIFIED`
-   Scope: The current execution environment.
-   Consequence if false: The proposed survey baseline may not match this environment.
-   Basis: [Verification](verifications/git-config-scope.md#conclusion).

## Understand

### Relevant context

The CLI parses subcommands and dispatches them in `src/main.rs`. Command implementations live in `src/cmd/` and return an exit code or one runtime error. `gir doctor` stores local recommendations in a static tuple list, reports unset values, and fixes them with `git config --local`. It leaves nonempty existing values unchanged. `gir explain` reads topic text from Markdown sections, but a new top-level `gir tips` command does not need to become an explain topic. The README has a command table and describes doctor's scope.

`gir doctor` and `gir tips` have distinct roles. Doctor checks the current repository and may set repo-local recommendations. Tips teaches optional personal settings and must not change config. The doctor spec enumerates recommended values. The CLI spec enumerates commands and help behavior.

### UNDERSTAND exit claim

-   The model is sufficient to avoid uninformed action: `ESTABLISHED`.
-   Basis: The current CLI dispatcher, doctor config checks, explain topic loader, command module list, README command table, and relevant CLI, config, doctor, and explain specs were inspected. This establishes the existing command pattern, local config behavior, documentation locations, and required read-only boundary for tips.

### Assumptions

-   The catalog uses global commands for optional personal settings. Source: accepted plan. Verification state: unverified. Existing doctor local recommendations can still override these values in a repository.
-   The survey uses the current Git config reference available during implementation. Source: accepted plan. Verification state: unverified. Version-sensitive tips must disclose relevant limits.
-   The installed Git reference is an appropriate baseline for this survey. Source: accepted plan and installed Git 2.56.0. Verification state: unverified until evidence is recorded in INVESTIGATE.

### Open questions

-   Which candidate settings merit inclusion, and which useful groupings emerge from the survey? Resolve at INVESTIGATE and DECIDE.

### Deferred verification

-   Windows and macOS behavior of any included version- or platform-sensitive setting; settle during VERIFY with documentation and available platform evidence, recording limits where a platform cannot be tested.

## Investigate

The installed Git is 2.56.0 and lists 1,012 config variables. The full list and official descriptions are recorded in the Evidence files. The survey distinguishes presentation-only settings from changes to Git workflow, refs, and large-repository performance. It excludes specialty, tool-specific, server, and advanced tuning options from the initial audience.

The candidate inventory and preliminary dispositions are in [the survey](investigation.md). The current Git reference confirms the effects and limitations needed to choose a useful catalog.

### INVESTIGATE exit claim

-   Every decision-relevant uncertainty has a disposition: `ESTABLISHED`.
-   Basis: The complete installed variable list was retained and screened. The candidate settings' effects, defaults, scope, and material caveats were checked against Git's current manual. The remaining questions concern product choices about the curated subset and its grouping; DECIDE owns those choices.

## Decide

### DECIDE exit claim

-   The selected approach resolves the material product and architecture choices: `ESTABLISHED`.
-   Basis: The survey separates presentation settings from workflow, ref, and performance behavior. Three structurally distinct designs were compared against the task's contract and screened for design red flags. The cross-judge recommends Candidate 1 (25/25); [decision.md](decision.md) records the selected structure, twelve settings, disclosures, exclusions, output contract, specification delta, rejected alternatives, and verification plan.

## Implement

### IMPLEMENT exit claim

-   The intended change exists and can be evaluated: `ESTABLISHED`.
-   Basis: The integrated feature branch contains the private twelve-entry catalog, zero-argument `gir tips` dispatch and usage, `diff.colorMoved=plain` in doctor's local recommendations, tests for catalog and doctor behavior, and README/onboarding documentation. The changes follow the selected ownership and ordering in [decision.md](decision.md). The implementation commit is `3e4a0b2`; the separate documentation commit is `0617c7e`.

### Implementation reconciliation

-   The implementation follows Candidate 1's data shape and boundary. `page()` builds the output from two static slices, with all entries carrying both explanatory fields. The dispatcher writes the returned page and never loads config or enters a repository on the `tips` path.
-   The behavior copy adds the surveyed `remote.<name>.skipFetchAll` exception to `fetch.all` and explicitly says `push.followTags` excludes lightweight tags. These details sharpen the approved effects without changing the catalog selection.
-   README and the generated contributor guide present `gir tips` as optional advice. Neither `gir init` nor `gir doctor --fix` applies the global examples.

## Verify

### VERIFY exit claim

-   Evidence establishes whether completion is justified: `ESTABLISHED`.
-   Basis: [`verification.md`](verification.md) evaluates all three success claims against the integrated CLI tests and Clippy run. The catalog and read-only tests exercise actual command behavior. The doctor tests verify setting, fix, and preserve paths. No contradictory results appeared in the tested Linux scope.

## Learn

### LEARN exit claim

-   Material learning and its disposition were considered: `ESTABLISHED`.
-   Basis: The typed two-group catalog, explicit behavior copy, and read-only CLI contract are recorded in this Task and proposed for the current specs. No unresolved technical or process lesson requires a separate current artifact.

### Technical

No material technical learning needs a permanent artifact. The ordered static groups and separate doctor config policy are specific to this catalog and are preserved in the Task bundle and current specs.

### Process

No material process learning needs a permanent artifact. The isolated feature worktree and the nested implementation worktree both stayed beneath `.worktrees/`, and the addf transitions each have their own commit.

## Retention and promotion

### Promotion: `git-config-catalog`

-   Claim: [Catalog claim](#git-config-catalog).
-   Will this Claim's validity outlive this Task and inform a future decision? `NO`; the catalog contract is published in the current tips specification, while this Verification establishes only the current implementation.
-   Disposition: Not promoted; Task-scoped verification.

### Promotion: `doctor-diff-color-moved`

-   Claim: [Doctor claim](#doctor-diff-color-moved).
-   Will this Claim's validity outlive this Task and inform a future decision? `NO`; the doctor contract is published in its current specification, while this Verification establishes only the current implementation.
-   Disposition: Not promoted; Task-scoped verification.

### Promotion: `tips-read-only`

-   Claim: [Read-only claim](#tips-read-only).
-   Will this Claim's validity outlive this Task and inform a future decision? `NO`; the user-facing requirement is published in the tips specification, while this Verification establishes only the current implementation.
-   Disposition: Not promoted; Task-scoped verification.

### Promotion: `git-config-scope`

-   Claim: [Git config scope premise](#git-config-scope).
-   Will this Claim's validity outlive this Task and inform a future decision? `NO`; the installed version and option count describe the investigation environment only.
-   Disposition: Not promoted; Task-scoped observation.

## Archive readiness

The bundle is self-contained: its Ledger, survey, grounding, evidence, decision, and verification artifacts are included. Task-internal links are ordinary relative links. Git manuals and articles are supplemental evidence. No current specification or consumer depends on the Task bundle after publication.

## Terminal record

Not terminal.

## Stop record

Not stopped.
