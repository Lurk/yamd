use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    matcher::Matcher,
    node::Node,
};

use super::{anchor::Anchor, text::Text};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum HeadingNodes {
    Text(Text),
    Anchor(Anchor),
}

impl From<Text> for HeadingNodes {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

impl From<Anchor> for HeadingNodes {
    fn from(anchor: Anchor) -> Self {
        Self::Anchor(anchor)
    }
}

impl Node for HeadingNodes {
    fn len(&self) -> usize {
        match self {
            Self::Text(text) => text.len(),
            Self::Anchor(anchor) => anchor.len(),
        }
    }
}

impl Display for HeadingNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{}", text),
            Self::Anchor(anchor) => write!(f, "{}", anchor),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Heading {
    pub level: u8,
    pub nodes: Vec<HeadingNodes>,
}

impl Heading {
    pub fn new(level: u8, nodes: Vec<HeadingNodes>) -> Self {
        let normalized_level = match level {
            0 => 1,
            7.. => 6,
            l => l,
        };
        Heading {
            nodes,
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
                return Self::parse_branch(
                    heading.body,
                    "",
                    Self::new((start_tokens.len() - i).try_into().unwrap_or(1), vec![]),
                );
            }
        }

        None
    }
}

impl Display for Heading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = String::from('#').repeat(self.level as usize);
        write!(
            f,
            "{} {}",
            level,
            self.nodes.iter().map(|n| n.to_string()).collect::<String>()
        )
    }
}

impl Node for Heading {
    fn len(&self) -> usize {
        self.nodes.iter().map(|n| n.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

impl Branch<HeadingNodes> for Heading {
    fn push<I: Into<HeadingNodes>>(&mut self, node: I) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<HeadingNodes>> {
        vec![Anchor::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<HeadingNodes>> {
        Some(Text::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        self.level as usize + 1
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Heading;
    use crate::{
        nodes::{anchor::Anchor, text::Text},
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn level_one() {
        assert_eq!(
            Heading::new(1, vec![Text::new("Header").into()]).to_string(),
            "# Header"
        );
    }

    #[test]
    fn level_gt_six() {
        let h = Heading::new(7, vec![Text::new("Header").into()]).to_string();
        assert_eq!(h, "###### Header");
        let h = Heading::new(34, vec![Text::new("Header").into()]).to_string();
        assert_eq!(h, "###### Header");
    }

    #[test]
    fn level_eq_zero() {
        let h = Heading::new(0, vec![Text::new("Header").into()]).to_string();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_eq_four() {
        let h = Heading::new(4, vec![Text::new("Header").into()]).to_string();
        assert_eq!(h, "#### Header");
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Heading::deserialize("## Header"),
            Some(Heading::new(2, vec![Text::new("Header").into()]))
        );
        assert_eq!(
            Heading::deserialize("### Head"),
            Some(Heading::new(3, vec![Text::new("Head").into()]))
        );
        assert_eq!(
            Heading::deserialize("### Head\n\nsome other thing"),
            Some(Heading::new(3, vec![Text::new("Head").into()]))
        );
        assert_eq!(Heading::deserialize("not a header"), None);
        assert_eq!(Heading::deserialize("######"), None);
        assert_eq!(Heading::deserialize("######also not a header"), None);
    }

    #[test]
    fn len() {
        assert_eq!(Heading::new(1, vec![Text::new("h").into()]).len(), 3);
        assert_eq!(Heading::new(2, vec![Text::new("h").into()]).len(), 4);
    }

    #[test]
    fn with_anchor() {
        let str = "## hey [a](b)";
        let h = Heading::deserialize(str);
        assert_eq!(
            h,
            Some(Heading::new(
                2,
                vec![Text::new("hey ").into(), Anchor::new("a", "b").into()]
            ))
        );
        assert_eq!(h.as_ref().unwrap().len(), 13);
        assert_eq!(h.as_ref().unwrap().to_string(), str);
    }
}
