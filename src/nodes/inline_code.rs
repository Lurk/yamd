use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq)]
pub struct InlineCode<'text> {
    pub text: &'text str,
}

impl<'text> InlineCode<'text> {
    pub fn new(text: &'text str) -> Self {
        InlineCode { text }
    }
}

impl<'text> Node<'text> for InlineCode<'text> {
    fn serialize(&self) -> String {
        format!("`{}`", self.text)
    }
    fn len(&self) -> usize {
        self.text.len() + 2
    }
}

impl<'text> Deserializer<'text> for InlineCode<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(inline_code) = matcher.get_match("`", "`", false) {
            return Some(InlineCode::new(inline_code.body));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::InlineCode;
    use crate::toolkit::{deserializer::Deserializer, node::Node};
    use pretty_assertions::assert_eq;

    #[test]
    fn to_string() {
        let inline_code: String = InlineCode::new("const bar = 'baz'").serialize();
        assert_eq!(inline_code, "`const bar = 'baz'`".to_string())
    }

    #[test]
    fn from_string() {
        assert_eq!(InlineCode::deserialize("`1`"), Some(InlineCode::new("1")));
        assert_eq!(
            InlineCode::deserialize("`const \nfoo='bar'`"),
            Some(InlineCode::new("const \nfoo='bar'"))
        );
        assert_eq!(InlineCode::deserialize("`a"), None);
    }
}
