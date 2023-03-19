use crate::{
    toolkit::{context::Context, deserializer::Deserializer},
    toolkit::{
        node::Node,
        tokenizer::{Pattern::Once, Tokenizer},
    },
};

/// Representation of strikethrough
#[derive(Debug, PartialEq)]
pub struct Strikethrough {
    text: String,
}

impl Strikethrough {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        Strikethrough { text: text.into() }
    }
}

impl Node for Strikethrough {
    fn len(&self) -> usize {
        self.text.len() + 4
    }
    fn serialize(&self) -> String {
        format!("~~{}~~", self.text)
    }
}

impl Deserializer for Strikethrough {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(body) =
            tokenizer.get_token_body(vec![Once('~'), Once('~')], vec![Once('~'), Once('~')])
        {
            return Some(Strikethrough::new(body));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::{deserializer::Deserializer, node::Node};

    use super::Strikethrough;

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
