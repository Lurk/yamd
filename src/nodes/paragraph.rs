use crate::nodes::{
    anchor::Anchor, bold::Bold, inline_code::InlineCode, italic::Italic,
    strikethrough::Strikethrough, text::Text, yamd::YamdNodes,
};
use crate::sd::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode, Node},
    serializer::Serializer,
    tokenizer::{Pattern::Once, Tokenizer},
};

#[derive(Debug, PartialEq)]
pub enum ParagraphNodes {
    A(Anchor),
    B(Bold),
    I(Italic),
    S(Strikethrough),
    Text(Text),
    InlineCode(InlineCode),
}

impl Node for ParagraphNodes {
    fn len(&self) -> usize {
        match self {
            ParagraphNodes::A(node) => node.len(),
            ParagraphNodes::B(node) => node.len(),
            ParagraphNodes::I(node) => node.len(),
            ParagraphNodes::S(node) => node.len(),
            ParagraphNodes::Text(node) => node.len(),
            ParagraphNodes::InlineCode(node) => node.len(),
        }
    }
}

impl Serializer for ParagraphNodes {
    fn serialize(&self) -> String {
        match self {
            ParagraphNodes::A(node) => node.serialize(),
            ParagraphNodes::B(node) => node.serialize(),
            ParagraphNodes::I(node) => node.serialize(),
            ParagraphNodes::S(node) => node.serialize(),
            ParagraphNodes::Text(node) => node.serialize(),
            ParagraphNodes::InlineCode(node) => node.serialize(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Paragraph {
    nodes: Vec<ParagraphNodes>,
}

impl Branch<ParagraphNodes> for Paragraph {
    fn new(_: &Option<Context>) -> Self {
        Self { nodes: vec![] }
    }

    fn from_vec(data: Vec<ParagraphNodes>, _: Option<Context>) -> Self {
        Self { nodes: data }
    }

    fn push<TP: Into<ParagraphNodes>>(&mut self, element: TP) {
        self.nodes.push(element.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<ParagraphNodes>> {
        vec![
            Anchor::maybe_node(),
            Bold::maybe_node(),
            Italic::maybe_node(),
            Strikethrough::maybe_node(),
            InlineCode::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<ParagraphNodes>> {
        Some(Text::fallback_node())
    }
    fn get_outer_token_length(&self) -> usize {
        0
    }
}

impl Deserializer for Paragraph {
    fn deserialize(input: &str, _: Option<Context>) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        let body = tokenizer
            .get_token_body_with_options(vec![], vec![Once('\n'), Once('\n')], true)
            .unwrap_or(input);
        Self::parse_branch(body, &None)
    }
}

impl Serializer for Paragraph {
    fn serialize(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .concat()
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new(&None)
    }
}

impl From<Paragraph> for YamdNodes {
    fn from(value: Paragraph) -> Self {
        YamdNodes::P(value)
    }
}

impl Node for Paragraph {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum()
    }
}

impl FallbackNode for Paragraph {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<BranchNodes>
    where
        Self: Into<BranchNodes>,
    {
        Box::new(|input| {
            Paragraph::deserialize_without_context(input)
                .unwrap_or(Paragraph::new(&None))
                .into()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::bold::Bold,
        nodes::inline_code::InlineCode,
        nodes::text::Text,
        sd::deserializer::{Branch, Deserializer},
        sd::serializer::Serializer,
    };

    use super::Paragraph;

    #[test]
    fn push() {
        let mut p = Paragraph::new(&None);
        p.push(Text::new("simple text "));
        p.push(Bold::from_vec(vec![Text::new("bold text").into()], None));
        p.push(InlineCode::new("let foo='bar';"));

        assert_eq!(
            p.serialize(),
            "simple text **bold text**`let foo='bar';`".to_string()
        );
    }

    #[test]
    fn from_vec() {
        let p: String = Paragraph::from_vec(
            vec![
                Text::new("simple text ").into(),
                Bold::from_vec(vec![Text::new("bold text").into()], None).into(),
                InlineCode::new("let foo='bar';").into(),
            ],
            None,
        )
        .serialize();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Paragraph::deserialize_without_context("simple text **bold text**`let foo='bar';`"),
            Some(Paragraph::from_vec(
                vec![
                    Text::new("simple text ").into(),
                    Bold::from_vec(vec![Text::new("bold text").into()], None).into(),
                    InlineCode::new("let foo='bar';").into(),
                ],
                None
            ))
        );
        assert_eq!(
            Paragraph::deserialize_without_context("1 2\n\n3"),
            Some(Paragraph::from_vec(vec![Text::new("1 2").into()], None))
        );
    }
}
