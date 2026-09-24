# Probe: `<question summary>`

## Question

<What material uncertainty is being resolved?>

## Investigates

-   Claim: <relative path and local anchor, when a Claim already exists>
-   Open question: <use when the Probe precedes a candidate Claim>

## Hypothesis

<Current explanation or expected behavior.>

## Alternatives

-   `<plausible competing explanation>`

Omit only when alternatives are immaterial.

## Expected observations

-   `<what should be observed if the hypothesis is correct>`
-   `<observation that helps distinguish the hypothesis from alternatives>`

## Method

<Smallest reliable experiment/inspection that can answer the
question.>

## Budget

-   Time: `<optional>`
-   Attempts: `<optional>`
-   Resources: `<optional>`

## Observations

<a id="<local-observation-anchor-when-referenced>"></a>
### `<local anchor when referenced>`

-   Fact: <what was directly observed>
-   Source and method: <relative source or command and relevant inputs>
-   Context: <environment, revision, or observation time when material>
-   Limitations: <what the Observation does not establish>

## Interpretation

<What the observations imply and why.>

## Evidence relationships

### `<relative Claim reference>`

-   Observation: <relative Observation reference>
-   Relationship: `SUPPORTS | CONTRADICTS | INCONCLUSIVE`
-   Reason: <why the Observation bears on the Claim>
-   Scope: <any limit on the relationship>

## Contradictory evidence

-   <material `CONTRADICTS` relationship and how it affects the
    interpretation>

## Confidence

`LOW | MEDIUM | HIGH`

<Optional concise reason.>

## Consequence

<What decision or plan changes because of this result?>

## Residual uncertainty

<What remains unknown and whether it blocks progress?>

## Disposition

<Resolved, irrelevant, `DEFERRED_VERIFICATION`, accepted as residual
risk by reference to a gate record, or still open.>

For accepted risk, reference the gate record that names the authorized
actor, Claim, consequence, scope, and permitted work. `RISK_ACCEPTED`
does not replace a future `DEFERRED_VERIFICATION` obligation.

## Probe invariants

-   A passing/green outcome is an observation, not automatic proof of
    the hypothesis.
-   Prefer discriminating probes that make competing explanations
    predict different observations.
-   Record observations separately from interpretation.
-   Reference the Claim and Observation directly when recording an
    Evidence relationship.
-   Actively consider falsifying evidence when the impact of being wrong
    is material.
-   Stop and reassess when the probe budget is exhausted or the probe is
    no longer producing useful information.
