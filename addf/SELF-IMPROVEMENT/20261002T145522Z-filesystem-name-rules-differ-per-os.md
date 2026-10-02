# Self-improvement: `filesystem-name-rules-differ-per-os`

**Date (UTC):** `2026-10-02T14:55:22Z`

## Trigger

During `doctor-fix-file-mode`, keeping track of how macOS, Windows and Linux filesystems treat file names was a source of friction.
Each difference surfaced only after a round trip to another system, never during local work on Linux:

-   On Windows, four doctor tests failed because their shared fixture inserted names that Git's `core.protectNTFS` rejects; `3e69631` disabled it for the fixture, as an older test already did.
-   On macOS, hosted CI rejected a working-tree file name containing byte `0xe9` with `Illegal byte sequence`, first in `doctor_fix_sets_exec_bit_on_a_non_utf8_hook_name` (`58bea3b`, made index-only) and then in `split_traces_non_utf8_file_names` (`e507d42`, limited to Unix systems other than macOS).

`rules/os-agnostic-code.md` names Windows and Linux and says other Unix-like systems "count as covered by Linux unless a difference is known".
macOS is such a difference for file names, and nothing in addf records which name rules differ per system, so each one was rediscovered from a failing run.

## What changed

`NONE`. This entry records friction only.

## Why

Each difference cost a push, a hosted CI run or a manual Windows run, and a return from `VERIFY` to `IMPLEMENT`.
See `history:tasks/doctor-fix-file-mode/TASK.md`, sections "Windows test report and reassessment", "Hosted CI and macOS fixture reassessment" and "Hosted CI green".
