use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq)]
pub struct Code<'text> {
    pub lang: &'text str,
    pub code: &'text str,
    consumed_all_input: bool,
}

impl<'text> Code<'text> {
    pub fn new(consumed_all_input: bool, lang: &'text str, code: &'text str) -> Self {
        Self {
            lang,
            code,
            consumed_all_input,
        }
    }
}

impl<'text> Node<'text> for Code<'text> {
    fn serialize(&self) -> String {
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        format!("```{}\n{}\n```{end}", self.lang, self.code)
    }
    fn len(&self) -> usize {
        let end = if self.consumed_all_input { 0 } else { 2 };
        self.lang.len() + self.code.len() + 8 + end
    }
}

impl<'text> Deserializer<'text> for Code<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
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
            Code::new(true, "rust", "let foo:usize=1;").serialize(),
            String::from("```rust\nlet foo:usize=1;\n```")
        );
        assert_eq!(
            Code::new(false, "rust", "let foo:usize=1;").serialize(),
            String::from("```rust\nlet foo:usize=1;\n```\n\n")
        );
    }

    #[test]
    fn len() {
        assert_eq!(Code::new(true, "r", "b").len(), 10);
        assert_eq!(Code::new(false, "r", "b").len(), 12);
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
