use crate::toolkit::{
    context::Context, deserializer::Deserializer, matcher::Matcher, node::Node,
    pattern::Quantifiers::*,
};

#[derive(Debug, PartialEq)]
pub struct Heading {
    level: u8,
    text: String,
    consumed_all_input: bool,
}

impl Heading {
    pub fn new<S: Into<String>>(text: S, level: u8, consumed_all_input: bool) -> Self {
        let normalized_level = match level {
            0 => 1,
            7.. => 6,
            l => l,
        };
        Heading {
            text: text.into(),
            level: normalized_level,
            consumed_all_input,
        }
    }
}

impl Deserializer for Heading {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let start_tokens = [
            [RepeatTimes(6, '#'), Once(' ')],
            [RepeatTimes(5, '#'), Once(' ')],
            [RepeatTimes(4, '#'), Once(' ')],
            [RepeatTimes(3, '#'), Once(' ')],
            [RepeatTimes(2, '#'), Once(' ')],
            [Once('#'), Once(' ')],
        ];

        for (i, start_token) in start_tokens.iter().enumerate() {
            let mut matcher = Matcher::new(input);
            if let Some(heading) = matcher.get_match(start_token, &[RepeatTimes(2, '\n')], true) {
                return Some(Self::new(
                    heading.body,
                    (start_tokens.len() - i).try_into().unwrap_or(1),
                    heading.end_token.is_empty(),
                ));
            }
        }

        None
    }
}

impl Node for Heading {
    fn serialize(&self) -> String {
        let level = String::from('#').repeat(self.level as usize);
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        format!("{} {}{end}", level, self.text)
    }
    fn len(&self) -> usize {
        let end = if self.consumed_all_input { 0 } else { 2 };
        self.text.len() + self.level as usize + 1 + end
    }
}

#[cfg(test)]
mod tests {
    use super::Heading;
    use crate::toolkit::{deserializer::Deserializer, node::Node};
    use pretty_assertions::assert_eq;

    #[test]
    fn level_one() {
        assert_eq!(Heading::new("Header", 1, true).serialize(), "# Header");
        assert_eq!(Heading::new("Header", 1, false).serialize(), "# Header\n\n");
    }

    #[test]
    fn level_gt_six() {
        let h = Heading::new("Header", 7, true).serialize();
        assert_eq!(h, "###### Header");
        let h = Heading::new("Header", 34, true).serialize();
        assert_eq!(h, "###### Header");
    }

    #[test]
    fn level_eq_zero() {
        let h = Heading::new("Header", 0, true).serialize();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_eq_four() {
        let h = Heading::new("Header", 4, true).serialize();
        assert_eq!(h, "#### Header");
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Heading::deserialize("## Header"),
            Some(Heading::new("Header", 2, true))
        );
        assert_eq!(
            Heading::deserialize("### Head"),
            Some(Heading::new("Head", 3, true))
        );
        assert_eq!(
            Heading::deserialize("### Head\n\nsome other thing"),
            Some(Heading::new("Head", 3, false))
        );
        assert_eq!(Heading::deserialize("not a header"), None);
        assert_eq!(Heading::deserialize("######"), None);
        assert_eq!(Heading::deserialize("######also not a header"), None);
    }

    #[test]
    fn len() {
        assert_eq!(Heading::new("h", 1, true).len(), 3);
        assert_eq!(Heading::new("h", 2, true).len(), 4);
        assert_eq!(Heading::new("h", 2, false).len(), 6);
    }
}
