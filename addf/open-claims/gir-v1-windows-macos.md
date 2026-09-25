# Open Claim: `gir-v1-windows-macos`

## Claim

<a id="gir-v1-windows-macos"></a>
-   Claim: `cargo test` and `cargo clippy --all-targets -- -D warnings`
    pass on Windows and macOS, including the Windows-only
    `doctor_repairs_unset_windows_longpaths`.
-   State: `UNVERIFIED`
-   Scope: `src/` and `tests/` at `f83b6bb`, Windows and macOS.
-   Consequence if false: `gir, which promises Windows, Linux and macOS, could fail on a platform the Linux evidence does not cover.`
-   Basis: `DEFERRED_VERIFICATION`; the operator will run the suite on
    Windows personally, and no macOS machine was available.

## Originating Task

-   `history:tasks/gir-v1-spec/TASK.md#gir-v1-windows-macos`

## Next action

On each OS, from the repository root, run `cargo clippy --all-targets -- -D
warnings` and `cargo test --no-fail-fast`, keeping the full output. On
Windows, also confirm that `doctor_repairs_unset_windows_longpaths` is listed
as passed. Both commands exiting `0` on an OS settles the Claim for that OS.
Record the result in a Verification, and split the Claim if only one OS is
settled.
