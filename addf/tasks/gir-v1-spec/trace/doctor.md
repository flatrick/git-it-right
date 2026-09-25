# Trace: doctor

| Requirement | Test | Assertion |
|---|---|---|
| `req-doctor-fix-flag` | `doctor_fix_converges` | `tests/cli.rs:256-257` asserts that `--fix` created `.gitattributes` and set local `rebase.autoSquash=true`. |
| `req-doctor-warning-exit` | `doctor_fix_converges` | `tests/cli.rs:244,255` asserts exit `1` with warnings and `0` after fixing them. |
| `req-doctor-fix-converges` | `doctor_fix_converges` | `tests/cli.rs:254-255` asserts that the second run has no `warn ` lines and exits `0`. |
| `req-doctor-autosquash` | `doctor_fix_converges` | `tests/cli.rs:246-247,257` asserts the warning names `rebase.autoSquash` and the local setting becomes `true`. |
| `req-doctor-gitattributes-missing` | `doctor_fix_converges` | `tests/cli.rs:246-247,256` asserts the missing-file warning and that `--fix` creates `.gitattributes`. |
| `req-doctor-exec-bit` | `doctor_fix_converges` | `tests/cli.rs:246-247,254` asserts the initial `exec-bit` warning disappears after `--fix`. |
