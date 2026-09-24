use crate::config::{Config, group_title};

const PAGES: &str = include_str!("explain.md");

pub fn topics() -> Vec<&'static str> {
    let mut t: Vec<&str> = PAGES.lines().filter_map(|l| l.strip_prefix("## ")).collect();
    t.push("types");
    t
}

pub fn page(topic: &str, cfg: &Config) -> Option<String> {
    if topic == "types" {
        return Some(types(cfg));
    }
    let start = PAGES.find(&format!("## {topic}\n"))?;
    let body = &PAGES[start..];
    let body = &body[body.find('\n')? + 1..];
    let end = body.find("\n## ").map_or(body.len(), |i| i + 1);
    Some(body[..end].trim_end().to_string())
}

fn types(cfg: &Config) -> String {
    let mut out = String::from("Allowed types (gir.types in .girconfig):\n");
    for t in &cfg.types {
        out.push_str(&format!("  {t:<9} {}\n", describe(t).unwrap_or_else(|| format!("changelog group: {}", group_title(t)))));
    }
    if !cfg.aliases.is_empty() {
        let aliases: Vec<String> = cfg.aliases.iter().map(|(a, b)| format!("{a}->{b}")).collect();
        out.push_str(&format!("Auto-mapped aliases: {}\n", aliases.join(" ")));
    }
    out.push_str("Breaking change: add `!` before `:` or a `BREAKING CHANGE: <text>` footer.");
    out
}

fn describe(ty: &str) -> Option<String> {
    Some(
        match ty {
            "feat" => "new user-facing capability (minor bump)",
            "fix" => "bug fix (patch bump)",
            "docs" => "documentation only",
            "style" => "formatting, whitespace; no behavior change",
            "refactor" => "restructuring without behavior change",
            "perf" => "performance improvement",
            "test" => "adding or fixing tests",
            "build" => "build system, dependencies",
            "ci" => "CI configuration",
            "chore" => "maintenance that fits nothing else",
            "revert" => "reverts an earlier commit",
            _ => return None,
        }
        .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cc::RULES;

    #[test]
    fn every_rule_has_a_page() {
        let cfg = Config::default();
        for rule in RULES.iter().chain(&["fixup-unsquashed", "fixup", "config", "hooks", "doctor", "types"]) {
            let p = page(rule, &cfg).unwrap_or_else(|| panic!("no page for {rule}"));
            assert!(!p.is_empty() && !p.contains("\n## "), "{rule}");
        }
    }
}
