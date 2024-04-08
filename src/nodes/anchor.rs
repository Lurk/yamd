use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::toolkit::{context::Context, parser::Parse};

/// Representation of an anchor
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Anchor {
    pub text: String,
    pub url: String,
}

impl Anchor {
    pub fn new<S: Into<String>>(text: S, url: S) -> Self {
        Anchor {
            text: text.into(),
            url: url.into(),
        }
    }
}

impl Display for Anchor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]({})", self.text, self.url)
    }
}

impl Parse for Anchor {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        if input[current_position..].starts_with('[') {
            if let Some(middle) = input[current_position + 1..].find("](") {
                let mut level = 1;
                for (i, c) in input[current_position + middle + 3..].char_indices() {
                    if c == '(' {
                        level += 1;
                    } else if c == ')' {
                        level -= 1;
                    }
                    if level == 0 {
                        return Some((
                            Anchor::new(
                                &input[current_position + 1..current_position + middle + 1],
                                &input[current_position + middle + 3
                                    ..current_position + middle + 3 + i],
                            ),
                            middle + 3 + i + 1,
                        ));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::parser::Parse;

    use super::Anchor;
    use pretty_assertions::assert_eq;

    #[test]
    fn happy_path() {
        let a = Anchor::new("nice link", "https://test.io");
        assert_eq!(a.text, "nice link");
        assert_eq!(a.url, "https://test.io");
    }

    #[test]
    fn serialize() {
        let a: String = Anchor::new("nice link", "https://test.io").to_string();
        assert_eq!(a, "[nice link](https://test.io)".to_string());
    }

    #[test]
    fn parse() {
        assert_eq!(
            Anchor::parse("[1](2)", 0, None),
            Some((Anchor::new("1", "2"), 6))
        );
        assert_eq!(Anchor::parse("[1", 0, None), None);
        assert_eq!(Anchor::parse("[1](2", 0, None), None);
    }

    #[test]
    fn deserilalze_with_parentesis_in_url() {
        assert_eq!(
            Anchor::parse(
                "[the Rope data structure](https://en.wikipedia.org/wiki/Rope_(data_structure))",
                0,
                None
            ),
            Some((
                Anchor::new(
                    "the Rope data structure",
                    "https://en.wikipedia.org/wiki/Rope_(data_structure)"
                ),
                78
            ))
        );
    }
}
