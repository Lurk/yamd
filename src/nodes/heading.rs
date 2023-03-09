use crate::sd::{
    context::Context,
    deserializer::Deserializer,
    node::Node,
    tokenizer::{Pattern::Once, Pattern::RepeatTimes, Tokenizer},
};

#[derive(Debug, PartialEq)]
pub struct Heading {
    level: u8,
    text: String,
}

impl Heading {
    pub fn new<S: Into<String>>(text: S, level: u8) -> Self {
        let normalized_level = match level {
            0 => 1,
            7.. => 6,
            l => l,
        };
        Heading {
            text: text.into(),
            level: normalized_level,
        }
    }
}

impl Deserializer for Heading {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let start_tokens = [
            vec![Once('#'), Once(' ')],
            vec![RepeatTimes(2, '#'), Once(' ')],
            vec![RepeatTimes(3, '#'), Once(' ')],
            vec![RepeatTimes(4, '#'), Once(' ')],
            vec![RepeatTimes(5, '#'), Once(' ')],
            vec![RepeatTimes(6, '#'), Once(' ')],
        ];

        for (i, start_token) in start_tokens.iter().enumerate() {
            let mut tokenizer = Tokenizer::new(input);
            if let Some(body) = tokenizer.get_token_body_with_end_of_input(
                start_token.to_vec(),
                vec![Once('\n'), Once('\n')],
                true,
            ) {
                return Some(Self::new(body, (i + 1).try_into().unwrap_or(1)));
            }
        }

        None
    }
}

impl Node for Heading {
    fn len(&self) -> usize {
        self.text.len() + self.level as usize + 1
    }
    fn serialize(&self) -> String {
        let level = String::from('#').repeat(self.level as usize);
        format!("{} {}", level, self.text)
    }
}

#[cfg(test)]
mod tests {
    use crate::sd::{deserializer::Deserializer, node::Node};

    use super::Heading;

    #[test]
    fn level_one() {
        let h = Heading::new("Header", 1).serialize();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_gt_six() {
        let h = Heading::new("Header", 7).serialize();
        assert_eq!(h, "###### Header");
        let h = Heading::new("Header", 34).serialize();
        assert_eq!(h, "###### Header");
    }

    #[test]
    fn level_eq_zero() {
        let h = Heading::new("Header", 0).serialize();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_eq_four() {
        let h = Heading::new("Header", 4).serialize();
        assert_eq!(h, "#### Header");
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Heading::deserialize("## Header"),
            Some(Heading::new("Header", 2))
        );
        assert_eq!(
            Heading::deserialize("### Head"),
            Some(Heading::new("Head", 3))
        );
        assert_eq!(
            Heading::deserialize("### Head\n\nsome other thing"),
            Some(Heading::new("Head", 3))
        );
        assert_eq!(Heading::deserialize("not a header"), None);
        assert_eq!(Heading::deserialize("######"), None);
        assert_eq!(Heading::deserialize("######also not a header"), None);
    }

    #[test]
    fn len() {
        assert_eq!(Heading::new("h", 1).len(), 3);
        assert_eq!(Heading::new("h", 2).len(), 4);
    }
}
