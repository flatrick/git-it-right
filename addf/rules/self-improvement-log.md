# Rule: self-improvement-log

**SHALL:** Record, under `SELF-IMPROVEMENT/`, in the same pass it is
noticed or made: (a) every change to addf's own framework mechanics —
`templates/`, `scripts/` (e.g. `check-capsule`), `skills/`, `rules/`,
`RECOMMENDATIONS.md`, `CORE.md`, `REFERENCES.md`, or `ADOPT.md` — and (b)
any friction detected while using addf, whether or not it was fixed in the
same pass. Name each entry `<utc-datetime>-<topic>.md`, with the UTC
datetime in `YYYYMMDDTHHMMSSZ` form (e.g. `20260915T113630Z`) and `<topic>`
a short kebab-case slug. Use `templates/SELF-IMPROVEMENT.md`'s shape.
Documenting here is the whole obligation; deciding whether and how far to
pull any entry further — into a Skill, a Rule, Knowledge, or upstream into
another repository's capsule — is a separate human call, made when someone
works on the framework directly, not something to track, flag, or decide
from inside a Task.

**Reason:**
A record kept only inside a Task-scoped Learning file goes dark once its
owning Task terminalizes and archives — the same visibility problem those
files describe applies to the record itself. A dedicated, top-level,
never-archived log fixes that specifically for changes to addf's own
mechanics, independent of any one Task's lifecycle.

**Instead:** N/A — this is a `SHALL`, not a `SHALL NOT`.

**Scope:** Any change to files that constitute addf's own generic mechanics
in this repository (`templates/`, `scripts/`, `skills/`, `rules/`,
`RECOMMENDATIONS.md`, `CORE.md`, `REFERENCES.md`, `ADOPT.md`). Does not
apply to Task-instance content — `tasks/`, `SPEC.md`'s product content,
`ledger/` threads, or `knowledge/` — which is already tracked through the
Task lifecycle and Ledger.

## Invariants

-   One rule expresses one constraint.
-   Use normative `SHALL` or `SHALL NOT` wording.
-   The behavior and violation must be observable.
-   Keep the reason concise. Detailed rationale belongs in a Decision or
    the applicable design artifact. Reference Knowledge only for
    validated repository facts.
-   A negative rule should provide replacement behavior when practical.
-   Do not embed a procedure; reference the applicable skill instead.
-   Do not duplicate mechanics already enforced by tooling.
