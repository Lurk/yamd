use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

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

impl Node for Divider {
    fn len(&self) -> usize {
        5
    }
}

impl Deserializer for Divider {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if matcher.get_match("-----", "\n\n", true).is_some() {
            return Some(Divider {});
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::divider::Divider,
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn deserialize() {
        assert_eq!(Divider::deserialize("-----"), Some(Divider {}));
        assert_eq!(Divider::deserialize("----\n\n"), None);
    }

    #[test]
    fn len() {
        assert_eq!(Divider::new().len(), 5);
    }

    #[test]
    fn serialize() {
        assert_eq!(Divider::new().to_string(), String::from("-----"));
    }
}
