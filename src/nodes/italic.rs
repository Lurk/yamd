use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::toolkit::{context::Context, parser::Parse};

/// Representation of an Italic text
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Italic {
    text: String,
}

impl Italic {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        Italic { text: text.into() }
    }
}

impl Parse for Italic {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        if input[current_position..].starts_with('_') {
            if let Some(end) = input[current_position + 1..].find('_') {
                return Some((
                    Italic::new(&input[current_position + 1..current_position + 1 + end]),
                    end + 2 - current_position,
                ));
            }
        }
        None
    }
}

impl Display for Italic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "_{}_", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::Italic;
    use crate::toolkit::parser::Parse;
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
        assert_eq!(
            Italic::parse("_italic_", 0, None),
            Some((Italic::new("italic"), 8))
        );
        assert_eq!(
            Italic::parse("_italic_not", 0, None),
            Some((Italic::new("italic"), 8))
        );
        assert_eq!(
            Italic::parse("_it alic_not", 0, None),
            Some((Italic::new("it alic"), 9))
        );
        assert_eq!(Italic::parse("not italic_not", 0, None), None);
        assert_eq!(Italic::parse("*italic not", 0, None), None);
        assert_eq!(
            Italic::parse("_ita\nlic_", 0, None),
            Some((Italic::new("ita\nlic"), 9))
        );
    }
}
