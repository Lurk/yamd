use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    matcher::Matcher,
    node::Node,
};

use super::{
    anchor::Anchor, bold::Bold, inline_code::InlineCode, italic::Italic,
    strikethrough::Strikethrough, text::Text,
};

#[derive(Debug, PartialEq)]
pub enum ListItemContentNodes<'text> {
    A(Anchor<'text>),
    B(Bold<'text>),
    I(Italic<'text>),
    S(Strikethrough),
    Text(Text<'text>),
    InlineCode(InlineCode),
}

impl<'text> From<Anchor<'text>> for ListItemContentNodes<'text> {
    fn from(value: Anchor<'text>) -> Self {
        ListItemContentNodes::A(value)
    }
}

impl<'text> From<Bold<'text>> for ListItemContentNodes<'text> {
    fn from(value: Bold<'text>) -> Self {
        ListItemContentNodes::B(value)
    }
}

impl<'text> From<Italic<'text>> for ListItemContentNodes<'text> {
    fn from(value: Italic<'text>) -> Self {
        ListItemContentNodes::I(value)
    }
}

impl From<Strikethrough> for ListItemContentNodes<'_> {
    fn from(value: Strikethrough) -> Self {
        ListItemContentNodes::S(value)
    }
}

impl<'text> From<Text<'text>> for ListItemContentNodes<'text> {
    fn from(value: Text<'text>) -> Self {
        ListItemContentNodes::Text(value)
    }
}

impl From<InlineCode> for ListItemContentNodes<'_> {
    fn from(value: InlineCode) -> Self {
        ListItemContentNodes::InlineCode(value)
    }
}

impl Node<'_> for ListItemContentNodes<'_> {
    fn serialize(&self) -> String {
        match self {
            ListItemContentNodes::A(node) => node.serialize(),
            ListItemContentNodes::B(node) => node.serialize(),
            ListItemContentNodes::I(node) => node.serialize(),
            ListItemContentNodes::S(node) => node.serialize(),
            ListItemContentNodes::Text(node) => node.serialize(),
            ListItemContentNodes::InlineCode(node) => node.serialize(),
        }
    }

    fn len(&self) -> usize {
        match self {
            ListItemContentNodes::A(node) => node.len(),
            ListItemContentNodes::B(node) => node.len(),
            ListItemContentNodes::I(node) => node.len(),
            ListItemContentNodes::S(node) => node.len(),
            ListItemContentNodes::Text(node) => node.len(),
            ListItemContentNodes::InlineCode(node) => node.len(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ListItemContent<'text> {
    consumed_all_input: bool,
    pub nodes: Vec<ListItemContentNodes<'text>>,
}

impl<'text> Node<'text> for ListItemContent<'text> {
    fn serialize(&self) -> String {
        format!(
            "{}{}",
            self.nodes
                .iter()
                .map(|node| node.serialize())
                .collect::<Vec<String>>()
                .join(""),
            if self.consumed_all_input { "" } else { "\n" }
        )
    }

    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

impl<'text> ListItemContent<'text> {
    pub fn new(consumed_all_input: bool) -> Self {
        Self::new_with_nodes(consumed_all_input, vec![])
    }
    pub fn new_with_nodes(
        consumed_all_input: bool,
        nodes: Vec<ListItemContentNodes<'text>>,
    ) -> Self {
        ListItemContent {
            consumed_all_input,
            nodes,
        }
    }
}

impl<'text> Branch<'text, ListItemContentNodes<'text>> for ListItemContent<'text> {
    fn push<CanBeNode: Into<ListItemContentNodes<'text>>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<'text, ListItemContentNodes<'text>>> {
        vec![
            Anchor::maybe_node(),
            Bold::maybe_node(),
            Italic::maybe_node(),
            Strikethrough::maybe_node(),
            InlineCode::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<'text, ListItemContentNodes<'text>>> {
        Some(Text::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        if self.consumed_all_input {
            0
        } else {
            1
        }
    }
}

impl<'text> Deserializer<'text> for ListItemContent<'text> {
    fn deserialize_with_context(input: &'text str, _: Option<Context>) -> Option<Self> {
        let mut m = Matcher::new(input);
        if let Some(list_item_content) = m.get_match("", "\n", true) {
            return Self::parse_branch(
                list_item_content.body,
                Self::new(list_item_content.end_token.is_empty()),
            );
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::ListItemContent;
    use crate::{
        nodes::{
            anchor::Anchor, bold::Bold, inline_code::InlineCode, italic::Italic,
            strikethrough::Strikethrough, text::Text,
        },
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_consume_all() {
        let input = "This is a list item content";
        let expected = ListItemContent::new_with_nodes(true, vec![Text::new(input).into()]);
        assert_eq!(ListItemContent::deserialize(input), Some(expected));
    }

    #[test]
    fn test_consume_all_with_newline() {
        let input = "This is a list item content\nAnd this is not";
        let expected = ListItemContent::new_with_nodes(
            false,
            vec![Text::new("This is a list item content").into()],
        );
        assert_eq!(ListItemContent::deserialize(input), Some(expected));
    }

    #[test]
    fn len() {
        assert_eq!(ListItemContent::new(true).len(), 0);
        assert_eq!(ListItemContent::new(false).len(), 1);
        assert_eq!(
            ListItemContent::new_with_nodes(true, vec![Text::new("Hello").into()]).len(),
            5
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(ListItemContent::new(true).serialize(), "");
        assert_eq!(ListItemContent::new(false).serialize(), "\n");
        assert_eq!(
            ListItemContent::new_with_nodes(true, vec![Text::new("Hello").into()]).serialize(),
            "Hello"
        );
        assert_eq!(
            ListItemContent::new_with_nodes(false, vec![Text::new("Hello").into()]).serialize(),
            "Hello\n"
        );
    }

    #[test]
    fn deserialize_with_all_nodes() {
        assert_eq!(
            ListItemContent::deserialize(
                "simple text **bold text** `let foo='bar';` [a](u) _I_ ~~S~~"
            ),
            Some(ListItemContent::new_with_nodes(
                true,
                vec![
                    Text::new("simple text ").into(),
                    Bold::new_with_nodes(vec![Text::new("bold text").into()]).into(),
                    Text::new(" ").into(),
                    InlineCode::new("let foo='bar';").into(),
                    Text::new(" ").into(),
                    Anchor::new("a", "u").into(),
                    Text::new(" ").into(),
                    Italic::new("I").into(),
                    Text::new(" ").into(),
                    Strikethrough::new("S").into(),
                ]
            ))
        );
    }
    #[test]
    fn serialize_with_all_nodes() {
        assert_eq!(
            "simple text **bold text** `let foo='bar';` [a](u) _I_ ~~S~~",
            ListItemContent::new_with_nodes(
                true,
                vec![
                    Text::new("simple text ").into(),
                    Bold::new_with_nodes(vec![Text::new("bold text").into()]).into(),
                    Text::new(" ").into(),
                    InlineCode::new("let foo='bar';").into(),
                    Text::new(" ").into(),
                    Anchor::new("a", "u").into(),
                    Text::new(" ").into(),
                    Italic::new("I").into(),
                    Text::new(" ").into(),
                    Strikethrough::new("S").into(),
                ]
            )
            .serialize()
        );
    }
}
