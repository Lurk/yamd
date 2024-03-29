use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    matcher::Matcher,
    node::Node,
};

use super::list_item::ListItem;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ListTypes {
    Unordered,
    Ordered,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ListNodes {
    ListItem(ListItem),
}

impl Display for ListNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListNodes::ListItem(node) => write!(f, "{}", node),
        }
    }
}

impl Node for ListNodes {
    fn len(&self) -> usize {
        match self {
            ListNodes::ListItem(node) => node.len(),
        }
    }
}

impl From<ListItem> for ListNodes {
    fn from(value: ListItem) -> Self {
        ListNodes::ListItem(value)
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct List {
    pub list_type: ListTypes,
    pub level: usize,
    pub nodes: Vec<ListNodes>,
}

impl List {
    pub fn new(list_type: ListTypes, level: usize, nodes: Vec<ListNodes>) -> Self {
        Self {
            list_type,
            level,
            nodes,
        }
    }

    fn get_level_from_context(ctx: &Option<Context>) -> usize {
        if let Some(actual_ctx) = ctx {
            if let Some(level) = actual_ctx.get_usize_value("level") {
                return level + 1;
            }
        }
        0
    }

    pub fn create_context(level: usize, list_type: &ListTypes) -> Context {
        let mut ctx = Context::new();
        ctx.add(
            "list_type",
            match list_type {
                ListTypes::Unordered => '-',
                ListTypes::Ordered => '+',
            },
        );
        ctx.add("level", level);
        ctx
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Node for List {
    fn len(&self) -> usize {
        let add = if self.is_empty() {
            0
        } else {
            self.nodes.len() - 1
        };
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + add
    }
    fn context(&self) -> Option<Context> {
        Some(Self::create_context(self.level, &self.list_type))
    }
}

impl Deserializer for List {
    fn deserialize_with_context(input: &str, ctx: Option<Context>) -> Option<Self> {
        let level = Self::get_level_from_context(&ctx);
        let mut matcher = Matcher::new(input);
        if let Some(unordered_list) =
            matcher.get_match(format!("{}- ", " ".repeat(level)).as_str(), "\n\n", true)
        {
            return Self::parse_branch(
                &input[..unordered_list.start_token.len() + unordered_list.body.len()],
                "\n",
                Self::new(ListTypes::Unordered, level, vec![]),
            );
        } else if let Some(ordered_list) =
            matcher.get_match(format!("{}+ ", " ".repeat(level)).as_str(), "\n\n", true)
        {
            let res = Self::parse_branch(
                &input[..ordered_list.start_token.len() + ordered_list.body.len()],
                "\n",
                Self::new(ListTypes::Ordered, level, vec![]),
            );
            return res;
        }
        None
    }
}

impl Branch<ListNodes> for List {
    fn push<CanBeNode: Into<ListNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into())
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<ListNodes>> {
        vec![ListItem::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<ListNodes>> {
        None
    }

    fn get_outer_token_length(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{List, ListTypes};
    use crate::{
        nodes::{list_item::ListItem, list_item_content::ListItemContent, text::Text},
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_unordered() {
        assert_eq!(
            List {
                list_type: ListTypes::Unordered,
                level: 0,
                nodes: vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new(vec![Text::new("unordered list item").into()],)
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new(vec![Text::new("unordered list item").into()],)
                    )
                    .into(),
                ],
            }
            .to_string(),
            "- unordered list item\n- unordered list item"
        );
        assert_eq!(
            List {
                list_type: ListTypes::Unordered,
                level: 0,
                nodes: vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new(vec![Text::new("unordered list item").into()],)
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new(vec![Text::new("unordered list item").into()],)
                    )
                    .into(),
                ],
            }
            .to_string(),
            "- unordered list item\n- unordered list item"
        );
    }

    #[test]
    fn serialize_ordered() {
        let list = List::new(
            ListTypes::Ordered,
            0,
            vec![
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    ListItemContent::new(vec![Text::new("ordered list item").into()]),
                )
                .into(),
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    ListItemContent::new(vec![Text::new("ordered list item").into()]),
                )
                .into(),
            ],
        );

        assert_eq!(list.to_string(), "+ ordered list item\n+ ordered list item");
    }

    #[test]
    fn deserialize_wrong_level() {
        assert_eq!(
            List::deserialize_with_context(
                "- level 0\n- level 0",
                Some(List::create_context(1, &ListTypes::Unordered))
            ),
            None
        );
    }

    #[test]
    fn deserialize_unordered() {
        assert_eq!(
            List::deserialize("- level 0\n- level 0"),
            Some(List::new(
                ListTypes::Unordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new(vec![Text::new("level 0").into()])
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new(vec![Text::new("level 0").into()])
                    )
                    .into(),
                ],
            ))
        );
        assert_eq!(
            List::deserialize("- level 0\n- level 0\n\n"),
            Some(List::new(
                ListTypes::Unordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new(vec![Text::new("level 0").into()])
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        ListItemContent::new(vec![Text::new("level 0").into()])
                    )
                    .into(),
                ],
            ))
        );
    }

    #[test]
    fn deserialize_ordered() {
        assert_eq!(
            List::deserialize("+ level 0\n+ level 0"),
            Some(List::new(
                ListTypes::Ordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new(vec![Text::new("level 0").into()])
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new(vec![Text::new("level 0").into()])
                    )
                    .into(),
                ],
            ))
        );
        assert_eq!(
            List::deserialize("+ level 0\n+ level 0\n\n"),
            Some(List::new(
                ListTypes::Ordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new(vec![Text::new("level 0").into()])
                    )
                    .into(),
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        ListItemContent::new(vec![Text::new("level 0").into()])
                    )
                    .into(),
                ],
            ))
        );
    }

    #[test]
    fn deserialize_mixed() {
        let list = List::new(
            ListTypes::Ordered,
            0,
            vec![ListItem::new_with_nested_list(
                ListTypes::Ordered,
                0,
                ListItemContent::new(vec![Text::new("level 0").into()]),
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new(vec![Text::new("level 0").into()]),
                    )
                    .into()],
                )),
            )
            .into()],
        );

        assert_eq!(List::deserialize("+ level 0\n - level 0"), Some(list));
    }

    #[test]
    fn deserialized_nested() {
        let list = List::new(
            ListTypes::Unordered,
            0,
            vec![ListItem::new_with_nested_list(
                ListTypes::Unordered,
                0,
                ListItemContent::new(vec![Text::new("one").into()]).into(),
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        ListItemContent::new(vec![Text::new("two").into()]),
                    )
                    .into()],
                )),
            )
            .into()],
        );

        let input = r#"- one
 - two

"#;
        assert_eq!(List::deserialize(input), Some(list));
    }

    #[test]
    fn len() {
        let list = List::new(
            ListTypes::Ordered,
            0,
            vec![
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    ListItemContent::new(vec![Text::new("l").into()]),
                )
                .into(),
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    ListItemContent::new(vec![Text::new("l").into()]),
                )
                .into(),
            ],
        );
        assert_eq!(list.len(), 7);
    }

    #[test]
    fn empty_list() {
        let list = List::new(ListTypes::Ordered, 0, vec![]);
        assert_eq!(list.len(), 0);
    }
}
