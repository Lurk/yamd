use crate::sd::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
    node::Node,
    tokenizer::{
        Pattern::{Once, RepeatTimes, ZerroOrMore},
        Tokenizer,
    },
};

use super::list_item::ListItem;

#[derive(Debug, PartialEq, Clone)]
pub enum ListTypes {
    Unordered,
    Ordered,
}

#[derive(Debug, PartialEq)]
pub enum ListNodes {
    ListItem(ListItem),
}

impl Node for ListNodes {
    fn len(&self) -> usize {
        match self {
            ListNodes::ListItem(node) => node.len(),
        }
    }
    fn serialize(&self) -> String {
        match self {
            ListNodes::ListItem(node) => node.serialize(),
        }
    }
}

impl From<ListItem> for ListNodes {
    fn from(value: ListItem) -> Self {
        ListNodes::ListItem(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct List {
    list_type: ListTypes,
    level: usize,
    nodes: Vec<ListNodes>,
}

impl List {
    fn get_list_type_from_context(ctx: &Option<Context>) -> ListTypes {
        if let Some(ctx) = ctx {
            if let Some(list_type) = ctx.get_char_value("list_type") {
                if list_type == '+' {
                    return ListTypes::Ordered;
                }
            }
        }
        ListTypes::Unordered
    }

    fn get_level_from_context(ctx: &Option<Context>) -> usize {
        if let Some(actual_ctx) = ctx {
            if let Some(level) = actual_ctx.get_usize_value("level") {
                return level;
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

impl Node for List {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.nodes.len() - 1
    }
    fn context(&self) -> Option<Context> {
        Some(Self::create_context(self.level, &self.list_type))
    }
    fn serialize(&self) -> String {
        self.nodes
            .iter()
            .map(|node| node.serialize())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Deserializer for List {
    fn deserialize_with_context(input: &str, ctx: Option<Context>) -> Option<Self> {
        let level = match ctx {
            Some(_) => Self::get_level_from_context(&ctx) + 1,
            None => 0,
        };
        let tokenizer = Tokenizer::new(input);
        if tokenizer
            .get_body_start_position(vec![
                ZerroOrMore('\n'),
                RepeatTimes(level, ' '),
                Once('-'),
                Once(' '),
            ])
            .is_some()
        {
            return Self::parse_branch(
                input,
                &Some(Self::create_context(level, &ListTypes::Unordered)),
            );
        } else if tokenizer
            .get_body_start_position(vec![
                ZerroOrMore('\n'),
                RepeatTimes(level, ' '),
                Once('+'),
                Once(' '),
            ])
            .is_some()
        {
            return Self::parse_branch(
                input,
                &Some(Self::create_context(level, &ListTypes::Ordered)),
            );
        }
        None
    }
}

impl Branch<ListNodes> for List {
    fn new_with_context(ctx: &Option<Context>) -> Self {
        Self {
            list_type: Self::get_list_type_from_context(ctx),
            nodes: vec![],
            level: Self::get_level_from_context(ctx),
        }
    }

    fn push<CanBeNode: Into<ListNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into())
    }

    fn from_vec_with_context(nodes: Vec<ListNodes>, ctx: Option<Context>) -> Self {
        Self {
            list_type: Self::get_list_type_from_context(&ctx),
            nodes,
            level: Self::get_level_from_context(&ctx),
        }
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
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{list_item::ListItem, paragraph::Paragraph, text::Text},
        sd::{
            deserializer::{Branch, Deserializer},
            node::Node,
        },
    };

    use super::{List, ListTypes};

    #[test]
    fn serialize_unordered() {
        let list = List {
            list_type: ListTypes::Unordered,
            level: 0,
            nodes: vec![
                ListItem::from_vec(vec![Paragraph::from_vec(vec![Text::new(
                    "unordered list item",
                )
                .into()])
                .into()])
                .into(),
                ListItem::from_vec(vec![Paragraph::from_vec(vec![Text::new(
                    "unordered list item",
                )
                .into()])
                .into()])
                .into(),
            ],
        };

        assert_eq!(
            list.serialize(),
            "- unordered list item\n- unordered list item"
        );
    }

    #[test]
    fn serialize_ordered() {
        let list = List::from_vec_with_context(
            vec![
                ListItem::from_vec_with_context(
                    vec![Paragraph::from_vec(vec![Text::new("ordered list item").into()]).into()],
                    Some(List::create_context(0, &ListTypes::Ordered)),
                )
                .into(),
                ListItem::from_vec_with_context(
                    vec![Paragraph::from_vec(vec![Text::new("ordered list item").into()]).into()],
                    Some(List::create_context(0, &ListTypes::Ordered)),
                )
                .into(),
            ],
            Some(List::create_context(0, &ListTypes::Ordered)),
        );

        assert_eq!(list.serialize(), "+ ordered list item\n+ ordered list item");
    }

    #[test]
    fn deserialize_unordered() {
        let list = List::from_vec_with_context(
            vec![
                ListItem::from_vec_with_context(
                    vec![Paragraph::from_vec(vec![Text::new("level 0").into()]).into()],
                    Some(List::create_context(0, &ListTypes::Unordered)),
                )
                .into(),
                ListItem::from_vec_with_context(
                    vec![Paragraph::from_vec(vec![Text::new("level 0").into()]).into()],
                    Some(List::create_context(0, &ListTypes::Unordered)),
                )
                .into(),
            ],
            Some(List::create_context(0, &ListTypes::Unordered)),
        );

        assert_eq!(List::deserialize("- level 0\n- level 0"), Some(list));
    }

    #[test]
    fn deserialize_ordered() {
        let list = List::from_vec_with_context(
            vec![
                ListItem::from_vec_with_context(
                    vec![Paragraph::from_vec(vec![Text::new("level 0").into()]).into()],
                    Some(List::create_context(0, &ListTypes::Ordered)),
                )
                .into(),
                ListItem::from_vec_with_context(
                    vec![Paragraph::from_vec(vec![Text::new("level 0").into()]).into()],
                    Some(List::create_context(0, &ListTypes::Ordered)),
                )
                .into(),
            ],
            Some(List::create_context(0, &ListTypes::Ordered)),
        );

        assert_eq!(List::deserialize("+ level 0\n+ level 0"), Some(list));
    }

    #[test]
    fn deserialize_mixed() {
        let list = List::from_vec_with_context(
            vec![ListItem::from_vec_with_context(
                vec![
                    Paragraph::from_vec(vec![Text::new("level 0").into()]).into(),
                    List::from_vec_with_context(
                        vec![ListItem::from_vec_with_context(
                            vec![Paragraph::from_vec(vec![Text::new("level 0").into()]).into()],
                            Some(List::create_context(1, &ListTypes::Unordered)),
                        )
                        .into()],
                        Some(List::create_context(1, &ListTypes::Unordered)),
                    )
                    .into(),
                ],
                Some(List::create_context(0, &ListTypes::Ordered)),
            )
            .into()],
            Some(List::create_context(0, &ListTypes::Ordered)),
        );

        assert_eq!(List::deserialize("+ level 0\n - level 0"), Some(list));
    }

    #[test]
    fn len() {
        let list = List::from_vec_with_context(
            vec![
                ListItem::from_vec_with_context(
                    vec![Paragraph::from_vec(vec![Text::new("l").into()]).into()],
                    Some(List::create_context(0, &ListTypes::Ordered)),
                )
                .into(),
                ListItem::from_vec_with_context(
                    vec![Paragraph::from_vec(vec![Text::new("l").into()]).into()],
                    Some(List::create_context(0, &ListTypes::Ordered)),
                )
                .into(),
            ],
            Some(List::create_context(0, &ListTypes::Ordered)),
        );

        assert_eq!(list.len(), 7);
    }
}
