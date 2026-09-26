# Self-improvement: `unverified-statements-around-verified-work`

**Date (UTC):** `2026-09-26T13:28:53Z`

## Trigger

In the session that settled `b001-b002-windows` and `gir-v1-windows-macos`
(commits `3ba3098` to `96bc952` on `fix/b001-b002`), the agent treated the
central Claims rigorously: it built controls from the pre-fix `839a991`,
discarded a void run, checked that archive findings already existed at
`42b84ad`, made each new test fail against old code, and refused to mark
macOS `VERIFIED` on the operator's assumption. The operator reviewed the
session and found that statements around those Claims did not get the same
treatment. The agent agreed. Each item below names the statement, where it
is, and the check that would have settled it. Paths and line numbers refer
to commit `012f126`; later commits corrected those records.

1.  **CI was assumed to run.** In chat the agent said a push to PR #2
    "triggers CI". Two committed records rest on the same assumption:
    `evidence/gir-v1-windows-verification.md:72` ("a passing run there
    would be evidence") and `evidence/b001-b002-windows-verification.md:97`
    ("not yet on the `windows-latest` CI runner"). `gh api
    repos/flatrick/git-it-right/actions/permissions` returns
    `{"enabled":false}`, and `gh run list` returns no runs, so no CI run
    was pending. The operator then had the `ci` workflow disabled as well.
2.  **A guess followed by a manual inspection was offered as a
    safeguard.** In chat the agent said `check-capsule` "probably" did not
    check `.archive/open-claims/` and that it had checked those Basis links
    "by eye". The answer was in `load_context` in `scripts/check-capsule`,
    which the agent had already opened. `skills/evidence-and-verification.md`
    counts manual inspection as Evidence, not a Basis. The gap was fixed in
    `5559ac0` only after the operator objected.
3.  **An unconfirmed cause was written into an Observation.**
    `evidence/b001-b002-windows-verification.md:61-63` says a void B-002
    run happened because "`Set-Content -NoNewline` failed in that shell".
    Later in the same session, in the same PowerShell tool,
    `Set-Content -NoNewline` worked. The void run is real, since the file
    was never created, but its cause is unknown. The record states a guess
    as a fact inside an Observation, which the Exit check forbids
    ("Observations contain no interpretation").
4.  **A statement about another repository was not checked.**
    `SELF-IMPROVEMENT/20260926T131042Z-check-capsule-checks-archived-open-claims.md:20`
    and commit `5559ac0` say the change "has not been made upstream in
    `agent-driven-development-template`". The agent never looked at that
    repository.
5.  **The evidence cannot be re-derived from the repository.** Both
    Verifications cite logs under the main checkout's `.scratch/`, for
    example `evidence/b001-b002-windows-verification.md:40` and `:59`, and
    `evidence/gir-v1-windows-verification.md:41`. `.scratch/` is
    gitignored and exists on one machine. The records say "local, not
    committed", but a reviewer of either Verification sees only the
    agent's summary of what it observed.
6.  **Verifications were written after the fact with no new
    observation.** To clear two `CLAIM_BASIS` findings, `bd83b18` added
    `verification-b001-reproduces` and `verification-b002-reproduces`
    (`.archive/tasks/fix-b001-b002/TASK.md:214-237`). They restate runs
    already recorded in that Task's Investigate section, and nothing was
    re-run. `check-capsule` then passed. The commit message says "No
    conclusion changes" but does not say that no observation was repeated.
7.  **Platform behavior was stated from memory.** In chat the agent said an
    administrator token does not bypass an explicit deny ACE without
    backup privileges. It did not check Microsoft documentation, which the
    operator's own instructions require for Microsoft behavior.

The pattern: rigor followed the Claim records and faded in Remaining
uncertainty, Limitations, Context, commit messages, self-improvement
entries and chat. Those are the places a later reader acts on without a
State or Basis to check.

Gaps in addf that these items point to, recorded for a later decision:

-   The invariant in `CORE.md` and the Exit check in
    `skills/evidence-and-verification.md` govern Claims and Observations.
    No rule says whether a factual statement in Limitations, Remaining
    uncertainty, Context, a commit message or a self-improvement entry must
    be verified, labeled as an assumption, or left out (items 1, 3, 4).
-   Nothing asks that the raw observations behind a Verification outlive
    the machine they were made on (item 5).
-   `check-capsule` checks the structure of a Verification, not that it
    records a new observation. Pointing an embedded Verification at older
    notes satisfies it (item 6). A passing capsule check is itself a green
    result, which `CORE.md` says is never a verified Claim.
-   addf has no guidance for statements an agent makes in conversation.
    Those are what the operator acts on in real time (items 1, 2, 7).

## What changed

`NONE`. Friction only. The records named in items 1, 3, 4 and 6 were
uncorrected when this entry was written and were corrected in the next
commit.

## Why

The operator treats proof as a base pillar of addf. This session shows
that an agent can follow addf's Claim mechanics to the letter and still
publish unverified statements beside them, which the checker cannot
detect. These are observations from one session, kept so they can be
weighed when addf's own rules are next revised.
