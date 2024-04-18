use crate::toolkit::parser::Parse;
use serde::Serialize;
use std::fmt::{Display, Formatter};

/// Representation of strike through
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Strikethrough {
    pub text: String,
}

impl Strikethrough {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        Strikethrough { text: text.into() }
    }
}

impl Parse for Strikethrough {
    fn parse(input: &str, current_position: usize) -> Option<(Self, usize)> {
        if input[current_position..].starts_with("~~") {
            if let Some(end) = input[current_position + 2..].find("~~") {
                return Some((
                    Strikethrough::new(&input[current_position + 2..current_position + 2 + end]),
                    end + 4,
                ));
            }
        }
        None
    }
}

impl Display for Strikethrough {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "~~{}~~", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::Strikethrough;
    use crate::toolkit::parser::Parse;
    use pretty_assertions::assert_eq;

    #[test]
    fn happy_path() {
        let s = Strikethrough::new("2+2=5");
        assert_eq!(s.text, "2+2=5".to_string());
    }

    #[test]
    fn to_string() {
        let s: String = Strikethrough::new("2+2=5").to_string();
        assert_eq!(s, "~~2+2=5~~".to_string());
    }

    #[test]
    fn parse() {
        assert_eq!(
            Strikethrough::parse("~~2+2=5~~", 0),
            Some((Strikethrough::new("2+2=5"), 9))
        );
        assert_eq!(
            Strikethrough::parse("~~is~~not", 0),
            Some((Strikethrough::new("is"), 6))
        );
        assert_eq!(Strikethrough::parse("~~not", 0), None);
        assert_eq!(
            Strikethrough::parse("~~i\ns~~", 0),
            Some((Strikethrough::new("i\ns"), 7))
        );
    }
}
