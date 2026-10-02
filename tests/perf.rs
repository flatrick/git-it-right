//! Latency budget for the commit-msg hook, the path that runs on every commit.
//! Its own test binary so other tests do not run in parallel and skew timing.

use std::process::Command;
use std::time::{Duration, Instant};

const BUDGET: Duration = Duration::from_millis(750);
const RUNS: usize = 15;

#[test]
fn commit_msg_hook_stays_within_budget() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    let global = dir.join("gitconfig");
    std::fs::write(&global, "").unwrap();
    let git = |args: &[&str]| {
        let ok = Command::new("git")
            .args(args)
            .current_dir(dir)
            .env("GIT_CONFIG_GLOBAL", &global)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .status()
            .unwrap();
        assert!(ok.success());
    };
    git(&["init", "-q"]);
    std::fs::write(dir.join(".girconfig"), "[gir]\n\tsubjectMax = 72\n").unwrap();
    let msg = dir.join("MSG");

    let mut samples = Vec::with_capacity(RUNS);
    for _ in 0..RUNS {
        std::fs::write(&msg, "Feat(api): add x.\nbody\n# comment\n").unwrap();
        let start = Instant::now();
        let out = Command::new(env!("CARGO_BIN_EXE_gir"))
            .args(["hook", "commit-msg", "MSG"])
            .current_dir(dir)
            .env("GIT_CONFIG_GLOBAL", &global)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .output()
            .unwrap();
        samples.push(start.elapsed());
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    }
    samples.sort();
    let median = samples[RUNS / 2];
    let line = format!(
        "gir perf: commit-msg hook median {} ms, max {} ms, budget {} ms ({})",
        median.as_millis(),
        samples[RUNS - 1].as_millis(),
        BUDGET.as_millis(),
        std::env::consts::OS
    );
    println!("{line}");
    assert!(median <= BUDGET, "{line}");
}
