# Self-improvement: git metadata writes need approval

**Date (UTC):** `2026-10-23T17:59:56Z`

## Trigger

In the isolated worktree, the first `git commit` failed because the sandbox denied writes to `.git/index.lock`, while repository files were writable. The same Git operation succeeded after requesting elevated access.

## What changed

`NONE`.

## Why

Work control requires separate commits for Task state transitions. In this managed workspace, Git metadata writes may need explicit sandbox escalation even after the user selects an isolated worktree. The implementation and Task history are in [the SourceGit template Task](../tasks/sourcegit-commit-template/TASK.md).
