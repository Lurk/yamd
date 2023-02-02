use crate::{b::BContent, p::ParagraphTags, parser::Parser};

/// Representation of a regular text
#[derive(Debug, PartialEq)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Text { text: text.into() }
    }
}

impl From<Text> for String {
    fn from(value: Text) -> Self {
        value.text
    }
}

impl From<Text> for BContent {
    fn from(value: Text) -> Self {
        BContent::Text(value)
    }
}

impl From<Text> for ParagraphTags {
    fn from(value: Text) -> Self {
        ParagraphTags::Text(value)
    }
}

impl Parser for Text {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)> {
        Some((Text::new(input[start_position..].to_string()), input.len()))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::Text;

    #[test]
    fn happy_path() {
        let text = Text::new("shiny text");
        assert_eq!(text.text, "shiny text".to_string());
    }

    #[test]
    fn to_string() {
        let text: String = Text::new("shiny text").into();
        assert_eq!(text, "shiny text".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Text::parse("t", 0), Some((Text::new("t"), 1)));
    }
}
