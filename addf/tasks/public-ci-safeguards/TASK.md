# TASK - protect the public repository before enabling CI

## Resume

**Contract version:** `2`

**State:** `VERIFY`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE -> IMPLEMENT -> VERIFY`

**Resume at:** Read back every safeguard and confirm the pushed workflow, then enable repository Actions and the `ci` workflow and observe PR #5.

**Open obligations:** Readback, workflow checks, safely enabling CI, and hosted run observation block `VERIFY`.

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
- State: `VERIFIED`
- Scope: `flatrick/git-it-right` after the settings change.
- Consequence if false: an authorized account could rewrite or delete `main`, or branch protection could block normal work while CI is disabled.
- Basis: [Verification](#verification-main-protected).

<a id="fork-approval"></a>
#### `fork-approval`

- Claim: GitHub requires approval before Actions runs a workflow from every external fork PR.
- State: `VERIFIED`
- Scope: `flatrick/git-it-right` after the settings change.
- Consequence if false: unreviewed external PRs can start runs.
- Basis: [Verification](#verification-fork-approval).

<a id="workflow-bounded"></a>
#### `workflow-bounded`

- Claim: The CI workflow grants read-only repository access, uses standard hosted runners, and has bounded job duration and redundant-run cancellation.
- State: `VERIFIED`
- Scope: the final revision of `.github/workflows/ci.yml`.
- Consequence if false: a permitted run can consume more resources or receive more authority than needed.
- Basis: [Verification](#verification-workflow-bounded).

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

- GitHub's branch protection API accepts `allow_force_pushes:false` and `allow_deletions:false`; `enforce_admins:true` applies the restrictions to administrators. The four required fields allow `null` for checks, PR reviews, and push restrictions, so no unavailable CI check needs to block `main`.
- GitHub's fork approval API accepts `all_external_contributors`, the policy the operator selected. It applies to workflow runs from fork PRs; the existing workflow has no other externally writable trigger.
- GitHub reports `permissions.admin:true` for the authenticated account. The actual setting calls will confirm that the token permits updates.
- Enabling Actions has two controls: the repository Actions permission and the `ci` workflow's `disabled_manually` state. Both remain off until protection, fork approval, and the revised workflow are verified.
- The repository has no self-hosted runners, and the workflow has no artifact upload or cache action. The current matrix uses standard hosted runners, whose use in public repositories GitHub says is free. The plausible misuse is unwanted run churn; approval for every external fork PR is the direct control.

### INVESTIGATE gate

`ESTABLISHED`: the API fields, the two enable controls, and the current run path are known. Actual write permission is checked during implementation, with a failed update leaving Actions disabled.

## Decide

- Set `main` branch protection through GitHub's API with `enforce_admins:true`, `allow_force_pushes:false`, `allow_deletions:false`, and no required checks, PR reviews, or push restrictions. This blocks the two destructive operations for admins too, while direct maintenance and the existing PR remain possible.
- Set the repository fork PR approval policy to `all_external_contributors`. Read it back before enabling Actions.
- Add `permissions: contents: read` to the workflow. Give the Rust test job 30 minutes and the capsule job 10 minutes. Cancel an older run when a new run starts for the same ref. Keep the `pull_request` event, standard hosted runner matrix, and no artifact uploads.
- Verify both live settings and the workflow on the feature branch while Actions remains disabled. Then enable repository Actions and the `ci` workflow. Push a new commit to trigger the existing PR, and observe the resulting jobs, including Windows. If a safeguard fails, leave Actions disabled while correcting it.
- Requiring a CI status check before CI runs would block `main` unnecessarily. Relying on timeouts alone would still let every external PR start a run. Disabling PR runs would prevent the Windows PR verification needed by `doctor-fix-file-mode`.
- Verification uses API readback for branch protection, approval policy, Actions, and workflow state; static workflow inspection and validation; and the hosted run on draft PR #5. No destructive push or fork PR is needed to test the two remote policies.

### DECIDE gate

`ESTABLISHED`: each requested protection has a setting or workflow change and a readback check. The order keeps Actions disabled until the safeguards are in place.

## Implement

- Set `main` protection through GitHub's API. The immediate response and a separate readback both showed `enforce_admins:true`, `allow_force_pushes:false`, `allow_deletions:false`, and no required checks or PR reviews.
- Set fork PR contributor approval to `all_external_contributors`. A separate readback showed that value; Actions remained disabled.
- Committed `.github/workflows/ci.yml` at `19b3cf6`: explicit read-only token permissions, checkout without persisted credentials, per-ref run cancellation, and 30-minute test and 10-minute capsule job limits.
- PyYAML parsed the workflow, and a focused check found the expected permissions, cancellation, job limits, and checkout settings. `actionlint` is not installed; the hosted run will check GitHub's workflow interpretation.
- Pushed `feat/fixup-modes` at `e2dfdfc` before enabling Actions, so GitHub has the revised workflow for PR #5.

### IMPLEMENT gate

`ESTABLISHED`: branch protection and approval policy are live, the revised workflow is committed and pushed, and all can be evaluated while Actions remains disabled.

## Verify

<a id="verification-main-protected"></a>
### Verification: `main-protected`

- Claim: [main-protected](#main-protected).
- Method: Set branch protection, then read its configuration back with a separate GitHub API call on 2026-10-02.
- Evidence considered: GitHub returned `enforce_admins:true`, `allow_force_pushes:false`, `allow_deletions:false`, `required_status_checks:null`, and `required_pull_request_reviews:null`.
- Conclusion: `VERIFIED` for the live branch protection configuration.
- Limitations: This does not attempt a destructive push. An admin with settings access can later change branch protection.

<a id="verification-fork-approval"></a>
### Verification: `fork-approval`

- Claim: [fork-approval](#fork-approval).
- Method: Set the policy, read it back through GitHub's API, and check its documented meaning.
- Evidence considered: GitHub returned `approval_policy:all_external_contributors`. GitHub's repository Actions settings documentation says this policy requires approval for users who are not a repository member or owner. The workflow uses `pull_request`, which the policy covers, and has no `pull_request_target` trigger.
- Conclusion: `VERIFIED` for the policy and current trigger set.
- Limitations: No external fork PR was opened to exercise the approval UI. A later workflow or repository membership change may alter the effective boundary.

<a id="verification-workflow-bounded"></a>
### Verification: `workflow-bounded`

- Claim: [workflow-bounded](#workflow-bounded).
- Method: Parse and inspect the workflow, then compare its local Git blob with the GitHub copy on `feat/fixup-modes`.
- Evidence considered: PyYAML parsed the workflow and the focused check passed. Its jobs use `ubuntu-latest`, `windows-latest`, and `macos-latest`; top-level permissions are `contents: read`; checkout uses `persist-credentials: false`; job timeouts are 30 and 10 minutes; concurrency cancels the older run for the same ref. There is no artifact upload or cache step. GitHub and `git hash-object` both reported workflow blob `baf6f632afeb43d522e6e42274b32f14765d05f3`.
- Conclusion: `VERIFIED` for the pushed workflow contents.
- Limitations: GitHub has not yet interpreted and run the workflow. The hosted run is evaluated under `actions-state`.

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
