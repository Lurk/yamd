use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq, Serialize)]
pub struct Code {
    pub lang: String,
    pub code: String,
    consumed_all_input: bool,
}

impl Code {
    pub fn new<S: Into<String>>(consumed_all_input: bool, lang: S, code: S) -> Self {
        Self {
            lang: lang.into(),
            code: code.into(),
            consumed_all_input,
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        write!(f, "```{}\n{}\n```{}", self.lang, self.code, end)
    }
}

impl Node for Code {
    fn len(&self) -> usize {
        let end = if self.consumed_all_input { 0 } else { 2 };
        self.lang.len() + self.code.len() + 8 + end
    }
}

impl Deserializer for Code {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(lang) = matcher.get_match("```", "\n", false) {
            if let Some(code) = matcher.get_match("", "\n```", false) {
                let consumed_all_input = matcher.get_match("\n\n", "", false).is_none();
                return Some(Self::new(consumed_all_input, lang.body, code.body));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::code::Code,
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize() {
        assert_eq!(
            Code::new(true, "rust", "let foo:usize=1;").to_string(),
            String::from("```rust\nlet foo:usize=1;\n```")
        );
        assert_eq!(
            Code::new(false, "rust", "let foo:usize=1;").to_string(),
            String::from("```rust\nlet foo:usize=1;\n```\n\n")
        );
    }

    #[test]
    fn len() {
        assert_eq!(Code::new(true, 'r', 'b').len(), 10);
        assert_eq!(Code::new(false, 'r', 'b').len(), 12);
    }

    #[test]
    fn deserializer() {
        assert_eq!(
            Code::deserialize("```rust\nlet a=1;\n```"),
            Some(Code::new(true, "rust", "let a=1;"))
        );
        assert_eq!(
            Code::deserialize("```rust\nlet a=1;\n```\n\n"),
            Some(Code::new(false, "rust", "let a=1;"))
        );
        assert_eq!(Code::deserialize("```rust\nlet a=1;\n"), None);
    }
}
