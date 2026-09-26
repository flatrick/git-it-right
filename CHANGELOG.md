# Changelog
## v0.0.3 - 2026-09-26

### Documentation

- Add RELEASE-GUIDE.md for cutting a release
- **addf:** Open init-adopted-repo task in DEFINE
- Add GIT-IT-RIGHT.md from gir init --optional
- **addf:** Complete init-adopted-repo and publish its spec delta

### Features

- **init:** Generate GIT-IT-RIGHT.md and respect removed optional files
## v0.0.2 - 2026-09-26

### Bug Fixes

- **doctor:** Report an unreadable .gitattributes instead of calling it missing
- **cli:** Exit 141 quietly when stdout is closed early
- **addf:** Check retired open claims with --include-archive

### CI

- Check the addf archive

### Documentation

- Close task fix-b001-b002 and publish its spec delta
- File B-004 to B-010 from the Rust review
- Reproduce B-008 and B-009
- Record the B-007 decision to ask the remote with ls-remote
- Narrow the open B-007 question to the second connection
- Verify B-001, B-002 and the test suite on Windows
- **addf:** Fix the archived fix-b001-b002 record for --include-archive
- **addf:** Note the Windows B-002 tests in its Verification
- **addf:** Log unverified statements made around verified work
- **addf:** Correct unverified statements in the Windows records
- **addf:** Commit the logs behind the Windows verifications
- **addf:** Re-verify the Linux claims with committed output
- **addf:** Drop the WSL2 caveat from the Linux re-verification

### Miscellaneous

- Open task fix-b001-b002

### Testing

- Add failing regressions for B-001 and B-002
- Report when the unreadable-file regressions are skipped as root
- Cover B-002 with a .gitattributes directory
- Cover B-002 on Windows with icacls read denies
## v0.0.1 - 2026-09-25

### Bug Fixes

- Take upstream check-capsule UTF-8 output fix
- **cli:** Accept --range only for gir lint
- **lint:** Reject recorded commits that still need safe fixes
- **fixup:** Refuse explicit targets outside the branch or on the base
- **init:** Stage hooks only when written or not executable in the index
- Run init, doctor and fixup from the repository root
- **config:** Fail on a .girconfig git cannot parse
- **lint:** Report a period-only description as desc-empty
- **fixup:** Say which staged file has no lines to trace
- **doctor:** Stop reporting unmerged stages as case collisions
- **lint:** Refuse --fix together with --range
- **doctor:** Keep non-UTF-8 .gitignore and .gitattributes content
- **init:** Keep a differing non-UTF-8 file without --force
- **fixup:** Refuse hunks that replace base-branch lines

### Build

- Set the crate version to 0.0.1

### CI

- Check the addf capsule

### Documentation

- Add FRICTION.md with a format checked by cargo test
- Add README explaining how to install and use gir
- Add a commit type cheatsheet that gir explain prints
- Log check-capsule Windows output-encoding friction
- **addf:** Start the gir-v1-spec Task
- **addf:** Draft the gir v1 spec delta and record conflict rulings
- **addf:** Follow the test helper move in trace citations
- Show the bugfix alias on the gir explain config page
- **addf:** Let trace.py accept OS-gated tests
- **addf:** Log the gir-v1-spec implementation so far
- **addf:** Trace cli, config, lint and hooks requirements to tests
- **addf:** Trace fixup, init, doctor and explain requirements
- **addf:** Fold the conformance review into the spec delta
- **addf:** Fold the second conformance review into the spec delta
- **addf:** Fold the third conformance review into the spec delta
- **addf:** Complete gir-v1-spec and publish the gir v1 spec
- Add BUGS.md for known unfixed defects
- Record Windows reproductions of B-001 and B-002

### Features

- Add gir, a Conventional Commits linter with fixup and doctor

### Miscellaneous

- Initialize repository with ignore and attribute rules
- Dogfood gir hooks and config
- Apply gir doctor fixes
- Add __pycache__/ to .gitignore
- Adopt addf agent-driven development framework
- Exclude addf archive from ripgrep
- **addf:** Archive the completed gir-v1-spec Task

### Testing

- Guard commit-msg hook latency with a 750 ms budget
- Verify every row of the cheatsheet fixup table against git
- Move the shared Repo helper into tests/common
- Cover the cli, config, lint and hooks spec requirements
- Cover the doctor and explain spec requirements
- Pin the full .editorconfig and fixup's git output order
- Pin --version and --help ignoring trailing arguments
- Pin footer scope, pre-push stdin and flag edge cases
