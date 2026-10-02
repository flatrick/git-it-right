# TASK - protect the public repository before enabling CI

## Resume

**Contract version:** `2`

**State:** `INVESTIGATE`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE`

**Resume at:** Settle the branch protection payload, fork approval policy, and safe order for enabling Actions.

**Open obligations:** Live settings and workflow inspection block `UNDERSTAND`; repository settings and workflow changes block `IMPLEMENT`; readback, workflow checks, and safely enabling CI block `VERIFY`.

## Owned artifacts

- `ledger.md` - the operator's choices and the open scope question.

## Specification impact

- Current contract: `framework:SPEC.md`; the product specification has no repository administration or CI contract.
- Proposed delta: `NONE`; this Task changes repository settings and the CI workflow, not `gir` behavior.
- Terminal publication: `PENDING`

## Define

### Objective

Protect `main` against force pushes and deletion, prevent unapproved external PRs from starting CI runs, then enable CI after verifying the safeguards.

### Success criteria

<a id="main-protected"></a>
#### `main-protected`

- Claim: GitHub protects `main` against force pushes and deletion, including for administrators, without requiring a disabled CI check.
- State: `UNVERIFIED`
- Scope: `flatrick/git-it-right` after the settings change.
- Consequence if false: an authorized account could rewrite or delete `main`, or branch protection could block normal work while CI is disabled.
- Basis: Pending readback from GitHub.

<a id="fork-approval"></a>
#### `fork-approval`

- Claim: GitHub requires approval before Actions runs a workflow from every external fork PR.
- State: `UNVERIFIED`
- Scope: `flatrick/git-it-right` after the settings change.
- Consequence if false: unreviewed external PRs can start runs.
- Basis: Pending readback from GitHub.

<a id="workflow-bounded"></a>
#### `workflow-bounded`

- Claim: The CI workflow grants read-only repository access, uses standard hosted runners, and has bounded job duration and redundant-run cancellation.
- State: `UNVERIFIED`
- Scope: the final revision of `.github/workflows/ci.yml`.
- Consequence if false: a permitted run can consume more resources or receive more authority than needed.
- Basis: Pending workflow inspection and validation.

<a id="actions-state"></a>
#### `actions-state`

- Claim: Actions remains disabled until the safeguards are verified, then the `ci` workflow runs on this branch without unapproved external PR execution.
- State: `UNVERIFIED`
- Scope: `flatrick/git-it-right` at completion.
- Consequence if false: CI starts before the safeguards are in place or remains disabled after the operator-approved enable step.
- Basis: Pending GitHub readback.

### Constraints

- Work in the current `.worktrees/fixup-modes` worktree on `feat/fixup-modes`, as the operator chose in `ledger.md`.
- Require approval for every external PR, as the operator chose in `ledger.md`.
- Keep Actions disabled until protection, approval policy, and workflow safeguards are verified. Then enable CI as the operator requested.
- This Task precedes `doctor-fix-file-mode`'s hosted Windows CI verification. It proposes no product specification delta; that Task publishes its doctor delta after its own CI verification.

### Material empirical premises

<a id="public-unprotected"></a>
#### `public-unprotected`

- Claim: `flatrick/git-it-right` is public, Actions is disabled, and `main` currently has no branch protection.
- State: `VERIFIED`
- Scope: GitHub settings observed on 2026-10-02.
- Consequence if false: the safeguard plan may target the wrong state.
- Basis: [Verification](#verification-public-unprotected).

### DEFINE gate

`ESTABLISHED`: the operator asked to secure the public repository before enabling CI, chose this branch and approval for every external PR, asked to protect `main` against force pushes and deletion, and explicitly agreed to the proposed scope and success criteria with CI enabled after the safeguards are verified.

## Understand

### Relevant context

- GitHub reports a public repository with `main` as the default branch. `main` is unprotected and there are no repository rulesets.
- Repository Actions are disabled, and workflow `ci` is also `disabled_manually`. The default workflow token permission is `read`; there are no self-hosted runners.
- Fork PR approval is currently `first_time_contributors`. The operator chose `all_external_contributors`.
- `.github/workflows/ci.yml` runs on pushes to `main` and on pull requests. Its jobs use standard GitHub-hosted Ubuntu, Windows, and macOS runners. It uploads no artifacts or caches, but has no explicit token permissions, job time limits, or concurrency cancellation.
- The workflow uses `pull_request`, not `pull_request_target`. It checks out and tests PR code. Approval is therefore the main control against an external contributor repeatedly starting unreviewed runs.
- GitHub documents that standard hosted runner use in public repositories is free. Larger runners and storage have different billing rules, so the workflow should remain on standard runners and avoid artifact storage.

### Assumptions

- GitHub's `all_external_contributors` policy prevents unapproved external fork PR workflow runs, as documented; the live policy must be read back after setting it.

### Open questions

- Whether the current credentials can set branch protection and the fork PR approval policy.
- Whether the existing `ci` workflow can be enabled after repository Actions are enabled, and then observed running on this branch.

### Deferred verification

- An unapproved fork PR will not be created merely to exercise the approval gate. Verify the live policy via GitHub's API and its documented meaning; a future real fork PR can confirm the user interface.

### UNDERSTAND gate

`ESTABLISHED`: live settings, the workflow trigger path, and the controls available for the requested outcome are identified. Credential support and the enable sequence are narrow uncertainties for `INVESTIGATE`.

## Investigate

Pending.

## Decide

Pending.

## Implement

Pending.

## Verify

<a id="verification-public-unprotected"></a>
### Verification: `public-unprotected`

- Claim: [public-unprotected](#public-unprotected).
- Method: Read the repository, Actions, branch, rulesets, and workflow settings through GitHub's API on 2026-10-02.
- Evidence considered: `gh api repos/flatrick/git-it-right` returned `private:false`, `visibility:public`, `default_branch:main`; the Actions permissions endpoint returned `enabled:false`; the `main` endpoint returned `protected:false`; the rulesets endpoint returned `[]`; the workflow endpoint returned `state:disabled_manually`.
- Conclusion: `VERIFIED` for the observed state.
- Limitations: GitHub settings can change after observation; final settings require fresh readback.

## Learn

Pending.

## Retention and promotion

Pending.

## Archive readiness

Pending.

## Terminal record

Pending.

## Stop record

Pending.
