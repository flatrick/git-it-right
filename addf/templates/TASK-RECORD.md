# TASK — `<objective>`

## Resume

**Contract version:** `2`

**State:** `<DEFINE | UNDERSTAND | INVESTIGATE | DECIDE | IMPLEMENT | VERIFY | LEARN | COMPLETED | BLOCKED | IMPOSSIBLE | DEFERRED | ABANDONED | FAILED>`

**State path:** `<DEFINE -> ... -> current State>`

**Resume at:** <specific next action, question, or artifact reference;
use `NONE` in a terminal state>

**Open obligations:** <item and earliest gate it blocks, or item and
explicit deferred checkpoint; use `NONE` when empty>

State must equal the final entry in State path. Append an accepted
transition's destination before beginning work there. The path records
only which transitions occurred; retain material reasons and evidence
in the applicable Task section or owned artifact.

State is the authoritative execution cursor, not proof that a preceding
gate was valid. Do not infer State from artifact presence.

## Owned artifacts

-   <bundle-relative path; Probe, Evidence, Decision, Verification, or
    Learning role>

## Specification impact

- Current contract: <one or more `framework:SPEC.md` or
  `framework:spec/...#...` references, or `NONE` only when no current
  specification exists>
- Proposed delta: <completed behavior this Task proposes to add, change, or
  remove; use `NONE` when the Task does not change the product contract>
- Terminal publication: `PENDING | NONE | <published framework references>`

## Define

### Objective

`<desired state>`

### Success criteria

<a id="<stable-local-claim-anchor>"></a>
#### `<stable local Claim anchor>`

-   Claim: <one coherent, falsifiable success proposition>
-   State: `UNVERIFIED | VERIFIED | REFUTED | DISPUTED`
-   Scope: <revision, environment, inputs, or observation time when
    material>
-   Consequence if false: `<impact>`
-   Basis: <current applicable Verification reference, pending check,
    or why none exists>

### Constraints

-   `<constraint>`

### Material empirical premises

<a id="<stable-local-claim-anchor>"></a>
#### `<stable local Claim anchor>`

-   Claim: <one coherent, falsifiable empirical proposition>
-   State: `UNVERIFIED | VERIFIED | REFUTED | DISPUTED`
-   Scope: <revision, environment, inputs, or observation time when
    material>
-   Consequence if false: `<impact>`
-   Basis: <current applicable Verification reference, pending check,
    or why none exists>

## Understand

### Relevant context

`<minimum context needed>`

### Assumptions

-   <assumption; source; verification state; impact if false when
    material>

### Open questions

-   `<decision-relevant uncertainty>`

### Deferred verification

-   <claim; why it cannot be verified now; earliest checkpoint;
    settling observation; consequence if false; blocked work>

## Investigate

<Record or reference only material probes and evidence. Give each
material uncertainty one disposition: resolved, irrelevant,
`DEFERRED_VERIFICATION`, or accepted residual risk. Do not preserve
disposable noise unless needed to reconstruct a conclusion.>

Record a gate-level `RISK_ACCEPTED` separately. It does not replace a
future `DEFERRED_VERIFICATION` obligation.

## Decide

<Reference the selected approach, supporting evidence, rejected
material alternatives, residual uncertainty, and verification
strategy.>

## Implement

<What was actually changed, any material deviation from the decision,
and the result of each verification checkpoint reached during
implementation. Do not duplicate the decision rationale.>

## Verify

For each evaluated Claim, create a standalone Verification from
`templates/VERIFICATION.md` or use this complete embedded shape:

<a id="verification-<claim-anchor>"></a>
### Verification: `<claim anchor>`

- Claim: <durable reference to the evaluated Claim>
- Method: <how the Claim was tested or observed>
- Evidence considered: <supporting, contradicting, and inconclusive Evidence>
- Conclusion: `VERIFIED | REFUTED | UNVERIFIED | DISPUTED`, with reason
- Limitations: <remaining uncertainty and whether it blocks completion>

A settled Claim's Basis points to the Verification's explicit anchor. A
command, changed file, or inspection belongs under Evidence considered and is
not a direct Basis.

## Learn

### Technical

`<problem/implementation learning>`

### Process

`<what should improve in the way of working>`

If no material learning exists, record that conclusion instead of
manufacturing a learning artifact.

## Retention and promotion

For each Claim considered for Knowledge promotion:

### Promotion: `<claim anchor>`

-   Claim: <link to the evaluated Claim>
-   Will this Claim's validity outlive this Task and inform a future
    decision? `<yes | no>`, with reason
-   Disposition: `<promoted to knowledge/<file> | carried forward to
    open-claims/<file> | not promoted — Task-scoped only>`

If no Claim qualifies, record that conclusion explicitly instead of
omitting this section.

## Archive readiness

<Confirm that the task bundle is self-contained, its internal references
follow `framework:REFERENCES.md`, current consumers do not depend on files that
will move, and external references are supplemental Evidence anchors.>

## Terminal record

### Summary

<Concise final state and reason.>

### Gate basis

<Evidence that permits the terminal transition, including the status
of success-critical claims and deferred verification obligations.>

## Stop record

<Use when State is `BLOCKED`, `IMPOSSIBLE`, `DEFERRED`, `ABANDONED`, or
`FAILED`. Record the prior active state, trigger, evidence, budget or
resources consumed, unresolved obligations and uncertainty, learning,
why continuing was unjustified, and any condition or next action that
could justify a later traversal. Select the outcome using Stewardship's
ordered criteria. Before archival, make State, the final State path entry,
Resume at, obligations, this record, and the current-artifact index agree in
one durable terminal checkpoint.>
