# Rule: os-agnostic-code

**SHALL:** Every piece of code this repository produces — product source, tests, build and helper scripts, generated hooks, and the scripts and probes inside addf Tasks — behaves the same on Windows and on Linux.
Paths, temporary locations, null devices, line endings, executable bits, shells and external tools come from the platform (for example Python's `tempfile` and `os.devnull`, Rust's `std::env::temp_dir` and `std::path`), never from one operating system's fixed names such as `/tmp`, `/dev/null` or `C:\`.
Where behavior must differ, both branches are written and both are exercised.

**Reason:**
gir is used from Windows and Linux shells and from GUI git clients on both, and code that only works on one fails silently for half its users.

**Instead:** N/A — this is a `SHALL`, not a `SHALL NOT`.

**Scope:**
All code added or changed in this repository from 2026-10-02 on.
Other Unix-like systems count as covered by Linux unless a difference is known.
Archived Tasks are history and are not rewritten to satisfy this rule.

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
