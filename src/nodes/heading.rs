use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    parser::{parse_to_consumer, parse_to_parser, Branch, Consumer, Parse, Parser},
};

use super::{anchor::Anchor, text::Text};

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
pub enum HeadingNodes {
    Text(Text),
    A(Anchor),
}

impl From<Text> for HeadingNodes {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

impl From<Anchor> for HeadingNodes {
    fn from(anchor: Anchor) -> Self {
        Self::A(anchor)
    }
}

impl Display for HeadingNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{}", text),
            Self::A(anchor) => write!(f, "{}", anchor),
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

impl Branch<HeadingNodes> for Heading {
    fn push_node(&mut self, node: HeadingNodes) {
        self.nodes.push(node);
    }

    fn get_parsers(&self) -> Vec<Parser<HeadingNodes>> {
        vec![parse_to_parser::<HeadingNodes, Anchor>()]
    }

    fn get_consumer(&self) -> Option<Consumer<HeadingNodes>> {
        Some(parse_to_consumer::<HeadingNodes, Text>())
    }
}

impl Parse for Heading {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        let start_tokens = ["###### ", "##### ", "#### ", "### ", "## ", "# "];

        for (i, start_token) in start_tokens.iter().enumerate() {
            if input[current_position..].starts_with(start_token) {
                let end = input[current_position + start_token.len()..]
                    .find("\n\n")
                    .unwrap_or(input[current_position + start_token.len()..].len());
                let heading =
                    Heading::new((start_tokens.len() - i).try_into().unwrap_or(1), vec![]);

                return Some((
                    heading
                        .parse_branch(
                            &input[current_position + start_token.len()
                                ..current_position + start_token.len() + end],
                            "",
                            None,
                        )
                        .expect("heading should always succeed"),
                    start_token.len() + end,
                ));
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

#[cfg(test)]
mod tests {
    use super::Heading;
    use crate::{
        nodes::{anchor::Anchor, text::Text},
        toolkit::parser::Parse,
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
            Heading::parse("## Header", 0, None),
            Some((Heading::new(2, vec![Text::new("Header").into()]), 9))
        );
        assert_eq!(
            Heading::parse("### Head", 0, None),
            Some((Heading::new(3, vec![Text::new("Head").into()]), 8))
        );
        assert_eq!(Heading::parse("not a header", 0, None), None);
        assert_eq!(Heading::parse("######", 0, None), None);
        assert_eq!(Heading::parse("######also not a header", 0, None), None);
    }

    #[test]
    fn with_anchor() {
        let str = "## hey [a](b)";
        let h = Heading::parse(str, 0, None);
        assert_eq!(
            h,
            Some((
                Heading::new(
                    2,
                    vec![Text::new("hey ").into(), Anchor::new("a", "b").into()]
                ),
                13
            ))
        );
        assert_eq!(h.map(|(node, _)| node.to_string()).unwrap(), str);
    }
}
