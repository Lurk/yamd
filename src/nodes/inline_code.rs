use crate::{
    nodes::paragraph::ParagraphNodes,
    sd::deserializer::{Deserializer, Node, Tokenizer},
    sd::serializer::Serializer,
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

impl Serializer for InlineCode {
    fn serialize(&self) -> String {
        format!("`{}`", self.text)
    }
}

impl From<InlineCode> for ParagraphNodes {
    fn from(value: InlineCode) -> Self {
        ParagraphNodes::InlineCode(value)
    }
}

impl Node for InlineCode {
    fn len(&self) -> usize {
        self.text.len() + 2
    }
}

impl Deserializer for InlineCode {
    fn deserialize(input: &str) -> Option<Self> {
        let mut chars = Tokenizer::new(input);
        if let Some(body) = chars.get_token_body(vec!['`'], vec!['`']) {
            return Some(InlineCode::new(body.to_string().replace('\n', "")));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::sd::{deserializer::Deserializer, serializer::Serializer};

    use super::InlineCode;

    #[test]
    fn to_string() {
        let inline_code: String = InlineCode::new("const bar = 'baz'").serialize();
        assert_eq!(inline_code, "`const bar = 'baz'`".to_string())
    }

    #[test]
    fn from_string() {
        assert_eq!(InlineCode::deserialize("`1`"), Some(InlineCode::new('1')));
        assert_eq!(
            InlineCode::deserialize("`const \nfoo='bar'`"),
            Some(InlineCode::new("const foo='bar'"))
        );
        assert_eq!(InlineCode::deserialize("`a"), None);
        assert_eq!(InlineCode::deserialize("`const \n\nfoo='bar'`"), None);
    }
}
