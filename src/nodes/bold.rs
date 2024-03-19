use std::fmt::Display;

use serde::Serialize;

use crate::{
    nodes::{italic::Italic, strikethrough::Strikethrough, text::Text},
    toolkit::{
        context::Context,
        parser::{parse_to_consumer, parse_to_parser, Branch, Consumer, Parse, Parser},
    },
};

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
pub enum BoldNodes {
    Italic(Italic),
    Strikethrough(Strikethrough),
    Text(Text),
}

impl From<Italic> for BoldNodes {
    fn from(i: Italic) -> Self {
        BoldNodes::Italic(i)
    }
}

impl From<Strikethrough> for BoldNodes {
    fn from(s: Strikethrough) -> Self {
        BoldNodes::Strikethrough(s)
    }
}

impl From<Text> for BoldNodes {
    fn from(t: Text) -> Self {
        BoldNodes::Text(t)
    }
}

#[derive(Debug, PartialEq, Serialize, Clone, Default)]
pub struct Bold {
    nodes: Vec<BoldNodes>,
}

impl Branch<BoldNodes> for Bold {
    fn get_parsers(&self) -> Vec<Parser<BoldNodes>> {
        vec![
            parse_to_parser::<BoldNodes, Italic>(),
            parse_to_parser::<BoldNodes, Strikethrough>(),
        ]
    }

    fn push_node(&mut self, node: BoldNodes) {
        self.nodes.push(node);
    }

    fn get_consumer(&self) -> Option<Consumer<BoldNodes>> {
        Some(parse_to_consumer::<BoldNodes, Text>())
    }
}

impl Parse for Bold {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        if input[current_position..].starts_with("**") {
            if let Some(end) = input[current_position + 2..].find("**") {
                let b = Bold::new(vec![]);
                return Some((
                    b.parse_branch(
                        &input[current_position + 2..current_position + end],
                        "",
                        None,
                    )
                    .expect("bold should always succed"),
                    end + 2,
                ));
            }
        }
        None
    }
}

impl Bold {
    pub fn new(nodes: Vec<BoldNodes>) -> Self {
        Self { nodes }
    }
}

impl Display for BoldNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoldNodes::Text(node) => write!(f, "{}", node),
            BoldNodes::Italic(node) => write!(f, "{}", node),
            BoldNodes::Strikethrough(node) => write!(f, "{}", node),
        }
    }
}

impl Display for Bold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "**{}**",
            self.nodes
                .iter()
                .map(|element| { element.to_string() })
                .collect::<Vec<String>>()
                .concat()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{bold::Bold, italic::Italic, strikethrough::Strikethrough, text::Text},
        toolkit::parser::{Branch, Parse},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn only_text() {
        let mut b = Bold::default();
        b.push_node(Text::new("B as bold").into());
        let str = b.to_string();
        assert_eq!(str, "**B as bold**".to_string());
    }

    #[test]
    fn from_vec() {
        let b: String = Bold::new(vec![
            Text::new("B as bold ").into(),
            Italic::new("Italic").into(),
            Strikethrough::new("Strikethrough").into(),
        ])
        .to_string();
        assert_eq!(b, "**B as bold _Italic_~~Strikethrough~~**".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Bold::parse("**b**", 0, None),
            Some((Bold::new(vec![Text::new("b").into()]), 5))
        );

        assert_eq!(
            Bold::parse("**b ~~st~~ _i t_**", 0, None),
            Some((
                Bold::new(vec![
                    Text::new("b ").into(),
                    Strikethrough::new("st").into(),
                    Text::new(" ").into(),
                    Italic::new("i t").into()
                ]),
                18
            ))
        );
    }
}
