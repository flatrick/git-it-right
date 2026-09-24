//! Splits the raw file git hands to `commit-msg` into the message proper and the
//! git-generated tail (comment lines, scissors line, `commit -v` diff), using the
//! same rules as `git stripspace --strip-comments` / `--cleanup=scissors`,
//! honouring `core.commentChar`.

const SCISSORS: &str = " ------------------------ >8 ------------------------";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawMessage {
    pub text: String,
    pub tail: Vec<String>,
}

pub fn split(raw: &str, comment: &str) -> RawMessage {
    let raw = raw.replace("\r\n", "\n");
    let mut text = Vec::new();
    let mut tail = Vec::new();
    let mut lines = raw.lines();
    while let Some(line) = lines.next() {
        if line.starts_with(comment) && line[comment.len()..].starts_with(SCISSORS) {
            tail.push(line.to_string());
            tail.extend(lines.by_ref().map(str::to_string));
            break;
        }
        if line.starts_with(comment) {
            tail.push(line.to_string());
        } else {
            text.push(line);
        }
    }
    RawMessage { text: text.join("\n"), tail }
}

pub fn join(msg: &RawMessage) -> String {
    let mut out = msg.text.trim_end().to_string();
    out.push('\n');
    if !msg.tail.is_empty() {
        out.push('\n');
        out.push_str(&msg.tail.join("\n"));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_comments_and_everything_after_scissors() {
        let raw = "feat: x\n\nbody\n# comment\n# ------------------------ >8 ------------------------\ndiff --git a b\n+x\n";
        let m = split(raw, "#");
        assert_eq!(m.text, "feat: x\n\nbody");
        assert_eq!(m.tail.len(), 4);
        assert_eq!(join(&m), "feat: x\n\nbody\n\n# comment\n# ------------------------ >8 ------------------------\ndiff --git a b\n+x\n");
    }

    #[test]
    fn honours_custom_comment_char_and_crlf() {
        let m = split("feat: x\r\n; note\r\n# kept\r\n", ";");
        assert_eq!(m.text, "feat: x\n# kept");
        assert_eq!(m.tail, vec!["; note"]);
    }
}
