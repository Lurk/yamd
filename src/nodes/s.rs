use crate::{
    nodes::bold::BoldNodes,
    nodes::paragraph::ParagraphNodes,
    sd::deserializer::{Deserializer, Node, Tokenizer},
    sd::serializer::Serializer,
};

/// Representation of strikethrough
#[derive(Debug, PartialEq)]
pub struct S {
    text: String,
}

impl S {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        S { text: text.into() }
    }
}

impl Serializer for S {
    fn serialize(&self) -> String {
        format!("~~{}~~", self.text)
    }
}

impl From<S> for BoldNodes {
    fn from(value: S) -> Self {
        BoldNodes::S(value)
    }
}

impl From<S> for ParagraphNodes {
    fn from(value: S) -> Self {
        ParagraphNodes::S(value)
    }
}

impl Node for S {
    fn len(&self) -> usize {
        self.text.len() + 4
    }
}

impl Deserializer for S {
    fn deserialize(input: &str) -> Option<Self> {
        let mut chars = Tokenizer::new(input);
        if let Some(body) = chars.get_token_body(vec!['~', '~'], vec!['~', '~']) {
            return Some(S::new(body.to_string().replace('\n', "")));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::sd::{
        deserializer::{Deserializer, Node},
        serializer::Serializer,
    };

    use super::S;

    #[test]
    fn happy_path() {
        let s = S::new("2+2=5");
        assert_eq!(s.text, "2+2=5".to_string());
    }

    #[test]
    fn to_string() {
        let s: String = S::new("2+2=5").serialize();
        assert_eq!(s, "~~2+2=5~~".to_string());
    }

    #[test]
    fn parse() {
        assert_eq!(S::deserialize("~~2+2=5~~"), Some(S::new("2+2=5")));
        assert_eq!(S::deserialize("~~is~~not"), Some(S::new("is")));
        assert_eq!(S::deserialize("~~not"), None);
        assert_eq!(S::deserialize("~~i\ns~~"), Some(S::new("is")));
    }

    #[test]
    fn len() {
        assert_eq!(S::new("s").len(), 5);
        assert_eq!(S::new("st").len(), 6);
    }
}
