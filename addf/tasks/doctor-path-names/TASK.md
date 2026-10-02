# TASK — `gir doctor reads file names exactly as git stores them`

## Resume

**Contract version:** `2`

**State:** `DECIDE`

**State path:** `DEFINE -> UNDERSTAND -> INVESTIGATE -> DECIDE`

**Resume at:** Ask the operator the two open questions under Understand, then record the design under Decide.

**Open obligations:** Every success criterion below is `UNVERIFIED` and blocks `VERIFY`.

## Owned artifacts

-   `ledger.md` - the questions that shaped this Task, with the operator's answers.
-   `probe_doctor_names.py` - Probe: `gir doctor` (and `--fix` where relevant) on repositories with each kind of file name; Windows-impossible names are Unix-only.
-   `logs/probe-head-4da58e7-20261002-1221.log` - Evidence: that probe at `4da58e7`.
-   `logs/probe-fix-nonutf8-4da58e7-20261002-1221.log` - Evidence: the full `gir doctor --fix` output for a non-UTF-8 hook name at `4da58e7`.

## Specification impact

- Current contract: `framework:spec/doctor.md#req-doctor-case-collision`, `framework:spec/doctor.md#req-doctor-windows-names`, `framework:spec/doctor.md#req-doctor-exec-bit`, `framework:spec/doctor.md#req-doctor-ignore-rules`
- Proposed delta: To be settled in `DECIDE` from what the investigation confirms; `NONE` if no requirement changes.
- Terminal publication: `PENDING`

## Define

### Objective

`gir doctor` reads every file name exactly as git stores it, so its checks judge real names, not git's quoted display form.

### Success criteria

<a id="c1-suspicions-settled"></a>
#### `c1-suspicions-settled`

-   Claim: Each suspicion is probed on the build at this Task's start and recorded as confirmed or refuted, with evidence: (a) names git quotes (double quote, backslash, tab, newline) reach the index checks in quoted form; (b) non-ASCII names reach the ignore-rule check in quoted form; (c) `windows_unsafe` does not treat control characters as unsafe; (d) non-UTF-8 names are mangled.
-   State: `VERIFIED`
-   Scope: Linux, git 2.56.0, `4da58e7`.
-   Consequence if false: a fix is made for a defect that does not exist, or a real one is missed.
-   Basis: [Verification](#verification-c1-suspicions-settled).

<a id="c2-real-names"></a>
#### `c2-real-names`

-   Claim: For every confirmed defect, `gir doctor` reports the real name and decides from it: `case-collision`, `windows-names`, `exec-bit` and the ignore-rule detection work for names with spaces, double quotes, backslashes, tabs, non-ASCII characters and, on Unix, non-UTF-8 bytes.
-   State: `UNVERIFIED`
-   Scope: Linux, git 2.56.0, this branch.
-   Consequence if false: doctor misjudges or misreports repositories with such names.
-   Basis: pending check.

<a id="c3-tests"></a>
#### `c3-tests`

-   Claim: Regression tests cover each confirmed defect and fail on the build at this Task's start; `cargo test` and `cargo clippy --all-targets -- -D warnings` pass on the final revision; tests for names Windows cannot hold are Unix-only with the reason stated.
-   State: `UNVERIFIED`
-   Scope: Linux, final revision of this branch.
-   Consequence if false: the fix regresses unnoticed.
-   Basis: pending check.

<a id="c4-spec"></a>
#### `c4-spec`

-   Claim: If a requirement changes (for example control characters in `windows-names`), the delta is recorded in `DECIDE` and published at completion; otherwise Specification impact stays `NONE`.
-   State: `UNVERIFIED`
-   Scope: this branch.
-   Consequence if false: the specification no longer describes doctor.
-   Basis: pending check.

### Constraints

-   Work in worktree `.worktrees/fixup-modes` on branch `feat/fixup-modes` (operator's answer in `ledger.md`).
-   Code and test scripts follow `rules/os-agnostic-code.md`.
-   Out of scope: other `doctor` checks.
-   No other Task is active.

### Material empirical premises

<a id="p1-suspicions-from-reading"></a>
#### `p1-suspicions-from-reading`

-   Claim: The four suspicions in `c1-suspicions-settled` come from reading `src/cmd/doctor.rs` at `76de5c5`, not from running it.
-   State: `VERIFIED`
-   Scope: `76de5c5`.
-   Consequence if false: the investigation starts from a wrong reading.
-   Basis: [Verification](#verification-p1-suspicions-from-reading).

### DEFINE gate

`ESTABLISHED`: on 2026-10-02 the operator explicitly agreed that the objective, success criteria and order are right, as written here (`ledger.md`).

## Understand

### Relevant context

Findings at `4da58e7` (`logs/probe-head-4da58e7-20261002-1221.log`, `logs/probe-fix-nonutf8-4da58e7-20261002-1221.log`):

-   **(a) Quoted names in the index checks: confirmed.** `index_checks` reads `git -c core.quotePath=false ls-files -s` as text, so a name with a double quote, backslash or tab arrives C-quoted. `exec-bit` misses `.githooks/pre "x"` entirely, and `--fix` leaves it `100644`. `windows-names` and `case-collision` still flag such names, but only by accident (the quoting adds `"` and `\`), and report them in quoted form (`"say \"hi\".txt"`). A space is not quoted and works.
-   **(b) Non-ASCII names in the ignore-rule check: confirmed.** `file_checks` reads `git ls-files --cached --others --exclude-standard` without `core.quotePath=false`, so `Ångström.csproj` arrives quoted, ends with `"`, and `bin/` and `obj/` are not suggested, where `Angstrom.csproj` gets them. `case-collision` on `Å.txt`/`å.txt` works, because the index checks do set `core.quotePath=false`.
-   **(c) Control characters: latent.** `bell<BEL>.txt` and `tab<TAB>here.txt` are flagged today only because their quoted form contains `\`; `windows_unsafe` itself has no rule for control characters, and the spec's `windows-names` list has none. Reading real names would silently stop flagging them.
-   **(d) Non-UTF-8 names: confirmed.** `git::run` decodes lossily, so `.githooks/hook<E9>` is reported as `.githooks/hook\u{fffd}`, and `gir doctor --fix` stops with `fatal: Unable to process path` and exit `2` after already applying the earlier fixes. `caf<E9>.txt` is not flagged by `windows-names`.

### Assumptions

-   Windows does not allow the characters 1 to 31 in file names. Source: memory of Microsoft's file naming rules; not verified here (no Microsoft documentation tool available).
-   Git for Windows cannot check out a path whose bytes are not valid UTF-8. Source: inference from Windows storing names as UTF-16; not verified.

### Open questions

-   Should `windows-names` cover control characters, and names that are not valid UTF-8? Both change the specification and rest on the unverified assumptions above; for the operator in `DECIDE`.
-   How should a report show a name with a control character or bytes that are not UTF-8, which cannot be printed as they are on one line? For the operator in `DECIDE`.

### Deferred verification

-   The two Windows assumptions; earliest checkpoint: the operator's Windows testing.

### UNDERSTAND gate

`ESTABLISHED`: every suspicion is settled by observation, and the remaining questions are decisions.

## Investigate

The probe settled every factual uncertainty (Understand). Dispositions:

-   The two Windows assumptions: `DEFERRED_VERIFICATION` to the operator's Windows testing (Deferred verification).
-   Whether `windows-names` covers control characters and non-UTF-8 names, and how reports show unprintable names: decisions for the operator in `DECIDE`, not uncertainties a probe can settle.

### INVESTIGATE gate

`ESTABLISHED`.

## Decide

`PENDING`

## Implement

`PENDING`

## Verify

<a id="verification-c1-suspicions-settled"></a>
### Verification: `c1-suspicions-settled`

- Claim: [c1-suspicions-settled](#c1-suspicions-settled)
- Method: `probe_doctor_names.py`, and a full `gir doctor --fix` run for a non-UTF-8 hook name, on the build at `4da58e7`.
- Evidence considered: `logs/probe-head-4da58e7-20261002-1221.log` and `logs/probe-fix-nonutf8-4da58e7-20261002-1221.log`, summarised under Understand: (a) confirmed, (b) confirmed, (c) latent rather than visible today, (d) confirmed, including a failing `--fix`.
- Conclusion: `VERIFIED`: every suspicion is recorded as confirmed or not, with evidence.
- Limitations: Linux, git 2.56.0; the Windows side of (c) and (d) is assumed, not observed.

<a id="verification-p1-suspicions-from-reading"></a>
### Verification: `p1-suspicions-from-reading`

- Claim: [p1-suspicions-from-reading](#p1-suspicions-from-reading)
- Method: compared the Task's starting record with the probe: the suspicions were written before `probe_doctor_names.py` existed, and the probe was the first run of `gir doctor` against them.
- Evidence considered: this Task's `ledger.md` and DEFINE commit; `logs/probe-head-4da58e7-20261002-1221.log`.
- Conclusion: `VERIFIED`.
- Limitations: none.

## Learn

### Technical

`PENDING`

### Process

`PENDING`

## Retention and promotion

`PENDING`

## Archive readiness

`PENDING`

## Terminal record

### Summary

`PENDING`

### Gate basis

`PENDING`
