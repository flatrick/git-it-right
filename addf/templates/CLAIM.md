# Claim: `<short name>`

## Claim

<a id="<stable-claim-anchor>"></a>
-   Claim: <one coherent, falsifiable proposition>
-   State: `UNVERIFIED | VERIFIED | REFUTED | DISPUTED`
-   Scope: <revision, environment, inputs, or observation time when
    material>
-   Consequence if false: `<impact>`
-   Basis: <current applicable Verification reference, pending check, or
    why none exists>

## Owning Task

-   <relative link to the Task (and its Material empirical premises or
    Success criteria section) that this Claim belongs to; a standalone
    Claim file is not self-explanatory without that objective/Constraints
    context>

## Claim invariants

-   One coherent, falsifiable proposition per file — the same splitting
    rule as any other Claim in the framework
    (`skills/evidence-and-verification.md`'s "split a Claim" guidance).
-   Follows the same State/Basis model as every other Claim: `UNVERIFIED`
    until a Verification settles it; Basis names the applicable
    Verification, never a green result, authority statement, or accepted
    risk alone.
-   Unlike `templates/KNOWLEDGE.md`, this is not a promotion target and
    carries no `verified_at`/evidence-anchor/provenance frontmatter — it
    stays owned by an active Task and may sit at any State
    (`UNVERIFIED`, `DISPUTED`, `REFUTED`, or `VERIFIED`) while that Task
    is still open. Promote a `VERIFIED` Claim to Knowledge only through
    Stewardship's normal promotion step, immediately before that Task's
    terminal transition — this file does not survive archival on its own
    authority.
-   A Claim carried forward past this Task's own terminalization becomes
    a standing Claim under `open-claims/` via `templates/OPEN-CLAIM.md`,
    not a continued `claims/` file — see Stewardship's "Carry forward open
    Claims" step.
-   Use this file instead of an inline Claim record only once a Task's
    Material empirical premises has grown large enough that
    `RECOMMENDATIONS.md`'s "Split large Claim sets into a `claims/`
    subfolder" entry applies. A Task with only a few Claims should keep
    them inline; this template exists for scale, not as the default.
-   Referenced from the owning Task's Material empirical premises section
    by a pointer to the containing directory (and, when useful, a short
    per-file index line), not duplicated inline.
