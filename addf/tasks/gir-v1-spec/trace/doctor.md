# Trace: doctor

| Requirement | Test | Assertion |
|---|---|---|
| `req-doctor-fix-flag` | `doctor_fix_converges` | `tests/cli.rs:184-185` asserts that `--fix` created `.gitattributes` and set local `rebase.autoSquash=true`. |
| `req-doctor-warning-exit` | `doctor_fix_converges` | `tests/cli.rs:172,183` asserts exit `1` with warnings and `0` after fixing them. |
| `req-doctor-fix-converges` | `doctor_fix_converges` | `tests/cli.rs:182-183` asserts that the second run has no `warn ` lines and exits `0`. |
| `req-doctor-autosquash` | `doctor_fix_converges` | `tests/cli.rs:174-175,185` asserts the warning names `rebase.autoSquash` and the local setting becomes `true`. |
| `req-doctor-gitattributes-missing` | `doctor_fix_converges` | `tests/cli.rs:174-175,184` asserts the missing-file warning and that `--fix` creates `.gitattributes`. |
| `req-doctor-exec-bit` | `doctor_fix_converges` | `tests/cli.rs:174-175,182` asserts the initial `exec-bit` warning disappears after `--fix`. |
