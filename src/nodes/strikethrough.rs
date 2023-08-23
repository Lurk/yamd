use crate::{
    toolkit::{context::Context, deserializer::Deserializer},
    toolkit::{matcher::Matcher, node::Node},
};

/// Representation of strike through
#[derive(Debug, PartialEq)]
pub struct Strikethrough {
    pub text: String,
}

impl Strikethrough {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        Strikethrough { text: text.into() }
    }
}

impl Node<'_> for Strikethrough {
    fn serialize(&self) -> String {
        format!("~~{}~~", self.text)
    }
    fn len(&self) -> usize {
        self.text.len() + 4
    }
}

impl<'text> Deserializer<'text> for Strikethrough {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(strikethrough) = matcher.get_match("~~", "~~", false) {
            return Some(Strikethrough::new(strikethrough.body));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Strikethrough;
    use crate::toolkit::{deserializer::Deserializer, node::Node};
    use pretty_assertions::assert_eq;

    #[test]
    fn happy_path() {
        let s = Strikethrough::new("2+2=5");
        assert_eq!(s.text, "2+2=5".to_string());
    }

    #[test]
    fn to_string() {
        let s: String = Strikethrough::new("2+2=5").serialize();
        assert_eq!(s, "~~2+2=5~~".to_string());
    }

    #[test]
    fn parse() {
        assert_eq!(
            Strikethrough::deserialize("~~2+2=5~~"),
            Some(Strikethrough::new("2+2=5"))
        );
        assert_eq!(
            Strikethrough::deserialize("~~is~~not"),
            Some(Strikethrough::new("is"))
        );
        assert_eq!(Strikethrough::deserialize("~~not"), None);
        assert_eq!(
            Strikethrough::deserialize("~~i\ns~~"),
            Some(Strikethrough::new("i\ns"))
        );
    }

    #[test]
    fn len() {
        assert_eq!(Strikethrough::new("s").len(), 5);
        assert_eq!(Strikethrough::new("st").len(), 6);
    }
}
