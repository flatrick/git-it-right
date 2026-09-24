pub mod header;
pub mod structure;

use crate::config::{Config, DescCase};
use header::Header;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub rule: &'static str,
    pub message: String,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fix {
    pub rule: &'static str,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Conventional,
    Autosquash(String),
    Merge,
    Revert,
}

#[derive(Debug, Clone)]
pub struct Outcome {
    pub kind: Kind,
    pub text: String,
    pub fixes: Vec<Fix>,
    pub violations: Vec<Violation>,
}

impl Outcome {
    pub fn ok(&self) -> bool {
        self.violations.is_empty()
    }
}

/// Every rule id `check` can emit; `gir explain` has a page for each.
pub const RULES: &[&str] = &[
    "empty",
    "type-missing",
    "type-case",
    "type-alias",
    "type-unknown",
    "scope-empty",
    "scope-unknown",
    "scope-required",
    "header-spacing",
    "header-length",
    "desc-empty",
    "desc-period",
    "desc-case",
    "body-separator",
    "breaking-footer",
    "merge-commit",
    "revert-commit",
    "spec",
];

/// Lints a comment-stripped message and applies every safe fix.
pub fn check(raw: &str, cfg: &Config) -> Outcome {
    let text = normalize(raw);
    let mut out = Outcome { kind: Kind::Conventional, text: text.clone(), fixes: Vec::new(), violations: Vec::new() };
    let Some(subject) = text.lines().next() else {
        out.violations.push(violation("empty", "commit message is empty".into(), None));
        return out;
    };

    let structure = structure::parse(&text);
    if let Some(prefix) = structure.autosquash_prefix {
        out.kind = Kind::Autosquash(prefix);
        return out;
    }
    if subject.starts_with("Merge ") {
        out.kind = Kind::Merge;
        if !cfg.allow_merge {
            out.violations.push(violation("merge-commit", "merge commits are disabled (gir.allowMerge=false)".into(), Some("git rebase <base> instead of merging".into())));
        }
        return out;
    }
    if subject.starts_with("Revert \"") {
        out.kind = Kind::Revert;
        if !cfg.allow_revert {
            out.violations.push(violation("revert-commit", "git's default revert subject is disabled (gir.allowRevert=false)".into(), Some("revert: <original description>".into())));
        }
        return out;
    }

    let Some(mut h) = header::lex(subject) else {
        out.violations.push(violation(
            "type-missing",
            "header must start with `<type>[(scope)][!]: `".into(),
            Some(format!("<type>: {}   types: {}", subject.trim(), cfg.types.join(" "))),
        ));
        return out;
    };

    let mut fixes = Vec::new();
    let mut violations = Vec::new();
    let respaced = h.render();
    if respaced != subject {
        fixes.push(Fix { rule: "header-spacing", from: subject.to_string(), to: respaced });
    }
    fix_header(&mut h, cfg, &mut fixes, &mut violations);

    let rendered = h.render();
    let header_len = rendered.chars().count();
    if header_len > cfg.subject_max {
        violations.push(violation(
            "header-length",
            format!("header is {header_len} chars (max {})", cfg.subject_max),
            Some("keep the header short; move detail into the body after a blank line".into()),
        ));
    }

    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    lines[0] = rendered;
    if structure.missing_blank_line {
        fixes.push(Fix { rule: "body-separator", from: lines[1].clone(), to: format!("\n{}", lines[1]) });
        lines.insert(1, String::new());
    }
    fix_breaking_footers(&mut lines, &mut fixes);

    out.text = lines.join("\n");
    if violations.is_empty()
        && let Err(e) = git_conventional::Commit::parse(&out.text)
    {
        violations.push(violation("spec", format!("not a valid Conventional Commit: {}", e.kind()), None));
    }
    out.fixes = fixes;
    out.violations = violations;
    out
}

fn fix_header(h: &mut Header, cfg: &Config, fixes: &mut Vec<Fix>, violations: &mut Vec<Violation>) {
    let lower = h.ty.to_lowercase();
    if lower != h.ty && (cfg.types.contains(&lower) || cfg.alias_for(&lower).is_some()) {
        fixes.push(Fix { rule: "type-case", from: h.ty.clone(), to: lower.clone() });
        h.ty = lower;
    }
    if let Some(target) = cfg.alias_for(&h.ty) {
        fixes.push(Fix { rule: "type-alias", from: h.ty.clone(), to: target.to_string() });
        h.ty = target.to_string();
    }
    if !cfg.types.contains(&h.ty) {
        let suggestion = nearest(&h.ty.to_lowercase(), &cfg.types);
        let mut example = h.clone();
        example.ty = suggestion.clone().unwrap_or_else(|| "<type>".into());
        let did_you_mean = suggestion.map(|s| format!(" (did you mean `{s}`?)")).unwrap_or_default();
        violations.push(violation(
            "type-unknown",
            format!("`{}` is not an allowed type{did_you_mean}", h.ty),
            Some(format!("{}   types: {}", example.render(), cfg.types.join(" "))),
        ));
    }

    if h.scope.as_deref() == Some("") {
        fixes.push(Fix { rule: "scope-empty", from: "()".into(), to: String::new() });
        h.scope = None;
    }
    match &h.scope {
        None if cfg.scope_required => violations.push(violation(
            "scope-required",
            "a scope is required".into(),
            Some(format!("{}(<scope>): {}{}", h.ty, h.desc, scope_list(cfg))),
        )),
        Some(scope) if !cfg.scopes.is_empty() => {
            for s in scope.split(',').filter(|s| !cfg.scopes.iter().any(|a| a == s)) {
                violations.push(violation("scope-unknown", format!("`{s}` is not an allowed scope"), Some(format!("{}(<scope>): {}{}", h.ty, h.desc, scope_list(cfg)))));
            }
        }
        _ => {}
    }

    if h.desc.is_empty() {
        violations.push(violation("desc-empty", "description after `: ` is empty".into(), Some(format!("{}: <what changed, imperative mood>", h.ty))));
        return;
    }
    if h.desc.ends_with('.') && !h.desc.ends_with("..") {
        let trimmed = h.desc.trim_end_matches('.').trim_end().to_string();
        fixes.push(Fix { rule: "desc-period", from: h.desc.clone(), to: trimmed.clone() });
        h.desc = trimmed;
    }
    if cfg.desc_case == DescCase::Lower {
        let mut chars = h.desc.chars();
        if let (Some(first), second) = (chars.next(), chars.next())
            && first.is_uppercase()
            && !second.is_some_and(char::is_uppercase)
        {
            let lowered: String = first.to_lowercase().chain(h.desc.chars().skip(1)).collect();
            fixes.push(Fix { rule: "desc-case", from: h.desc.clone(), to: lowered.clone() });
            h.desc = lowered;
        }
    }
}

fn fix_breaking_footers(lines: &mut [String], fixes: &mut Vec<Fix>) {
    let footer_start = lines.iter().rposition(|l| l.is_empty()).map_or(lines.len(), |i| i + 1);
    for line in &mut lines[footer_start..] {
        let Some((token, value)) = line.split_once(':') else { continue };
        let normalized = token.trim().to_uppercase().replace('_', " ");
        if normalized != "BREAKING CHANGE" && normalized != "BREAKING-CHANGE" {
            continue;
        }
        let value = value.trim();
        let fixed = format!("BREAKING CHANGE: {value}");
        if *line != fixed && *line != format!("BREAKING-CHANGE: {value}") {
            fixes.push(Fix { rule: "breaking-footer", from: line.clone(), to: fixed.clone() });
            *line = fixed;
        }
    }
}

/// Matches what `git commit` does anyway (git stripspace): strip trailing
/// whitespace, drop leading/trailing blank lines, collapse runs of blank lines.
fn normalize(raw: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for line in raw.lines().map(str::trim_end) {
        if line.is_empty() && out.last().is_none_or(|l| l.is_empty()) {
            continue;
        }
        out.push(line);
    }
    while out.last().is_some_and(|l| l.is_empty()) {
        out.pop();
    }
    out.join("\n")
}

fn nearest(word: &str, candidates: &[String]) -> Option<String> {
    candidates
        .iter()
        .map(|c| (distance(word, c), c))
        .filter(|(d, c)| *d <= 2 && *d < c.len())
        .min_by_key(|(d, _)| *d)
        .map(|(_, c)| c.clone())
}

fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut cur = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            cur.push((prev[j] + usize::from(ca != *cb)).min(prev[j + 1] + 1).min(cur[j] + 1));
        }
        prev = cur;
    }
    prev[b.len()]
}

fn scope_list(cfg: &Config) -> String {
    if cfg.scopes.is_empty() { String::new() } else { format!("   scopes: {}", cfg.scopes.join(" ")) }
}

fn violation(rule: &'static str, message: String, hint: Option<String>) -> Violation {
    Violation { rule, message, hint }
}

#[cfg(test)]
mod tests;
