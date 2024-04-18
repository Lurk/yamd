use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::parser::Parse;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Code {
    pub lang: String,
    pub code: String,
}

impl Code {
    pub fn new<S: Into<String>>(lang: S, code: S) -> Self {
        Self {
            lang: lang.into(),
            code: code.into(),
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "```{}\n{}\n```", self.lang, self.code)
    }
}

impl Parse for Code {
    fn parse(input: &str, current_position: usize) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if input[current_position..].starts_with("```") {
            if let Some(lang) = input[current_position + 3..].find('\n') {
                if let Some(end) = input[current_position + 3 + lang + 1..].find("\n```") {
                    return Some((
                        Code::new(
                            &input[current_position + 3..current_position + 3 + lang],
                            &input[current_position + 3 + lang + 1
                                ..current_position + 3 + lang + 1 + end],
                        ),
                        3 + lang + 1 + end + 4,
                    ));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{nodes::code::Code, toolkit::parser::Parse};
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize() {
        assert_eq!(
            Code::new("rust", "let foo:usize=1;").to_string(),
            String::from("```rust\nlet foo:usize=1;\n```")
        );
    }

    #[test]
    fn parser() {
        assert_eq!(
            Code::parse("```rust\nlet a=1;\n```", 0),
            Some((Code::new("rust", "let a=1;"), 20))
        );
        assert_eq!(Code::parse("```rust\nlet a=1;\n", 0), None);
        assert_eq!(Code::parse("not a code block", 0), None);
        assert_eq!(Code::parse("``````", 0), None);
    }
}
