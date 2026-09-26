# Index

A manually maintained, derived view of current framework and repository
artifacts. It is not authoritative: if an entry here disagrees with the
artifact it names, the artifact is correct and this file is stale. Paths
below are relative to the framework root.

This index excludes templates, supporting framework infrastructure
(`CORE.md`, this file, `REFERENCES.md`, `RECOMMENDATIONS.md`, `ADOPT.md`),
terminal Tasks, and everything beneath `.archive/`. A terminal Task's history
is available through its archived bundle, not through this index.

## Specification

- [Current specification](SPEC.md) - the root of this repository's completed,
  accepted product contract, with eight child modules under `spec/`.

## Active Tasks

`NONE`. No Task is currently active.

## Ledger

`NONE`. `LEDGER.md` holds no entries.

## Skills

- [Work control](skills/work-control.md) — start or resume a Task, or
  transition its lifecycle State.
- [Evidence and verification](skills/evidence-and-verification.md) — a
  material Claim must be established, contradicted, or deferred.
- [Stewardship](skills/stewardship.md) — `Complete`, a Stop outcome,
  archival, Knowledge promotion, or correcting misleading current material.

## Rules

- [Self-improvement log](rules/self-improvement-log.md) — record, in the
  same pass, any change to addf's own framework mechanics or any friction
  noticed while using it.

## Self-improvement

- [Self-improvement log](SELF-IMPROVEMENT/) — one entry per framework
  change or friction noticed, per
  [Self-improvement log](rules/self-improvement-log.md).

## Commands

`NONE`. No Command has been added yet.

## Scripts

- [Capsule checker](scripts/check-capsule) - optional structural checker for
  current framework truth, portable roots, and explicitly requested historical
  validation.

## Knowledge

`NONE`. No Knowledge has been promoted yet.

## Open Claims

- [gir-v1-windows-macos](open-claims/gir-v1-windows-macos.md) - gir v1's
  tests and clippy pass on Windows and macOS; unverified, awaiting the
  operator's Windows run.
- [b001-b002-windows](open-claims/b001-b002-windows.md) - B-001 and
  B-002 are fixed on Windows; unverified, awaiting the operator's Windows run.

## Reusable Evidence

`NONE`. No Evidence artifact is reused outside its originating Task.

## Index invariants

-   This file contains no unique semantic content; every entry links to
    its authoritative source.
-   Use an explicit `NONE` for an empty category instead of omitting the
    heading.
-   Update this index in the same pass that adds, terminalizes, promotes,
    or corrects a current artifact. Work control owns the active-Task and
    Ledger entries; Stewardship owns terminal removal, promotion, and
    correction.
-   A stale entry is a discovery defect in this file, not evidence
    against the artifact it names.
-   Do not copy another repository's populated `INDEX.md` during adoption.
    Create this file fresh and list only what exists at the destination.
-   The same applies to `SPEC.md`, `spec/`, and `LEDGER.md`. They are instance
    data, never source capsule content.
