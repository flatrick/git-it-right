use crate::cc::{Fix, Outcome, Violation};

pub fn fix_lines(fixes: &[Fix]) -> Vec<String> {
    fixes.iter().map(|f| format!("gir: fixed [{}] {} -> {}", f.rule, one_line(&f.from), one_line(&f.to))).collect()
}

/// At most three lines per violation: what, one example, where to read more.
pub fn violation_lines(label: &str, violations: &[Violation]) -> Vec<String> {
    let mut out = Vec::new();
    for v in violations {
        out.push(format!("gir: {label} [{}] {}", v.rule, v.message));
        if let Some(hint) = &v.hint {
            out.push(format!("  try: {hint}"));
        }
        out.push(format!("  more: gir explain {}", v.rule));
    }
    out
}

pub fn json(outcomes: &[(Option<String>, &Outcome)]) -> String {
    let items: Vec<String> = outcomes
        .iter()
        .map(|(sha, o)| {
            let fixes: Vec<String> =
                o.fixes.iter().map(|f| format!(r#"{{"rule":{},"from":{},"to":{}}}"#, s(f.rule), s(&f.from), s(&f.to))).collect();
            let violations: Vec<String> = o
                .violations
                .iter()
                .map(|v| {
                    let hint = v.hint.as_deref().map_or("null".to_string(), s);
                    format!(
                        r#"{{"rule":{},"message":{},"hint":{},"explain":{}}}"#,
                        s(v.rule),
                        s(&v.message),
                        hint,
                        s(&format!("gir explain {}", v.rule))
                    )
                })
                .collect();
            let sha = sha.as_deref().map_or("null".to_string(), s);
            format!(
                r#"{{"commit":{sha},"ok":{},"message":{},"fixes":[{}],"violations":[{}]}}"#,
                o.ok(),
                s(&o.text),
                fixes.join(","),
                violations.join(",")
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

fn one_line(text: &str) -> String {
    text.replace('\n', "\\n")
}

fn s(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_escapes() {
        assert_eq!(s("a\"b\\c\nd\u{1}"), r#""a\"b\\c\nd\u0001""#);
    }
}
