# Specification: gir (git-it-right)

This specification states the completed, accepted behavior of this repository's
system. It is the root of the current product contract.

## Authority

`SPEC.md` and the specification documents reachable through the ordered map
below define what this repository currently intends to provide. Active Tasks
contain proposed changes. Archived Tasks provide provenance only.

The specification is a requirement, not empirical proof that the
implementation conforms. Use Knowledge and Verification for observed facts.

## Purpose

`gir` is a single cross-platform CLI binary (Windows, Linux, macOS) that keeps
git usage honest for humans and coding agents alike. It lints commit messages
against Conventional Commits 1.0.0, fixing safe mistakes automatically and
rejecting the rest with actionable messages (`lint`, `hook`); creates
`fixup!` commits for `git rebase --autosquash` (`fixup`); checks repository
hygiene (`doctor`); sets up hooks and configuration (`init`); and explains its
rules (`explain`).

## Boundaries

- `gir` works alongside git and never rewrites existing commits; squashing
  `fixup!` commits is left to `git rebase --autosquash`.

## Specification map

List child specifications in required reading order, or use `NONE` when this
file contains the complete specification.

`NONE`.

## Change contract

Every Task records the current specification material it read, its proposed
delta or `NONE`, and the references published at completion. Publish a verified
delta only in the successful terminal checkpoint.
