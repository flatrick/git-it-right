use crate::config::{Config, group_title};

const RULE_PAGES: &str = include_str!("explain.md");
const CHEATSHEET: &str = include_str!("../CHEATSHEET.md");
const SOURCES: [&str; 2] = [CHEATSHEET, RULE_PAGES];

pub fn topics() -> Vec<&'static str> {
    let mut t: Vec<&str> = SOURCES.iter().flat_map(|s| s.lines().filter_map(|l| l.strip_prefix("## "))).collect();
    t.push("types");
    t
}

pub fn page(topic: &str, cfg: &Config) -> Option<String> {
    if topic == "types" {
        return Some(types(cfg));
    }
    SOURCES.iter().find_map(|source| section(source, topic))
}

fn section(source: &str, topic: &str) -> Option<String> {
    let start = source.find(&format!("## {topic}\n"))?;
    let body = &source[start..];
    let body = &body[body.find('\n')? + 1..];
    let end = body.find("\n## ").map_or(body.len(), |i| i + 1);
    Some(body[..end].trim_end().to_string())
}

fn types(cfg: &Config) -> String {
    let mut out = String::from("Allowed types (gir.types in .girconfig):\n");
    for t in &cfg.types {
        let summary = section(CHEATSHEET, t)
            .and_then(|s| s.lines().next().map(str::to_string))
            .unwrap_or_else(|| format!("changelog group: {}", group_title(t)));
        out.push_str(&format!("  {t:<9} {summary}\n"));
    }
    if !cfg.aliases.is_empty() {
        let aliases: Vec<String> = cfg.aliases.iter().map(|(a, b)| format!("{a}->{b}")).collect();
        out.push_str(&format!("Auto-mapped aliases: {}\n", aliases.join(" ")));
    }
    out.push_str("When to use each, with examples: gir explain <type>. Also: gir explain breaking, scopes, fixup.");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cc::{self, RULES};
    use crate::config::DEFAULT_TYPES;

    #[test]
    fn every_rule_and_type_has_a_page() {
        let cfg = Config::default();
        let types = DEFAULT_TYPES.iter().map(|(t, _)| t);
        let extra = ["fixup-unsquashed", "fix-pending", "fixup", "breaking", "scopes", "config", "hooks", "doctor", "types"];
        for topic in RULES.iter().chain(types).chain(&extra) {
            let p = page(topic, &cfg).unwrap_or_else(|| panic!("no page for {topic}"));
            assert!(!p.is_empty() && !p.contains("\n## "), "{topic}");
        }
    }

    #[test]
    fn topics_are_unique() {
        let mut t = topics();
        let n = t.len();
        t.sort();
        t.dedup();
        assert_eq!(t.len(), n, "a topic heading appears twice");
    }

    #[test]
    fn cheatsheet_examples_pass_the_linter_untouched() {
        let cfg = Config::default();
        for (ty, _) in DEFAULT_TYPES {
            let page = section(CHEATSHEET, ty).unwrap();
            let examples: Vec<_> = page.lines().filter_map(|l| l.strip_prefix("- `")?.strip_suffix('`')).collect();
            assert!(examples.len() >= 2, "{ty} has fewer than two examples");
            for example in examples {
                let o = cc::check(example, &cfg);
                assert!(o.ok() && o.fixes.is_empty(), "{example}: {:?} {:?}", o.violations, o.fixes);
                assert!(example.starts_with(ty), "{example} is listed under {ty}");
            }
        }
    }
}
