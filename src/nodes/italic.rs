use crate::{
    toolkit::{context::Context, deserializer::Deserializer},
    toolkit::{
        node::Node,
        tokenizer::{Pattern::Once, Tokenizer},
    },
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

impl Node for Italic {
    fn len(&self) -> usize {
        self.text.len() + 2
    }
    fn serialize(&self) -> String {
        format!("_{}_", self.text)
    }
}

impl Deserializer for Italic {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(body) = tokenizer.get_token_body(&[Once('_')], &[Once('_')]) {
            return Some(Italic::new(body));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::{deserializer::Deserializer, node::Node};

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
