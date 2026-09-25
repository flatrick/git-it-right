# Verification: `<claim summary>`

## Claim

**Reference:** <relative Claim reference, or `LOCAL` when this
Verification introduces a narrower Claim>

<a id="<local-claim-anchor-when-owned>"></a>
### `<local anchor when this Verification owns the Claim>`

-   Claim: <one coherent, falsifiable proposition>
-   State: `UNVERIFIED | VERIFIED | REFUTED | DISPUTED`
-   Scope: <revision, environment, inputs, or observation time when
    material>
-   Consequence if false: `<impact>`
-   Basis: <this Verification conclusion after it exists>

Omit the local Claim record when the referenced Claim is sufficient.

## Method

<How the claim will be tested/observed.>

## Expected observations

-   `<expected result>`
-   <where material, observation that distinguishes the intended
    mechanism from alternatives>

## Observed results

### `<local Observation anchor when referenced>`

-   Fact: `<what was directly observed>`
-   Source and method: <relative source or command and relevant inputs>
-   Context: <environment, revision, or observation time when material>
-   Limitations: <what the Observation does not establish>

## Evidence considered

-   <relative Evidence relationship or inline Observation and its
    relationship to the Claim; include independent, contradictory, and
    inconclusive results when material>

## Contradictory and inconclusive evidence

-   <relationship reference and its effect on the conclusion, or
    `NONE`>

## Conclusion

<a id="conclusion"></a>

**Result:** `VERIFIED | REFUTED | UNVERIFIED | DISPUTED`

<Concise explanation.>

## Remaining uncertainty

<What remains unproven and whether it is blocking.>

## Verification invariants

-   Test/command success is not synonymous with claim verification.
-   Verification is the only activity that settles Claim state.
-   Verify the intended invariant, not merely the current example.
-   A green result is insufficient when plausible unintended mechanisms
    can produce the same result.
-   Material unexplained contradictory evidence prevents a `VERIFIED`
    conclusion.
-   Use `REFUTED` only when adequate evidence establishes that the claim
    does not hold.
-   Use `DISPUTED` when material supporting and contradicting evidence
    remains unresolved.
-   If evidence supports only part of a Claim, narrow or split the Claim
    and leave the broader Claim `UNVERIFIED`. If adequate evidence
    refutes a required part, the broader Claim is `REFUTED`.
-   Before reevaluating a previously settled Claim, set its State to
    `UNVERIFIED` and record the pending check in Basis.
-   Write this Verification before publishing its conclusion in the
    evaluated Claim's State and Basis.
-   When embedded beneath a Task's `## Verify` heading, retain the Claim,
    Method, Evidence considered, Conclusion, and Limitations roles and give the
    conclusion a stable anchor.
-   Result records this evaluation event. Claim State and Basis record
    the current applicable Verification. Do not rewrite an earlier
    Result after a later Verification.
