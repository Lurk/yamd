use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::parser::Parse;

#[derive(Debug, PartialEq, Serialize, Clone, Default)]
pub struct Divider {}

impl Divider {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for Divider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-----")
    }
}

impl Parse for Divider {
    fn parse(input: &str, current_position: usize) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if input[current_position..].starts_with("-----") {
            Some((Divider::new(), 5))
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{nodes::divider::Divider, toolkit::parser::Parse};
    use pretty_assertions::assert_eq;

    #[test]
    fn parse() {
        assert_eq!(Divider::parse("-----", 0), Some((Divider {}, 5)));
    }

    #[test]
    fn serialize() {
        assert_eq!(Divider::new().to_string(), String::from("-----"));
    }
}
