use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq, Serialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    consumed_all_input: bool,
}

impl Heading {
    pub fn new<S: Into<String>>(consumed_all_input: bool, text: S, level: u8) -> Self {
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
        let start_tokens = ["###### ", "##### ", "#### ", "### ", "## ", "# "];

        for (i, start_token) in start_tokens.iter().enumerate() {
            let mut matcher = Matcher::new(input);
            if let Some(heading) = matcher.get_match(start_token, "\n\n", true) {
                return Some(Self::new(
                    heading.end_token.is_empty(),
                    heading.body,
                    (start_tokens.len() - i).try_into().unwrap_or(1),
                ));
            }
        }

        None
    }
}

impl Display for Heading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = String::from('#').repeat(self.level as usize);
        let end = if self.consumed_all_input { "" } else { "\n\n" };
        write!(f, "{} {}{}", level, self.text, end)
    }
}

impl Node for Heading {
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
        assert_eq!(Heading::new(true, "Header", 1).to_string(), "# Header");
        assert_eq!(Heading::new(false, "Header", 1).to_string(), "# Header\n\n");
    }

    #[test]
    fn level_gt_six() {
        let h = Heading::new(true, "Header", 7).to_string();
        assert_eq!(h, "###### Header");
        let h = Heading::new(true, "Header", 34).to_string();
        assert_eq!(h, "###### Header");
    }

    #[test]
    fn level_eq_zero() {
        let h = Heading::new(true, "Header", 0).to_string();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_eq_four() {
        let h = Heading::new(true, "Header", 4).to_string();
        assert_eq!(h, "#### Header");
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Heading::deserialize("## Header"),
            Some(Heading::new(true, "Header", 2))
        );
        assert_eq!(
            Heading::deserialize("### Head"),
            Some(Heading::new(true, "Head", 3))
        );
        assert_eq!(
            Heading::deserialize("### Head\n\nsome other thing"),
            Some(Heading::new(false, "Head", 3))
        );
        assert_eq!(Heading::deserialize("not a header"), None);
        assert_eq!(Heading::deserialize("######"), None);
        assert_eq!(Heading::deserialize("######also not a header"), None);
    }

    #[test]
    fn len() {
        assert_eq!(Heading::new(true, "h", 1).len(), 3);
        assert_eq!(Heading::new(true, "h", 2).len(), 4);
        assert_eq!(Heading::new(false, "h", 2).len(), 6);
    }
}
