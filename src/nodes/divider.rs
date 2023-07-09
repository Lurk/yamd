use crate::toolkit::{
    context::Context, deserializer::Deserializer, node::Node, pattern::Quantifiers::*,
    tokenizer::Matcher,
};

#[derive(Debug, PartialEq)]
pub struct Divider {
    consumed_all_input: bool,
}

impl Divider {
    pub fn new(consumed_all_input: bool) -> Self {
        Self { consumed_all_input }
    }
}

impl Node for Divider {
    fn serialize(&self) -> String {
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        format!("-----{end}")
    }

    fn len(&self) -> usize {
        if self.consumed_all_input {
            5
        } else {
            7
        }
    }
}

impl Deserializer for Divider {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(divider) =
            matcher.get_match(&[RepeatTimes(5, '-')], &[RepeatTimes(2, '\n')], true)
        {
            return Some(Divider {
                consumed_all_input: divider.end_token.is_empty(),
            });
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
        assert_eq!(
            Divider::deserialize("-----"),
            Some(Divider {
                consumed_all_input: true
            })
        );
        assert_eq!(
            Divider::deserialize("-----\n\n"),
            Some(Divider {
                consumed_all_input: false
            })
        );
        assert_eq!(Divider::deserialize("----\n\n"), None);
    }

    #[test]
    fn len() {
        assert_eq!(Divider::new(true).len(), 5);
        assert_eq!(Divider::new(false).len(), 7);
    }

    #[test]
    fn serialize() {
        assert_eq!(Divider::new(true).serialize(), String::from("-----"));
        assert_eq!(Divider::new(false).serialize(), String::from("-----\n\n"));
    }
}
