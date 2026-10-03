use std::collections::{BTreeSet, HashSet};
use std::io::Read;
use std::path::Path;

use super::fixup;
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
                crate::out!("{}", message::join(&message::RawMessage { text: outcome.text.clone(), tail: msg.tail }));
            }
            Ok(emit(&[(None, outcome)], &opts, "rejected"))
        }
        Source::Range(range) => {
            let commits = commits(&[&range])?;
            Ok(emit(&lint_recorded(&commits, cfg, false), &opts, "rejected"))
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
        crate::outln!("{}", report::json(&refs));
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
    let mut full = vec!["log", "--topo-order", "--format=%H%x1f%B%x1e"];
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

/// Lints commits already recorded, newest first as `commits` lists them, which the commit-msg
/// hook can no longer fix. An autosquash commit is judged against the older commits in the
/// list, as `git rebase --autosquash` would; `pushing` says the commits are being pushed, so a
/// target outside them is published and `git push --no-verify` is offered.
pub fn lint_recorded(commits: &[(String, String)], cfg: &Config, pushing: bool) -> Vec<(Option<String>, Outcome)> {
    let base = commits
        .last()
        .and_then(|(oldest, _)| git::run(&["rev-parse", "--verify", "--quiet", &format!("{oldest}^")]).ok())
        .map_or_else(|| "--root".to_string(), |b| b[..10].to_string());
    let tip = commits.first().map_or("", |(sha, _)| sha.as_str());
    let outcomes: Vec<Outcome> = commits.iter().map(|(_, body)| cc::check(body, cfg)).collect();
    let merges = if outcomes.iter().any(|o| matches!(o.kind, Kind::Autosquash(_))) { merges(commits) } else { HashSet::new() };
    commits
        .iter()
        .zip(outcomes)
        .enumerate()
        .map(|(i, ((sha, body), mut outcome))| {
            if let Kind::Autosquash(prefix) = &outcome.kind {
                let older = &commits[i + 1..];
                let violation = if folds(body, older, &merges, tip) {
                    Violation {
                        rule: "fixup-unsquashed",
                        message: format!("`{prefix}` commit must be squashed before pushing"),
                        hint: Some(format!("git rebase --autosquash {base}")),
                    }
                } else {
                    unmatched(prefix, sha, body, older, &base, pushing)
                };
                outcome.violations.push(violation);
            }
            reject_pending(&mut outcome);
            (Some(sha.clone()), outcome)
        })
        .collect()
}

/// A commit's subject as git reads it: the first paragraph, each line trimmed at the end and
/// joined with one space.
fn subject(body: &str) -> String {
    body.lines().take_while(|l| !l.trim().is_empty()).map(str::trim_end).collect::<Vec<_>>().join(" ")
}

/// The listed commits that are merges, which a plain `git rebase` drops from its todo.
fn merges(commits: &[(String, String)]) -> HashSet<String> {
    let ids: String = commits.iter().map(|(sha, _)| format!("{sha}\n")).collect();
    git::run_with_stdin(&["rev-list", "--merges", "--no-walk", "--stdin"], &ids)
        .map(|out| out.lines().map(str::to_string).collect())
        .unwrap_or_default()
}

/// Whether `git rebase --autosquash` finds a target among `older`, merges left out: after the
/// chain of prefixes, each followed by one or more spaces, an older subject equal to or
/// starting with the rest, or, when the rest has no space, a revision that resolves to an
/// older commit.
fn folds(body: &str, older: &[(String, String)], merges: &HashSet<String>, tip: &str) -> bool {
    let own = subject(body);
    let mut rest = own.as_str();
    while let Some(r) = ["fixup!", "squash!", "amend!"].iter().find_map(|p| rest.strip_prefix(p).filter(|r| r.starts_with(' '))) {
        rest = r.trim_start_matches(' ');
    }
    let mut targets = older.iter().filter(|(sha, _)| !merges.contains(sha));
    targets.clone().any(|(_, b)| subject(b).starts_with(rest))
        || (!rest.contains(' ')
            && !rest.starts_with('-')
            && git::run(&["rev-parse", "--verify", "--quiet", &format!("{}^{{commit}}", head_at(rest, tip))])
                .is_ok_and(|sha| targets.any(|(s, _)| *s == sha)))
}

/// `HEAD` and `@` name the tip being linted, which is what is checked out when the hinted
/// rebase runs.
fn head_at(rest: &str, tip: &str) -> String {
    for name in ["HEAD", "@"] {
        if let Some(r) = rest.strip_prefix(name)
            && (r.is_empty() || r.starts_with(['~', '^']))
        {
            return format!("{tip}{r}");
        }
    }
    rest.to_string()
}

/// The commits that last changed, before `sha`, the lines `sha` changes.
fn blamed(sha: &str) -> BTreeSet<String> {
    let parent = format!("{sha}^");
    let args = [
        "-c",
        "core.quotePath=false",
        "diff",
        "-U0",
        "--no-color",
        "--no-ext-diff",
        "--no-textconv",
        "--no-renames",
        "--src-prefix=a/",
        "--dst-prefix=b/",
        &parent,
        sha,
    ];
    let Ok(diff) = git::run_raw(&args) else { return BTreeSet::new() };
    fixup::parse_hunks(&diff).iter().flat_map(|h| fixup::blame_range(&parent, &h.path, &h.lines)).collect()
}

/// `fixup -C` takes the `amend!` commit's message without its subject, so an `amend!` with no
/// paragraph after its subject would leave its target with an empty message.
fn unmatched(prefix: &str, sha: &str, body: &str, older: &[(String, String)], base: &str, pushing: bool) -> Violation {
    let action = match prefix {
        "squash!" => "squash",
        "amend!" if body.lines().skip_while(|l| !l.trim().is_empty()).any(|l| !l.trim().is_empty()) => "fixup -C",
        _ => "fixup",
    };
    let leaves = format!("`{prefix}` commit matches no earlier commit, so git rebase --autosquash leaves it");
    let targets = blamed(sha);
    let only = if targets.len() == 1 { targets.first() } else { None };
    let (message, hint) = match only {
        Some(t) => match older.iter().find(|(s, _)| s == t) {
            Some((_, body)) => {
                (leaves, format!("git rebase -i {base}: move it below {} {}, change pick to {action}", &t[..10], subject(body)))
            }
            None => {
                let title = git::run(&["log", "-1", "--format=%s", t]).unwrap_or_default();
                let (outside, anyway) = if pushing {
                    ("is already published", ", or push it as is: git push --no-verify")
                } else {
                    ("is before the range", "")
                };
                (
                    format!("`{prefix}` commit's target {} {title} {outside}", &t[..10]),
                    format!("git rebase -i {base}: reword it into a normal commit{anyway}"),
                )
            }
        },
        None => (
            leaves,
            format!(
                "git rebase -i {base}: move it below the commit it belongs to and change pick to {action}, or reword it into a normal commit"
            ),
        ),
    };
    Violation { rule: "fixup-unmatched", message, hint: Some(hint) }
}

fn reject_pending(outcome: &mut Outcome) {
    if !outcome.fixes.is_empty() {
        let rules: Vec<&str> = outcome.fixes.iter().map(|f| f.rule).collect();
        outcome.violations.push(Violation {
            rule: "fix-pending",
            message: format!("message skipped the commit-msg hook; pending fixes: {}", rules.join(" ")),
            hint: outcome.text.lines().next().map(str::to_string),
        });
    }
}
