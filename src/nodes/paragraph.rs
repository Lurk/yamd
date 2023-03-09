use crate::nodes::{
    anchor::Anchor, bold::Bold, inline_code::InlineCode, italic::Italic,
    strikethrough::Strikethrough, text::Text,
};
use crate::sd::node::Node;
use crate::sd::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
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
    fn new_with_context(_: &Option<Context>) -> Self {
        Self { nodes: vec![] }
    }

    fn from_vec_with_context(data: Vec<ParagraphNodes>, _: Option<Context>) -> Self {
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
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        let body = tokenizer
            .get_token_body_with_end_of_input(vec![], vec![Once('\n'), Once('\n')], true)
            .unwrap_or(input);
        Self::parse_branch(body, &None)
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new_with_context(&None)
    }
}

impl Node for Paragraph {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum()
    }
    fn serialize(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .concat()
    }
}

impl FallbackNode for Paragraph {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<BranchNodes>
    where
        Self: Into<BranchNodes>,
    {
        Box::new(|input| {
            Paragraph::deserialize(input)
                .unwrap_or(Paragraph::new_with_context(&None))
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
        sd::{
            deserializer::{Branch, Deserializer},
            node::Node,
        },
    };

    use super::Paragraph;

    #[test]
    fn push() {
        let mut p = Paragraph::new();
        p.push(Text::new("simple text "));
        p.push(Bold::from_vec(vec![Text::new("bold text").into()]));
        p.push(InlineCode::new("let foo='bar';"));

        assert_eq!(
            p.serialize(),
            "simple text **bold text**`let foo='bar';`".to_string()
        );
    }

    #[test]
    fn from_vec() {
        let p: String = Paragraph::from_vec(vec![
            Text::new("simple text ").into(),
            Bold::from_vec_with_context(vec![Text::new("bold text").into()], None).into(),
            InlineCode::new("let foo='bar';").into(),
        ])
        .serialize();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Paragraph::deserialize("simple text **bold text**`let foo='bar';`"),
            Some(Paragraph::from_vec(vec![
                Text::new("simple text ").into(),
                Bold::from_vec(vec![Text::new("bold text").into()]).into(),
                InlineCode::new("let foo='bar';").into(),
            ]))
        );
        assert_eq!(
            Paragraph::deserialize("1 2\n\n3"),
            Some(Paragraph::from_vec(vec![Text::new("1 2").into()]))
        );
    }
}
