use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::{
    toolkit::{context::Context, deserializer::Deserializer},
    toolkit::{matcher::Matcher, node::Node},
};

/// Representation of an Italic text
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Italic {
    pub text: String,
}

impl Italic {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Italic { text: text.into() }
    }
}

impl Display for Italic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "_{}_", self.text)
    }
}

impl Node for Italic {
    fn len(&self) -> usize {
        self.text.len() + 2
    }
}

impl Deserializer for Italic {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(italic) = matcher.get_match("_", "_", false) {
            return Some(Italic::new(italic.body));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Italic;
    use crate::toolkit::{deserializer::Deserializer, node::Node};
    use pretty_assertions::assert_eq;

    #[test]
    fn happy_path() {
        let i = Italic::new("italic");
        assert_eq!(i.text, "italic".to_string());
    }

    #[test]
    fn to_string() {
        let i = Italic::new("italic").to_string();
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
