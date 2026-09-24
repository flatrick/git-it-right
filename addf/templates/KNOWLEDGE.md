---
evidence:
- anchor: <symbol, stable textual anchor, or other locator>
  role: <implementation\|behavioral-evidence\|configuration\|schema\|other>
  source: "`<repository path>`"
provenance:
  branch: "`<optional branch name>`"
  commit: "`<optional commit SHA>`"
verification:
  method:
  - <source-inspection\|test\|probe\|command\|other>
  verified_at: "`<ISO-8601 datetime with timezone; required when State is
    VERIFIED>`"
---

# Knowledge: `<Concise knowledge subject>`

## Claim

<a id="<stable-claim-anchor>"></a>
-   Claim: <one coherent, falsifiable claim about the current
    repository/system>
-   State: `UNVERIFIED | VERIFIED | REFUTED | DISPUTED`
-   Scope: <revision, environment, inputs, or observation time when
    material>
-   Consequence if false: `<impact>`
-   Basis: <durable reference to the Verification conclusion that establishes
    this State; use `history:` after the originating Task is archived>

## Derivation

<The minimum explanation needed for an independent agent to derive the
claim from the evidence anchors above and the Basis Verification.>

## Limitations

<Optional: material boundary or residual uncertainty. Omit when
unnecessary.>

## Knowledge invariants

-   Knowledge describes validated reality; it does not prescribe
    behavior.
-   The claim must be independently re-derivable from this text, the
    identified evidence anchors, and the Basis Verification.
-   Evidence anchors determine re-derivability; VCS provenance is
    optional historical context, not the validity mechanism.
-   State and Basis follow the same model as every other Claim in the
    framework: Basis names the current applicable Verification, never a
    green command, observation, or assumption alone.
-   `State: VERIFIED` requires `verified_at` with an ISO-8601 datetime
    including timezone.
-   Verification age alone never changes State; only new or
    re-evaluated evidence does.
-   Keep one coherent subject and exactly one Claim per file.
-   In current use, the normalized path relative to the framework root
    is the artifact's logical identity. Beneath `.archive/`, the same
    relative path retains that identity.
-   Publish Knowledge only during terminalization, after task
    Verification and immediately before `Complete` and archival.
-   Do not store historical narrative here; preserve history in
    task/decision/learning records.
-   If evidence changes, is contradicted, or disappears, update State
    and Basis to match instead of leaving a stale `VERIFIED` State
    beside newer evidence. Remove this artifact from current retrieval
    only when no accepted Claim remains to justify it there.
