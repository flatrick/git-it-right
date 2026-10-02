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
`fixup!`, `amend!` and `squash!` commits for `git rebase --autosquash`
(`fixup`, `amend`, `reword`, `squash`); checks repository
hygiene (`doctor`); sets up hooks and configuration (`init`); and explains its
rules (`explain`).

## Boundaries

- `gir` works alongside git and never rewrites existing commits; squashing
  `fixup!`, `amend!` and `squash!` commits is left to `git rebase --autosquash`.

## Specification map

List child specifications in required reading order, or use `NONE` when this
file contains the complete specification.

1. [CLI](spec/cli.md) - invocation, argument errors and exit codes.
2. [Configuration](spec/config.md) - `.girconfig` keys, defaults and errors.
3. [Lint](spec/lint.md) - message rules, safe fixes, output and `gir lint`.
4. [Hooks](spec/hooks.md) - `commit-msg`, `pre-push` and the installed scripts.
5. [Fixup](spec/fixup.md) - target selection, the picker, splitting, and `fixup!`, `amend!` and `squash!` commits.
6. [Doctor](spec/doctor.md) - repository hygiene checks and fixes.
7. [Init](spec/init.md) - generated files and git setup.
8. [Explain](spec/explain.md) - topics and pages.

## Change contract

Every Task records the current specification material it read, its proposed
delta or `NONE`, and the references published at completion. Publish a verified
delta only in the successful terminal checkpoint.
