# gir hooks

## Subspecifications

`NONE`.

## commit-msg

<a id="req-hooks-commit-msg-fixes"></a>
**commit-msg-fixes.** `gir hook commit-msg FILE` SHALL apply every safe fix to the message file in place, report each fix line on stderr, and exit `0` when no rejection remains, so `git commit` records the fixed message.

<a id="req-hooks-commit-msg-rejects"></a>
**commit-msg-rejects.** When a rejection remains, it SHALL report the rejection lines with label `commit rejected` and exit `1`, so `git commit` aborts.

<a id="req-hooks-commit-msg-keeps-tail"></a>
**commit-msg-keeps-tail.** When it rewrites the file, it SHALL keep comment lines, the scissors line and the `commit -v` diff after the fixed message, and SHALL NOT lint them.

<a id="req-hooks-commit-msg-autosquash"></a>
**commit-msg-autosquash.** It SHALL accept `fixup! `, `squash! ` and `amend! ` messages so `git commit --fixup` works.

## pre-push

<a id="req-hooks-pre-push-scope"></a>
**pre-push-scope.** `gir hook pre-push REMOTE URL` SHALL read git's `local-ref local-sha remote-ref remote-sha` lines from stdin and lint each commit being pushed that the remote does not already have: `remote-sha..local-sha` for an existing remote branch, or commits not reachable from any `REMOTE` remote-tracking ref for a new branch or when that range fails. A stdin line without exactly four fields SHALL be skipped.

<a id="req-hooks-pre-push-deletes"></a>
**pre-push-deletes.** A line whose local SHA is all zeros (a branch deletion) SHALL be skipped.

<a id="req-hooks-pre-push-rejects"></a>
**pre-push-rejects.** It SHALL reject commits with any lint rejection, unsquashed `fixup! `, `squash! ` or `amend! ` commits with rule `fixup-unsquashed`, and commits a safe fix would change with rule `fix-pending`, with label `push rejected` preceded by the short SHA, and exit `1` so `git push` aborts; it SHALL NOT modify any commit.

## Installed hook scripts

<a id="req-hooks-shim-delegates"></a>
**shim-delegates.** The `.githooks/commit-msg` and `.githooks/pre-push` scripts SHALL run `gir hook NAME` with git's arguments when `gir` is on `PATH`.

<a id="req-hooks-shim-missing-warn"></a>
**shim-missing-warn.** When `gir` is not on `PATH` and `gir.hookMissing` is not `fail`, the scripts SHALL print `gir: not installed, NAME check skipped` on stderr and exit `0`.

<a id="req-hooks-shim-missing-fail"></a>
**shim-missing-fail.** When `gir` is not on `PATH` and `.girconfig` sets `gir.hookMissing` to `fail`, the scripts SHALL print `gir: not installed, NAME check blocked (gir.hookMissing=fail)` on stderr and exit `1`.

## Latency

<a id="req-hooks-commit-msg-latency"></a>
**commit-msg-latency.** The median of 15 runs of `gir hook commit-msg` on a message that needs fixes SHALL take at most 750 ms.

## Module invariants

- This file is reachable from `SPEC.md` through one ordered parent link.
- This file contains current requirements only.
- This file does not require archived material to define current behavior.
