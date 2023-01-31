use crate::{b::BContent, p::ParagraphTags};

/// Representation of a regular text
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
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
}
