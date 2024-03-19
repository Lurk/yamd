use std::fmt::Display;

use serde::Serialize;

use crate::{
    nodes::{
        anchor::Anchor, bold::Bold, inline_code::InlineCode, italic::Italic,
        strikethrough::Strikethrough, text::Text,
    },
    toolkit::{
        context::Context,
        parser::{parse_to_consumer, parse_to_parser, Branch, Consumer, Parse, Parser},
    },
};

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ParagraphNodes {
    A(Anchor),
    B(Bold),
    I(Italic),
    S(Strikethrough),
    Text(Text),
    InlineCode(InlineCode),
}

impl From<Anchor> for ParagraphNodes {
    fn from(value: Anchor) -> Self {
        ParagraphNodes::A(value)
    }
}

impl From<Bold> for ParagraphNodes {
    fn from(value: Bold) -> Self {
        ParagraphNodes::B(value)
    }
}

impl From<Italic> for ParagraphNodes {
    fn from(value: Italic) -> Self {
        ParagraphNodes::I(value)
    }
}

impl From<Strikethrough> for ParagraphNodes {
    fn from(value: Strikethrough) -> Self {
        ParagraphNodes::S(value)
    }
}

impl From<Text> for ParagraphNodes {
    fn from(value: Text) -> Self {
        ParagraphNodes::Text(value)
    }
}

impl From<InlineCode> for ParagraphNodes {
    fn from(value: InlineCode) -> Self {
        ParagraphNodes::InlineCode(value)
    }
}

impl Display for ParagraphNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphNodes::A(node) => write!(f, "{}", node),
            ParagraphNodes::B(node) => write!(f, "{}", node),
            ParagraphNodes::I(node) => write!(f, "{}", node),
            ParagraphNodes::S(node) => write!(f, "{}", node),
            ParagraphNodes::Text(node) => write!(f, "{}", node),
            ParagraphNodes::InlineCode(node) => write!(f, "{}", node),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Paragraph {
    pub nodes: Vec<ParagraphNodes>,
}

impl Paragraph {
    pub fn new(nodes: Vec<ParagraphNodes>) -> Self {
        Self { nodes }
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Display for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .concat(),
        )
    }
}

impl Branch<ParagraphNodes> for Paragraph {
    fn get_parsers(&self) -> Vec<Parser<ParagraphNodes>> {
        vec![
            parse_to_parser::<ParagraphNodes, Anchor>(),
            parse_to_parser::<ParagraphNodes, Bold>(),
            parse_to_parser::<ParagraphNodes, Italic>(),
            parse_to_parser::<ParagraphNodes, Strikethrough>(),
            parse_to_parser::<ParagraphNodes, InlineCode>(),
        ]
    }

    fn get_consumer(&self) -> Option<Consumer<ParagraphNodes>> {
        Some(parse_to_consumer::<ParagraphNodes, Text>())
    }

    fn push_node(&mut self, node: ParagraphNodes) {
        self.nodes.push(node);
    }
}

impl Parse for Paragraph {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        let paragraph = Paragraph::default();
        Some((
            paragraph
                .parse_branch(&input[current_position..], "", None)
                .expect("paragraph should always succed"),
            input.len() - current_position,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Paragraph;
    use crate::{
        nodes::{
            anchor::Anchor, bold::Bold, inline_code::InlineCode, italic::Italic,
            strikethrough::Strikethrough, text::Text,
        },
        toolkit::parser::{Branch, Parse},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn push() {
        let mut p = Paragraph::default();
        p.push_node(Text::new("simple text ").into());
        p.push_node(Bold::new(vec![Text::new("bold text").into()]).into());
        p.push_node(InlineCode::new("let foo='bar';").into());

        assert_eq!(
            p.to_string(),
            "simple text **bold text**`let foo='bar';`".to_string()
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Paragraph::new(vec![
                Text::new("simple text ").into(),
                Bold::new(vec![Text::new("bold text").into()]).into(),
                InlineCode::new("let foo='bar';").into(),
                Anchor::new("a", "u").into(),
                Italic::new("I").into(),
                Strikethrough::new("S").into()
            ],)
            .to_string(),
            "simple text **bold text**`let foo='bar';`[a](u)_I_~~S~~".to_string()
        );
    }

    #[test]
    fn parse() {
        assert_eq!(
            Paragraph::parse("simple text **bold text**`let foo='bar';`[t](u)", 0, None),
            Some((
                Paragraph::new(vec![
                    Text::new("simple text ").into(),
                    Bold::new(vec![Text::new("bold text").into()]).into(),
                    InlineCode::new("let foo='bar';").into(),
                    Anchor::new("t", "u").into()
                ]),
                46
            ))
        );
    }
}
