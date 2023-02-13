use crate::{
    nodes::bold::BoldNodes,
    nodes::paragraph::ParagraphNodes,
    sd::deserializer::{Deserializer, Node, Pattern::Exact, Tokenizer},
    sd::serializer::Serializer,
};

/// Representation of an Italic text
#[derive(Debug, PartialEq)]
pub struct Italic {
    text: String,
}

impl Italic {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Italic { text: text.into() }
    }
}

impl Serializer for Italic {
    fn serialize(&self) -> String {
        format!("_{}_", self.text)
    }
}

impl From<Italic> for BoldNodes {
    fn from(value: Italic) -> Self {
        BoldNodes::I(value)
    }
}

impl From<Italic> for ParagraphNodes {
    fn from(value: Italic) -> Self {
        ParagraphNodes::I(value)
    }
}

impl Node for Italic {
    fn len(&self) -> usize {
        self.text.len() + 2
    }
}

impl Deserializer for Italic {
    fn deserialize(input: &str) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(body) = tokenizer.get_token_body(vec![Exact('_')], vec![Exact('_')]) {
            return Some(Italic::new(body));
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

    use super::Italic;

    #[test]
    fn happy_path() {
        let i = Italic::new("italic");
        assert_eq!(i.text, "italic".to_string());
    }

    #[test]
    fn to_string() {
        let i = Italic::new("italic").serialize();
        assert_eq!(i, "_italic_".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Italic::deserialize("_italic_"), Some(Italic::new("italic")));
        assert_eq!(
            Italic::deserialize("_italic_not"),
            Some(Italic::new("italic"))
        );
        assert_eq!(
            Italic::deserialize("_it alic_not"),
            Some(Italic::new("it alic"))
        );
        assert_eq!(Italic::deserialize("not italic_not"), None);
        assert_eq!(Italic::deserialize("*italic not"), None);
        assert_eq!(
            Italic::deserialize("_ita\nlic_"),
            Some(Italic::new("ita\nlic"))
        );
    }

    #[test]
    fn len() {
        assert_eq!(Italic::new("i").len(), 3);
    }
}
