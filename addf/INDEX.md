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

- [fixup-review-20261002](ledger/fixup-review-20261002.md) - twelve edge
  cases in `gir fixup` and `--split` and the order to fix them; nine open
  (Q1, Q2, Q8, Q11 taken).

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
- [OS-agnostic code](rules/os-agnostic-code.md) — all code the repository
  produces, including scripts inside addf Tasks, behaves the same on Windows
  and Linux.

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

- [gui-clients-have-no-tty](open-claims/gui-clients-have-no-tty.md) - GUI
  git clients run gir without a terminal, so they never see its prompt;
  unverified.

## Reusable Evidence

- [b001-b002-windows-verification](evidence/b001-b002-windows-verification.md) -
  B-001 and B-002 are fixed on Windows; verified.
- [b001-b002-linux-reverification](evidence/b001-b002-linux-reverification.md) -
  the fix-b001-b002 Linux Claims repeated with committed output; verified.
- [gir-v1-windows-verification](evidence/gir-v1-windows-verification.md) -
  gir's tests and clippy pass on Windows; verified. macOS is assumed, not
  verified.

## Index invariants

-   This file contains no unique semantic content; every entry links to
    its authoritative source.
-   Use an explicit `NONE` for an empty category instead of omitting the
    heading.
-   Update this index in the same pass that adds, terminalizes, promotes,
    or corrects a current artifact. Work control owns the active-Task and
    Ledger entries; Stewardship owns terminal removal, archived threads,
    promotion, and correction.
-   A stale entry is a discovery defect in this file, not evidence
    against the artifact it names.
-   Do not copy another repository's populated `INDEX.md` during adoption.
    Create this file fresh and list only what exists at the destination.
-   The same applies to `SPEC.md`, `spec/`, and `ledger/`. They are instance
    data, never source capsule content.
