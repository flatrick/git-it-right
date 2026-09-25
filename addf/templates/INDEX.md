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
  accepted product contract.

## Active Tasks

`NONE`. `<or one entry per active Task: a link to its TASK.md, a concise
objective phrase, and its current State>`

## Ledger

`NONE`. `<or a note that LEDGER.md currently holds unresolved entries>`

## Skills

`<one entry per adopted Skill: a link and the condition from its
description>`

## Rules

`NONE`. `<or one entry per current Rule>`

## Self-improvement

`NONE`. `<or a link to SELF-IMPROVEMENT/ once it holds an entry>`

## Commands

`NONE`. `<or one entry per current Command>`

## Scripts

`NONE`. `<or one entry per framework-owned script beneath scripts/>`

## Knowledge

`NONE`. `<or one entry per promoted Knowledge file>`

## Open Claims

`NONE`. `<or one entry per standing Claim under open-claims/, carried
forward from a terminalized Task>`

## Reusable Evidence

`NONE`. `<or one entry per Evidence artifact reused outside its originating
Task>`

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
