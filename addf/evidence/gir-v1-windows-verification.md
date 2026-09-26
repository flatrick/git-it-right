# Verification: `gir's tests and clippy pass on Windows`

## Claim

**Reference:** `LOCAL`, narrowed from
`history:open-claims/gir-v1-windows-macos.md#gir-v1-windows-macos`

<a id="gir-v1-windows"></a>
### `gir-v1-windows`

-   Claim: `cargo clippy --all-targets -- -D warnings` and `cargo test
    --no-fail-fast` exit `0` on Windows, and
    `doctor_repairs_unset_windows_longpaths` is reported as passed.
-   State: `VERIFIED`
-   Scope: `src/` and `tests/` at `42b84ad`, Windows 11, 2026-09-26.
-   Consequence if false: `gir, which promises Windows, could fail there.`
-   Basis: [Conclusion](#conclusion)

## Method

From the worktree root at `42b84ad`, run `cargo clippy --all-targets -- -D
warnings`, then `cargo test --no-fail-fast`, each with stdout and stderr
redirected to a log, and search the test log for every `test result:` line
and for `doctor_repairs_unset_windows_longpaths`.

## Expected observations

-   Both commands exit `0`.
-   Every `test result:` line reports `0 failed`, and
    `doctor_repairs_unset_windows_longpaths ... ok` appears, so the
    `#[cfg(windows)]` test ran rather than being compiled out.

## Observed results

### `windows-run`

-   Fact: clippy `exit=0`; cargo test `exit=0`; 14 `test result: ok` lines,
    each `0 failed; 0 ignored`; `test doctor_repairs_unset_windows_longpaths
    ... ok`.
-   Source and method: main checkout
    `.scratch/fix-b001-b002/clippy-20260926-1410.log` and
    `test-20260926-1410.log` (local, not committed).
-   Context: Windows 11 Home 10.0.26200, rustc and cargo 1.94.0, Git for
    Windows 2.55.0.windows.5.
-   Limitations: one machine, one run. The `#[cfg(unix)]` tests in
    `tests/doctor.rs` do not run on Windows.

## Evidence considered

-   [windows-run](#windows-run) SUPPORTS: both exits are `0` and the
    Windows-only test is listed as passed.

## Contradictory and inconclusive evidence

`NONE`.

## Conclusion

<a id="conclusion"></a>

**Result:** `VERIFIED`

The test suite and clippy pass on Windows at `42b84ad`, including
`doctor_repairs_unset_windows_longpaths`.

## Remaining uncertainty

The claim was narrowed to exclude macOS. It is not verified: on
2026-09-26 the operator decided not to verify on macOS and to rely on
POSIX adherence shared with Linux. That is an accepted assumption, not
evidence. The CI matrix in `.github/workflows/ci.yml` includes
`macos-latest`, and a passing run there would be evidence.
