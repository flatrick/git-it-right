use std::io::{BufRead, IsTerminal, Write};

/// Whether gir may ask the user. `GIR_INTERACTIVE=1` or `0` overrides the check that
/// stdin and stderr are terminals; GUI clients, CI and agents have no terminal.
pub fn interactive() -> bool {
    match std::env::var("GIR_INTERACTIVE").as_deref() {
        Ok("1") => true,
        Ok("0") => false,
        _ => std::io::stdin().is_terminal() && std::io::stderr().is_terminal(),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Choice {
    Item(usize),
    Split,
    Cancel,
}

/// Lists `items` numbered from 1 and asks until the answer is a listed number, `s` when
/// `split` describes that choice, or a cancel: an empty answer, `q`, or end of input.
pub fn choose(items: &[String], split: Option<&str>, mut input: impl BufRead, mut out: impl Write) -> Choice {
    for (i, item) in items.iter().enumerate() {
        let _ = writeln!(out, "  {}) {item}", i + 1);
    }
    if let Some(split) = split {
        let _ = writeln!(out, "  s) split: {split}");
    }
    let prompt = format!("pick [1-{}{}, q]: ", items.len(), if split.is_some() { ", s" } else { "" });
    loop {
        let _ = write!(out, "{prompt}");
        let _ = out.flush();
        let mut line = String::new();
        if input.read_line(&mut line).unwrap_or(0) == 0 {
            let _ = writeln!(out);
            return Choice::Cancel;
        }
        match line.trim() {
            "" | "q" => return Choice::Cancel,
            "s" if split.is_some() => return Choice::Split,
            n => {
                if let Ok(i) = n.parse::<usize>()
                    && (1..=items.len()).contains(&i)
                {
                    return Choice::Item(i - 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ask(input: &str, split: bool) -> (Choice, String) {
        let items = ["abc one".to_string(), "def two".to_string()];
        let mut out = Vec::new();
        let choice = choose(&items, split.then_some("one fixup! per commit"), input.as_bytes(), &mut out);
        (choice, String::from_utf8(out).unwrap())
    }

    #[test]
    fn lists_items_and_returns_the_chosen_index() {
        let (choice, out) = ask("2\n", true);
        assert_eq!(choice, Choice::Item(1));
        assert_eq!(out, "  1) abc one\n  2) def two\n  s) split: one fixup! per commit\npick [1-2, s, q]: ");
    }

    #[test]
    fn asks_again_after_an_answer_that_is_not_listed() {
        let (choice, out) = ask("0\n3\nx\ns\n1\n", false);
        assert_eq!(choice, Choice::Item(0));
        assert_eq!(out.matches("pick [1-2, q]: ").count(), 5);
        assert!(!out.contains("s) split"));
    }

    #[test]
    fn split_is_offered_only_when_described() {
        assert_eq!(ask("s\n", true).0, Choice::Split);
    }

    #[test]
    fn empty_answer_q_and_end_of_input_cancel() {
        for input in ["\n", "q\n", " q \n", ""] {
            assert_eq!(ask(input, true).0, Choice::Cancel, "{input:?}");
        }
    }
}
