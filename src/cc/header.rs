//! Lenient lexer for the header line. Unlike `git_conventional`, it accepts
//! malformed spacing and casing so the fixer knows what the author meant.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub ty: String,
    pub scope: Option<String>,
    pub bang: bool,
    pub desc: String,
}

impl Header {
    pub fn render(&self) -> String {
        let mut out = self.ty.clone();
        if let Some(scope) = &self.scope {
            out.push('(');
            out.push_str(scope);
            out.push(')');
        }
        if self.bang {
            out.push('!');
        }
        out.push_str(": ");
        out.push_str(&self.desc);
        out
    }
}

pub fn lex(line: &str) -> Option<Header> {
    let s = line.trim();
    let ty_end = s.find(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_')).unwrap_or(s.len());
    if ty_end == 0 {
        return None;
    }
    let ty = &s[..ty_end];
    let mut rest = s[ty_end..].trim_start();
    let mut scope = None;
    if let Some(r) = rest.strip_prefix('(') {
        let close = r.find(')')?;
        scope = Some(r[..close].split(',').map(str::trim).collect::<Vec<_>>().join(","));
        rest = r[close + 1..].trim_start();
    }
    let bang = rest.starts_with('!');
    if bang {
        rest = rest[1..].trim_start();
    }
    let desc = rest.strip_prefix(':')?.trim();
    Some(Header { ty: ty.to_string(), scope, bang, desc: desc.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(ty: &str, scope: Option<&str>, bang: bool, desc: &str) -> Option<Header> {
        Some(Header { ty: ty.into(), scope: scope.map(Into::into), bang, desc: desc.into() })
    }

    #[test]
    fn lexes_well_formed_and_sloppy_headers() {
        assert_eq!(lex("feat(api)!: add x"), h("feat", Some("api"), true, "add x"));
        assert_eq!(lex("feat :x"), h("feat", None, false, "x"));
        assert_eq!(lex("Feat ( a , b ) ! :  x"), h("Feat", Some("a,b"), true, "x"));
        assert_eq!(lex("feat(): x"), h("feat", Some(""), false, "x"));
        assert_eq!(lex("feat:"), h("feat", None, false, ""));
    }

    #[test]
    fn rejects_non_headers() {
        assert_eq!(lex("WIP stuff"), None);
        assert_eq!(lex("Update README.md"), None);
        assert_eq!(lex(": nothing"), None);
        assert_eq!(lex("feat(api: x"), None);
    }

    #[test]
    fn render_is_canonical() {
        assert_eq!(lex("feat ( api ) !:x").unwrap().render(), "feat(api)!: x");
    }
}
