use crate::toolkit::{
    context::Context,
    deserializer::Deserializer,
    node::Node,
    tokenizer::{
        Matcher,
        Quantifiers::{Once, RepeatTimes},
    },
};

#[derive(Debug, PartialEq)]
pub struct Code {
    lang: String,
    code: String,
    consumed_all_input: bool,
}

impl Code {
    pub fn new<S: Into<String>>(lang: S, code: S, consumed_all_input: bool) -> Self {
        Self {
            lang: lang.into(),
            code: code.into(),
            consumed_all_input,
        }
    }
}

impl Node for Code {
    fn serialize(&self) -> String {
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        format!("```{}\n{}\n```{end}", self.lang, self.code)
    }
    fn len(&self) -> usize {
        let end = if self.consumed_all_input { 0 } else { 2 };
        self.lang.len() + self.code.len() + 8 + end
    }
}

impl Deserializer for Code {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(lang) =
            matcher.get_match(&[Once('`'), Once('`'), Once('`')], &[Once('\n')], false)
        {
            if let Some(code) =
                matcher.get_match(&[], &[Once('\n'), Once('`'), Once('`'), Once('`')], false)
            {
                let consumed_all_input = matcher
                    .get_match(&[RepeatTimes(2, '\n')], &[], false)
                    .is_none();
                return Some(Self::new(lang.body, code.body, consumed_all_input));
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
            Code::new("rust", "let foo:usize=1;", true).serialize(),
            String::from("```rust\nlet foo:usize=1;\n```")
        );
        assert_eq!(
            Code::new("rust", "let foo:usize=1;", false).serialize(),
            String::from("```rust\nlet foo:usize=1;\n```\n\n")
        );
    }

    #[test]
    fn len() {
        assert_eq!(Code::new('r', 'b', true).len(), 10);
        assert_eq!(Code::new('r', 'b', false).len(), 12);
    }

    #[test]
    fn deserializer() {
        assert_eq!(
            Code::deserialize("```rust\nlet a=1;\n```"),
            Some(Code::new("rust", "let a=1;", true))
        );
        assert_eq!(
            Code::deserialize("```rust\nlet a=1;\n```\n\n"),
            Some(Code::new("rust", "let a=1;", false))
        );
        assert_eq!(Code::deserialize("```rust\nlet a=1;\n"), None);
    }
}
