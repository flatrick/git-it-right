# Self-improvement: `os-agnostic-code-rule`

**Date (UTC):** `2026-10-02T09:26:42Z`

## Trigger

The acceptance scripts of the Task `fixup-fixes-acceptance` used `/dev/null` for `GIT_CONFIG_GLOBAL` and matched temporary paths with a hard-coded `/tmp/`, so they would not have worked on Windows.
The operator asked that all code this repository produces work on Windows and Linux, as repository guidance.

## What changed

`rules/os-agnostic-code.md` is a new Rule: every piece of code the repository produces, including scripts and probes inside addf Tasks, behaves the same on Windows and Linux, and takes paths, temporary locations and similar from the platform instead of one operating system's fixed names.
`INDEX.md` lists it.
The CI workflow already runs on `ubuntu-latest` and `windows-latest` (and `macos-latest` for the main job); scripts inside addf Tasks are not run there, so for them the rule is checked only by review.

## Files touched

-   `rules/os-agnostic-code.md` — the new Rule.

## Why

The operator's instruction on 2026-10-02, during `fixup-fixes-acceptance`.
