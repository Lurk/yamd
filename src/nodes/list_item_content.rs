use std::fmt::Display;

use serde::Serialize;

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

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ListItemContentNodes {
    A(Anchor),
    B(Bold),
    I(Italic),
    S(Strikethrough),
    Text(Text),
    InlineCode(InlineCode),
}

impl From<Anchor> for ListItemContentNodes {
    fn from(value: Anchor) -> Self {
        ListItemContentNodes::A(value)
    }
}

impl From<Bold> for ListItemContentNodes {
    fn from(value: Bold) -> Self {
        ListItemContentNodes::B(value)
    }
}

impl From<Italic> for ListItemContentNodes {
    fn from(value: Italic) -> Self {
        ListItemContentNodes::I(value)
    }
}

impl From<Strikethrough> for ListItemContentNodes {
    fn from(value: Strikethrough) -> Self {
        ListItemContentNodes::S(value)
    }
}

impl From<Text> for ListItemContentNodes {
    fn from(value: Text) -> Self {
        ListItemContentNodes::Text(value)
    }
}

impl From<InlineCode> for ListItemContentNodes {
    fn from(value: InlineCode) -> Self {
        ListItemContentNodes::InlineCode(value)
    }
}

impl Display for ListItemContentNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListItemContentNodes::A(node) => write!(f, "{}", node),
            ListItemContentNodes::B(node) => write!(f, "{}", node),
            ListItemContentNodes::I(node) => write!(f, "{}", node),
            ListItemContentNodes::S(node) => write!(f, "{}", node),
            ListItemContentNodes::Text(node) => write!(f, "{}", node),
            ListItemContentNodes::InlineCode(node) => write!(f, "{}", node),
        }
    }
}

impl Node for ListItemContentNodes {
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

#[derive(Debug, PartialEq, Serialize)]
pub struct ListItemContent {
    #[serde(skip_serializing)]
    consumed_all_input: bool,
    pub nodes: Vec<ListItemContentNodes>,
}

impl Display for ListItemContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join(""),
            if self.consumed_all_input { "" } else { "\n" }
        )
    }
}

impl Node for ListItemContent {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

impl ListItemContent {
    pub fn new(consumed_all_input: bool) -> Self {
        Self::new_with_nodes(consumed_all_input, vec![])
    }
    pub fn new_with_nodes(consumed_all_input: bool, nodes: Vec<ListItemContentNodes>) -> Self {
        ListItemContent {
            consumed_all_input,
            nodes,
        }
    }
}

impl Branch<ListItemContentNodes> for ListItemContent {
    fn push<CanBeNode: Into<ListItemContentNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<ListItemContentNodes>> {
        vec![
            Anchor::maybe_node(),
            Bold::maybe_node(),
            Italic::maybe_node(),
            Strikethrough::maybe_node(),
            InlineCode::maybe_node(),
        ]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<ListItemContentNodes>> {
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

impl Deserializer for ListItemContent {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
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
        assert_eq!(ListItemContent::new(true).to_string(), "");
        assert_eq!(ListItemContent::new(false).to_string(), "\n");
        assert_eq!(
            ListItemContent::new_with_nodes(true, vec![Text::new("Hello").into()]).to_string(),
            "Hello"
        );
        assert_eq!(
            ListItemContent::new_with_nodes(false, vec![Text::new("Hello").into()]).to_string(),
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
            .to_string()
        );
    }
}
