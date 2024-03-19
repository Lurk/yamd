use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, parser::Parse};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Image {
    pub alt: String,
    pub src: String,
}

impl Image {
    pub fn new<S: Into<String>>(alt: S, src: S) -> Self {
        Self {
            alt: alt.into(),
            src: src.into(),
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "![{}]({})", self.alt, self.src)
    }
}

impl Parse for Image {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        if input[current_position..].starts_with("![") {
            if let Some(middle) = input[current_position + 1..].find("](") {
                let mut level = 1;
                for (i, c) in input[current_position + middle + 2..].char_indices() {
                    if c == '(' {
                        level += 1;
                    } else if c == ')' {
                        level -= 1;
                    }
                    if level == 0 {
                        return Some((
                            Image::new(
                                &input[current_position + 1..current_position + middle],
                                &input[current_position + middle + 2
                                    ..current_position + middle + 2 + i],
                            ),
                            middle + 2 + i + 1 - current_position,
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

    use super::Image;
    use pretty_assertions::assert_eq;

    #[test]
    fn serializer() {
        assert_eq!(Image::new('a', 'u').to_string(), String::from("![a](u)"));
    }

    #[test]
    fn parser() {
        assert_eq!(
            Image::parse("![alt](url)", 0, None),
            Some((Image::new("alt", "url"), 11))
        );
        assert_eq!(Image::parse("![alt](url", 0, None), None);
        assert_eq!(Image::parse("[alt](url)", 0, None), None);
    }
}
