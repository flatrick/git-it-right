# Command: `<name>`

## Purpose

<One stable semantic repository capability.>

## Invocation

`<command>`

## Inputs

-   <arguments/environment, if any>

## Outputs

-   `<semantic result>`

## Exit semantics

-   `0`: `<meaning>`
-   non-zero: `<meaning>`

Execution success and domain result are separate when applicable. For
example, a probe may execute successfully while contradicting its
hypothesis.

## Side effects

<What this command changes, if anything.>

## Safety

`READ | WRITE | EXTERNAL`

-   **READ** — no meaningful mutation.
-   **WRITE** — repository-local mutation.
-   **EXTERNAL** — changes state outside the repository.

## Prerequisites

-   `<required state/dependency>`

## Repeatability

`IDEMPOTENT | REPEATABLE | NON-REPEATABLE`

## Invariants

-   Expose semantics, not implementation details.
-   One command has one clear purpose.
-   Prefer machine-observable results and stable exit behavior.
-   Fail informatively enough for the caller to choose the next action.
-   Repository commands should be usable by humans without an agent.
