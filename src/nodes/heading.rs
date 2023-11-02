use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Heading {
    pub level: u8,
    pub text: String,
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
        let start_tokens = ["###### ", "##### ", "#### ", "### ", "## ", "# "];

        for (i, start_token) in start_tokens.iter().enumerate() {
            let mut matcher = Matcher::new(input);
            if let Some(heading) = matcher.get_match(start_token, "\n\n", true) {
                return Some(Self::new(
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
        write!(f, "{} {}", level, self.text)
    }
}

impl Node for Heading {
    fn len(&self) -> usize {
        self.text.len() + self.level as usize + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Heading;
    use crate::toolkit::{deserializer::Deserializer, node::Node};
    use pretty_assertions::assert_eq;

    #[test]
    fn level_one() {
        assert_eq!(Heading::new("Header", 1).to_string(), "# Header");
    }

    #[test]
    fn level_gt_six() {
        let h = Heading::new("Header", 7).to_string();
        assert_eq!(h, "###### Header");
        let h = Heading::new("Header", 34).to_string();
        assert_eq!(h, "###### Header");
    }

    #[test]
    fn level_eq_zero() {
        let h = Heading::new("Header", 0).to_string();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_eq_four() {
        let h = Heading::new("Header", 4).to_string();
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
