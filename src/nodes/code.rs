use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

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

impl Node for Code {
    fn len(&self) -> usize {
        self.lang.len() + self.code.len() + 8
    }
}

impl Deserializer for Code {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(lang) = matcher.get_match("```", "\n", false) {
            if let Some(code) = matcher.get_match("", "\n```", false) {
                return Some(Self::new(lang.body, code.body));
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
            Code::new("rust", "let foo:usize=1;").to_string(),
            String::from("```rust\nlet foo:usize=1;\n```")
        );
    }

    #[test]
    fn len() {
        assert_eq!(Code::new('r', 'b').len(), 10);
    }

    #[test]
    fn deserializer() {
        assert_eq!(
            Code::deserialize("```rust\nlet a=1;\n```"),
            Some(Code::new("rust", "let a=1;"))
        );
        assert_eq!(
            Code::deserialize("```rust\nlet a=1;\n```\n\n"),
            Some(Code::new("rust", "let a=1;"))
        );
        assert_eq!(Code::deserialize("```rust\nlet a=1;\n"), None);
    }
}
