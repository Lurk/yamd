use crate::nodes::{
    anchor::Anchor, bold::Bold, inline_code::InlineCode, italic::Italic,
    strikethrough::Strikethrough, text::Text,
};
use crate::toolkit::node::Node;
use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    matcher::Matcher,
};

#[derive(Debug, PartialEq)]
pub enum ParagraphNodes<'text> {
    A(Anchor<'text>),
    B(Bold<'text>),
    I(Italic),
    S(Strikethrough),
    Text(Text<'text>),
    InlineCode(InlineCode),
}

impl<'text> From<Anchor<'text>> for ParagraphNodes<'text> {
    fn from(value: Anchor<'text>) -> Self {
        ParagraphNodes::A(value)
    }
}

impl<'text> From<Bold<'text>> for ParagraphNodes<'text> {
    fn from(value: Bold<'text>) -> Self {
        ParagraphNodes::B(value)
    }
}

impl From<Italic> for ParagraphNodes<'_> {
    fn from(value: Italic) -> Self {
        ParagraphNodes::I(value)
    }
}

impl From<Strikethrough> for ParagraphNodes<'_> {
    fn from(value: Strikethrough) -> Self {
        ParagraphNodes::S(value)
    }
}

impl<'text> From<Text<'text>> for ParagraphNodes<'text> {
    fn from(value: Text<'text>) -> Self {
        ParagraphNodes::Text(value)
    }
}

impl From<InlineCode> for ParagraphNodes<'_> {
    fn from(value: InlineCode) -> Self {
        ParagraphNodes::InlineCode(value)
    }
}

impl Node<'_> for ParagraphNodes<'_> {
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

#[derive(Debug, PartialEq)]
pub struct Paragraph<'text> {
    consumed_all_input: bool,
    pub nodes: Vec<ParagraphNodes<'text>>,
}

impl<'text> Paragraph<'text> {
    pub fn new(consumed_all_input: bool) -> Self {
        Self::new_with_nodes(consumed_all_input, vec![])
    }
    pub fn new_with_nodes(consumed_all_input: bool, nodes: Vec<ParagraphNodes<'text>>) -> Self {
        Self {
            consumed_all_input,
            nodes,
        }
    }
}

impl<'text> Branch<'text, ParagraphNodes<'text>> for Paragraph<'text> {
    fn push<TP: Into<ParagraphNodes<'text>>>(&mut self, element: TP) {
        self.nodes.push(element.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<'text, ParagraphNodes<'text>>> {
        vec![
            Anchor::maybe_node(),
            Bold::maybe_node(),
            Italic::maybe_node(),
            Strikethrough::maybe_node(),
            InlineCode::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<'text, ParagraphNodes<'text>>> {
        Some(Text::fallback_node())
    }
    fn get_outer_token_length(&self) -> usize {
        if self.consumed_all_input {
            0
        } else {
            2
        }
    }
}

impl<'text> Deserializer<'text> for Paragraph<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(paragraph) = matcher.get_match("", "\n\n", true) {
            return Self::parse_branch(paragraph.body, Self::new(paragraph.end_token.is_empty()));
        }
        None
    }
}

impl<'text> Node<'text> for Paragraph<'text> {
    fn serialize(&self) -> String {
        let end_token = match self.consumed_all_input {
            true => "",
            false => "\n\n",
        };
        format!(
            "{}{}",
            self.nodes
                .iter()
                .map(|node| node.serialize())
                .collect::<Vec<String>>()
                .concat(),
            end_token
        )
    }

    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

impl<'text> FallbackNode<'text> for Paragraph<'text> {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<'text, BranchNodes>
    where
        Self: Into<BranchNodes>,
    {
        Box::new(|input| {
            Paragraph::deserialize(input)
                .unwrap_or(Paragraph::new(true))
                .into()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Paragraph;
    use crate::{
        nodes::bold::Bold,
        nodes::inline_code::InlineCode,
        nodes::{anchor::Anchor, italic::Italic, strikethrough::Strikethrough, text::Text},
        toolkit::{
            deserializer::{Branch, Deserializer},
            node::Node,
        },
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn push() {
        let mut p = Paragraph::new(true);
        p.push(Text::new("simple text "));
        p.push(Bold::new_with_nodes(vec![Text::new("bold text").into()]));
        p.push(InlineCode::new("let foo='bar';"));

        assert_eq!(
            p.serialize(),
            "simple text **bold text**`let foo='bar';`".to_string()
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Paragraph::new_with_nodes(
                true,
                vec![
                    Text::new("simple text ").into(),
                    Bold::new_with_nodes(vec![Text::new("bold text").into()]).into(),
                    InlineCode::new("let foo='bar';").into(),
                    Anchor::new("a", "u").into(),
                    Italic::new("I").into(),
                    Strikethrough::new("S").into()
                ],
            )
            .serialize(),
            "simple text **bold text**`let foo='bar';`[a](u)_I_~~S~~".to_string()
        );
        assert_eq!(
            Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).serialize(),
            "t\n\n".to_string()
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Paragraph::deserialize("simple text **bold text**`let foo='bar';`[t](u)"),
            Some(Paragraph::new_with_nodes(
                true,
                vec![
                    Text::new("simple text ").into(),
                    Bold::new_with_nodes(vec![Text::new("bold text").into()]).into(),
                    InlineCode::new("let foo='bar';").into(),
                    Anchor::new("t", "u").into()
                ]
            ))
        );
        assert_eq!(
            Paragraph::deserialize("1 2\n\n3"),
            Some(Paragraph::new_with_nodes(
                false,
                vec![Text::new("1 2").into()]
            ))
        );
    }
    #[test]
    fn len() {
        assert_eq!(Paragraph::new(true).len(), 0);
        assert_eq!(Paragraph::new(false).len(), 2);
    }
}
