use std::io::Read;
use std::path::Path;

use crate::cc::{self, Kind, Outcome, Violation};
use crate::config::Config;
use crate::{git, message, report};

pub enum Source {
    File(String),
    Stdin,
    Range(String),
}

pub struct Options {
    pub fix: bool,
    pub json: bool,
}

pub fn lint(source: Source, opts: Options, cfg: &Config) -> Result<i32, String> {
    match source {
        Source::File(path) => lint_file(Path::new(&path), &opts, cfg, "rejected"),
        Source::Stdin => {
            let mut raw = String::new();
            std::io::stdin().read_to_string(&mut raw).map_err(|e| e.to_string())?;
            let msg = message::split(&raw, &git::comment_string());
            let outcome = cc::check(&msg.text, cfg);
            if opts.fix && !opts.json {
                print!("{}", message::join(&message::RawMessage { text: outcome.text.clone(), tail: msg.tail }));
            }
            Ok(emit(&[(None, outcome)], &opts, "rejected"))
        }
        Source::Range(range) => {
            let commits = commits(&[&range])?;
            let outcomes: Vec<_> = commits
                .into_iter()
                .map(|(sha, body)| {
                    let mut outcome = cc::check(&body, cfg);
                    reject_autosquash(&mut outcome);
                    (Some(sha), outcome)
                })
                .collect();
            Ok(emit(&outcomes, &opts, "rejected"))
        }
    }
}

/// `commit-msg` hook entry: fixes the message file in place, rejects what it cannot fix.
pub fn lint_file(path: &Path, opts: &Options, cfg: &Config, label: &str) -> Result<i32, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let msg = message::split(&raw, &git::comment_string());
    let outcome = cc::check(&msg.text, cfg);
    if opts.fix && !outcome.fixes.is_empty() {
        let fixed = message::join(&message::RawMessage { text: outcome.text.clone(), tail: msg.tail });
        std::fs::write(path, fixed).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    }
    Ok(emit(&[(None, outcome)], opts, label))
}

pub fn emit(outcomes: &[(Option<String>, Outcome)], opts: &Options, label: &str) -> i32 {
    let failed = outcomes.iter().any(|(_, o)| !o.ok());
    if opts.json {
        let refs: Vec<_> = outcomes.iter().map(|(s, o)| (s.clone(), o)).collect();
        println!("{}", report::json(&refs));
    } else {
        for (sha, o) in outcomes {
            let label = match sha {
                Some(sha) => format!("{} {label}", &sha[..sha.len().min(10)]),
                None => label.to_string(),
            };
            if opts.fix {
                report::fix_lines(&o.fixes).iter().for_each(|l| eprintln!("{l}"));
            }
            report::violation_lines(&label, &o.violations).iter().for_each(|l| eprintln!("{l}"));
        }
    }
    i32::from(failed)
}

/// `(sha, message)` for each commit selected by `git log <args>`.
pub fn commits(args: &[&str]) -> Result<Vec<(String, String)>, String> {
    let mut full = vec!["log", "--format=%H%x1f%B%x1e"];
    full.extend_from_slice(args);
    let out = git::run(&full)?;
    Ok(out
        .split('\x1e')
        .filter_map(|rec| {
            let (sha, body) = rec.trim_start_matches('\n').split_once('\x1f')?;
            Some((sha.to_string(), body.to_string()))
        })
        .collect())
}

pub fn reject_autosquash(outcome: &mut Outcome) {
    if let Kind::Autosquash(prefix) = &outcome.kind {
        outcome.violations.push(Violation {
            rule: "fixup-unsquashed",
            message: format!("`{prefix}` commit must be squashed before pushing"),
            hint: Some("git rebase --autosquash <base>".into()),
        });
    }
}
