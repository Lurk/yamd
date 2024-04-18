use crate::toolkit::parser::Parse;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        Text { text: text.into() }
    }
}

impl Parse for Text {
    fn parse(input: &str, current_position: usize) -> Option<(Self, usize)> {
        Some((
            Text::new(&input[current_position..]),
            input.len() - current_position,
        ))
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::parser::Parse;

    use super::Text;
    use pretty_assertions::assert_eq;

    #[test]
    fn happy_path() {
        let text = Text::new("shiny text");
        assert_eq!(text.text, "shiny text".to_string());
    }

    #[test]
    fn to_string() {
        let text: String = Text::new("shiny text").to_string();
        assert_eq!(text, "shiny text".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(Text::parse("t", 0), Some((Text::new("t"), 1)));
    }
}
