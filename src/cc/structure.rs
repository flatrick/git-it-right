#[derive(Debug, Default, PartialEq, Eq)]
pub struct Structure {
    /// `fixup!`, `squash!` or `amend!` when the subject carries one.
    pub autosquash_prefix: Option<String>,
    /// The line after the subject is not blank, so git would glue it onto the subject.
    pub missing_blank_line: bool,
}

/// Inspects an already comment-stripped message.
pub fn parse(text: &str) -> Structure {
    let autosquash_prefix = ["fixup!", "squash!", "amend!"]
        .into_iter()
        .find(|p| text.strip_prefix(p).is_some_and(|rest| rest.starts_with(' ')))
        .map(str::to_string);
    Structure { autosquash_prefix, missing_blank_line: text.lines().nth(1).is_some_and(|l| !l.is_empty()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_autosquash_prefixes() {
        for p in ["fixup!", "squash!", "amend!"] {
            assert_eq!(parse(&format!("{p} feat: a\n")).autosquash_prefix.as_deref(), Some(p));
        }
        assert_eq!(parse("feat: a\n").autosquash_prefix, None);
        assert_eq!(parse("fixup!feat: a\n").autosquash_prefix, None);
    }

    #[test]
    fn detects_missing_blank_line() {
        assert!(parse("feat: a\nmore\n").missing_blank_line);
        assert!(!parse("feat: a\n\nmore\n").missing_blank_line);
        assert!(!parse("feat: a\n").missing_blank_line);
    }
}
