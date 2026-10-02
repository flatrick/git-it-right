# TASK - protect the public repository before enabling CI

## Resume

**Contract version:** `2`

**State:** `UNDERSTAND`

**State path:** `DEFINE -> UNDERSTAND`

**Resume at:** Verify GitHub Actions and branch settings, then inspect how the current workflow runs for fork PRs and pushes.

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
- State: `UNVERIFIED`
- Scope: GitHub settings observed on 2026-10-02.
- Consequence if false: the safeguard plan may target the wrong state.
- Basis: Preliminary API inspection; formal verification belongs in `UNDERSTAND`.

### DEFINE gate

`ESTABLISHED`: the operator asked to secure the public repository before enabling CI, chose this branch and approval for every external PR, asked to protect `main` against force pushes and deletion, and explicitly agreed to the proposed scope and success criteria with CI enabled after the safeguards are verified.

## Understand

Pending DEFINE gate.

## Investigate

Pending.

## Decide

Pending.

## Implement

Pending.

## Verify

Pending.

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
