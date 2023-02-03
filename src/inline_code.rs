use crate::{
    p::ParagraphTags,
    parser::{Deserializer, Leaf, Tokenizer},
};

#[derive(Debug, PartialEq)]
pub struct InlineCode {
    text: String,
}

impl InlineCode {
    pub fn new<S: Into<String>>(text: S) -> Self {
        InlineCode { text: text.into() }
    }
}

impl From<InlineCode> for String {
    fn from(value: InlineCode) -> Self {
        format!("`{}`", value.text)
    }
}

impl From<InlineCode> for ParagraphTags {
    fn from(value: InlineCode) -> Self {
        ParagraphTags::InlineCode(value)
    }
}

impl Leaf for InlineCode {}

impl Deserializer for InlineCode {
    fn deserialize(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = Tokenizer::new(input, start_position);
        if let Some(body) = chars.get_token_body(vec!['`'], vec!['`']) {
            return Some((
                InlineCode::new(body.to_string().replace('\n', "")),
                chars.get_next_position(),
            ));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Deserializer;

    use super::InlineCode;

    #[test]
    fn to_string() {
        let inline_code: String = InlineCode::new("const bar = 'baz'").into();
        assert_eq!(inline_code, "`const bar = 'baz'`".to_string())
    }

    #[test]
    fn from_string() {
        assert_eq!(
            InlineCode::deserialize("`1`", 0),
            Some((InlineCode::new('1'), 3))
        );
        assert_eq!(
            InlineCode::deserialize("not`1`", 3),
            Some((InlineCode::new('1'), 6))
        );
        assert_eq!(
            InlineCode::deserialize("`const \nfoo='bar'`", 0),
            Some((InlineCode::new("const foo='bar'"), 18))
        );
        assert_eq!(InlineCode::deserialize("not`a", 3), None);
        assert_eq!(InlineCode::deserialize("`const \n\nfoo='bar'`", 0), None);
    }
}
