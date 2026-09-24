//! Enforces the entry format documented at the top of FRICTION.md.

const OS_KEYS: [&str; 3] = ["windows", "linux", "macos"];
const OS_STATES: [&str; 3] = ["seen", "not-seen", "untested"];
const AREAS: [&str; 9] = ["agent", "harness", "hooks", "install", "lint", "fixup", "doctor", "ci", "tooling"];
const STATUSES: [&str; 4] = ["open", "workaround", "fixed", "wontfix"];

fn validate(text: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let Some((_, entries)) = text.split_once("\n## Entries\n") else {
        return vec!["missing `## Entries` section".into()];
    };
    let mut last_id = 0;
    for block in entries.split("\n### ").skip(1) {
        let (heading, body) = block.split_once('\n').unwrap_or((block, ""));
        let Some((id, title)) = heading.split_once(' ') else {
            errors.push(format!("`{heading}`: heading needs `F-NNN <title>`"));
            continue;
        };
        let at = |msg: &str| format!("{id}: {msg}");
        match id.strip_prefix("F-").filter(|n| n.len() == 3).and_then(|n| n.parse::<u32>().ok()) {
            Some(n) if n > last_id => last_id = n,
            Some(_) => errors.push(at("IDs must be unique and increasing")),
            None => errors.push(at("ID must look like F-001")),
        }
        if title.trim().is_empty() {
            errors.push(at("missing title"));
        }

        let mut fields = Vec::new();
        let mut prose = String::new();
        for line in body.lines().skip_while(|l| l.is_empty()) {
            match line.strip_prefix("- ").and_then(|l| l.split_once(": ")) {
                Some((k, v)) if prose.is_empty() => fields.push((k, v)),
                _ => prose.push_str(line),
            }
        }
        if prose.trim().is_empty() {
            errors.push(at("needs a description after the fields"));
        }
        let get = |key: &str| fields.iter().find(|(k, _)| *k == key).map(|(_, v)| *v);
        for (k, _) in &fields {
            if !["OS", "Shell", "Area", "Status", "Found"].contains(k) {
                errors.push(at(&format!("unknown field `{k}`")));
            }
        }

        match get("OS") {
            None => errors.push(at("missing `OS:`")),
            Some(os) => {
                let pairs: Vec<Option<(&str, &str)>> = os.split(' ').map(|p| p.split_once('=')).collect();
                let ok = pairs.len() == 3
                    && pairs.iter().zip(OS_KEYS).all(|(p, key)| p.is_some_and(|(k, s)| k == key && OS_STATES.contains(&s)));
                if !ok {
                    errors.push(at(&format!("`OS: {os}` must be `windows=<state> linux=<state> macos=<state>`, state one of {OS_STATES:?}")));
                }
            }
        }
        match get("Area") {
            Some(a) if AREAS.contains(&a) => {}
            other => errors.push(at(&format!("`Area: {}` must be one of {AREAS:?}", other.unwrap_or("")))),
        }
        match get("Status") {
            Some(s) if STATUSES.contains(&s) => {}
            other => errors.push(at(&format!("`Status: {}` must be one of {STATUSES:?}", other.unwrap_or("")))),
        }
        let date_ok = get("Found").is_some_and(|d| {
            let b = d.as_bytes();
            b.len() == 10 && b[4] == b'-' && b[7] == b'-' && d.chars().filter(char::is_ascii_digit).count() == 8
        });
        if !date_ok {
            errors.push(at("`Found:` must be YYYY-MM-DD"));
        }
    }
    errors
}

#[test]
fn friction_log_follows_the_format() {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/FRICTION.md")).unwrap();
    let errors = validate(&text.replace("\r\n", "\n"));
    assert!(errors.is_empty(), "FRICTION.md:\n  {}", errors.join("\n  "));
}

#[test]
fn validator_catches_each_mistake() {
    let bad = "# x\n\n## Entries\n\n\
### F-002 fine\n\n- OS: windows=seen linux=untested macos=untested\n- Area: lint\n- Status: open\n- Found: 2026-09-24\n\ntext\n\n\
### F-002 duplicate id\n\n- OS: windows=seen linux=untested macos=untested\n- Area: lint\n- Status: open\n- Found: 2026-09-24\n\ntext\n\n\
### F-003 bad os order\n\n- OS: linux=seen windows=untested macos=untested\n- Area: lint\n- Status: open\n- Found: 2026-09-24\n\ntext\n\n\
### F-004 bad state, area, status, date, field, no text\n\n- OS: windows=maybe linux=untested macos=untested\n- Area: misc\n- Status: done\n- Found: 24/09/2026\n- Owner: me\n";
    let errors = validate(bad);
    for want in ["F-002: IDs must be unique", "F-003: `OS:", "F-004: `OS:", "F-004: `Area:", "F-004: `Status:", "F-004: `Found:", "F-004: unknown field `Owner`", "F-004: needs a description"] {
        assert!(errors.iter().any(|e| e.starts_with(want)), "expected `{want}` in {errors:#?}");
    }
    assert_eq!(errors.len(), 8, "{errors:#?}");
}
