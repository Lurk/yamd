use super::unordered_list_item::UnorderedListItem;
use crate::sd::{
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode, Node},
    serializer::Serializer,
};

#[derive(Debug, PartialEq)]
pub enum ListNodes {
    UnorderedListItem(UnorderedListItem),
}

impl Node for ListNodes {
    fn len(&self) -> usize {
        match self {
            ListNodes::UnorderedListItem(node) => node.len(),
        }
    }
}

impl Serializer for ListNodes {
    fn serialize(&self) -> String {
        match self {
            ListNodes::UnorderedListItem(node) => node.serialize(),
        }
    }
}

impl From<UnorderedListItem> for ListNodes {
    fn from(value: UnorderedListItem) -> Self {
        ListNodes::UnorderedListItem(value)
    }
}

/// nested lists
/// - level 0
///  - level 1
///   - level 2
///    - level 3
/// hkjhkjhkjhkjh
///     + level 4
///     + level 4
///  - level 1
/// - level 0
///
/// the rules:
/// level increase can be done only by one
/// level decrease can be done by any number
#[derive(Debug, PartialEq)]
pub struct List {
    nodes: Vec<ListNodes>,
}

impl Node for List {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum()
    }
}

impl Serializer for List {
    fn serialize(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .concat()
    }
}

impl Branch<ListNodes> for List {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn push<CanBeNode: Into<ListNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn from_vec(nodes: Vec<ListNodes>) -> Self {
        Self { nodes }
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<ListNodes>> {
        vec![UnorderedListItem::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<ListNodes>> {
        None
    }

    fn get_outer_token_length(&self) -> usize {
        0
    }
}

impl Deserializer for List {
    fn deserialize(input: &str) -> Option<Self> {
        Self::parse_branch(input)
    }
}
