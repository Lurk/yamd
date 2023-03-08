use crate::sd::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    node::Node,
    tokenizer::{
        Pattern::{Once, RepeatTimes, ZerroOrMore},
        Tokenizer,
    },
};

use super::{
    list::{List, ListTypes},
    paragraph::Paragraph,
};

#[derive(Debug, PartialEq)]
pub enum UnorderedListItemNodes {
    Paragraph(Paragraph),
    List(List),
}

impl From<Paragraph> for UnorderedListItemNodes {
    fn from(value: Paragraph) -> Self {
        UnorderedListItemNodes::Paragraph(value)
    }
}

impl From<List> for UnorderedListItemNodes {
    fn from(value: List) -> Self {
        UnorderedListItemNodes::List(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct ListItem {
    list_type: ListTypes,
    level: usize,
    nodes: Vec<UnorderedListItemNodes>,
}

impl ListItem {
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
        match ctx {
            Some(value) => match value.get_usize_value("level") {
                Some(parrent_level) => parrent_level + 1,
                None => 0,
            },
            None => 0,
        }
    }
}

impl Node for UnorderedListItemNodes {
    fn len(&self) -> usize {
        match self {
            UnorderedListItemNodes::Paragraph(node) => node.len(),
            UnorderedListItemNodes::List(node) => node.len(),
        }
    }
    fn serialize(&self) -> String {
        match self {
            UnorderedListItemNodes::Paragraph(node) => node.serialize(),
            UnorderedListItemNodes::List(node) => node.serialize(),
        }
    }
}

impl Branch<UnorderedListItemNodes> for ListItem {
    fn new_with_context(ctx: &Option<Context>) -> Self {
        Self {
            list_type: Self::get_list_type_from_context(ctx),
            level: Self::get_level_from_context(ctx),
            nodes: vec![],
        }
    }

    fn push<CanBeNode: Into<UnorderedListItemNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn from_vec_with_context(nodes: Vec<UnorderedListItemNodes>, ctx: Option<Context>) -> Self {
        Self {
            list_type: Self::get_list_type_from_context(&ctx),
            level: Self::get_level_from_context(&ctx),
            nodes,
        }
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<UnorderedListItemNodes>> {
        vec![List::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<UnorderedListItemNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        let add = if self.level > 0 { 1 } else { 0 };
        2 + self.level + add
    }
}

impl Node for ListItem {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }

    fn context(&self) -> Option<Context> {
        let mut ctx = Context::new();
        ctx.add("level", self.level);
        Some(ctx)
    }
    fn serialize(&self) -> String {
        let list_type = match self.list_type {
            ListTypes::Unordered => '-',
            ListTypes::Ordered => '+',
        };
        format!(
            "{}{} {}",
            String::from(' ').repeat(self.level),
            list_type,
            self.nodes
                .iter()
                .map(|element| { element.serialize() })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Deserializer for ListItem {
    fn deserialize_with_context(input: &str, ctx: Option<Context>) -> Option<Self> {
        let level = Self::get_level_from_context(&ctx);
        let mut tokenizer = Tokenizer::new(input);
        if let Some(body) = tokenizer.get_token_body_with_options(
            vec![
                ZerroOrMore('\n'),
                RepeatTimes(level, ' '),
                Once('-'),
                Once(' '),
            ],
            vec![Once('\n'), RepeatTimes(level, ' '), Once('-'), Once(' ')],
            true,
        ) {
            return Self::parse_branch(body, &ctx);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{
            list::{List, ListTypes},
            paragraph::Paragraph,
            text::Text,
        },
        sd::{
            deserializer::{Branch, Deserializer},
            node::Node,
        },
    };

    use super::ListItem;

    #[test]
    fn deserialize() {
        assert_eq!(
            ListItem::deserialize("- foo"),
            Some(ListItem::from_vec(vec![Paragraph::from_vec(vec![
                Text::new("foo").into()
            ])
            .into()]))
        );
        assert_eq!(
            ListItem::deserialize_with_context(
                "    - foo",
                Some(List::create_context(Some(3), &ListTypes::Unordered))
            ),
            Some(ListItem::from_vec_with_context(
                vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()],
                Some(List::create_context(Some(3), &ListTypes::Unordered))
            ))
        );
        assert_eq!(ListItem::deserialize("  s  - foo"), None);
    }

    #[test]
    fn deserialize_with_body() {
        assert_eq!(
            ListItem::deserialize("- foo\nbla\n- bar"),
            Some(ListItem::from_vec(vec![Paragraph::from_vec(vec![
                Text::new("foo\nbla").into()
            ])
            .into()]))
        )
    }

    #[test]
    fn deserialize_with_nested_list() {
        assert_eq!(
            ListItem::deserialize("- foo\n - bar\n - baz"),
            Some(ListItem::from_vec(vec![
                Paragraph::from_vec(vec![Text::new("foo").into()]).into(),
                List::from_vec(vec![
                    ListItem::from_vec_with_context(
                        vec![Paragraph::from_vec(vec![Text::new("bar").into()]).into()],
                        Some(List::create_context(Some(0), &ListTypes::Unordered))
                    )
                    .into(),
                    ListItem::from_vec_with_context(
                        vec![Paragraph::from_vec(vec![Text::new("baz").into()]).into()],
                        Some(List::create_context(Some(0), &ListTypes::Unordered))
                    )
                    .into()
                ])
                .into()
            ]))
        );
    }

    #[test]
    fn deserialize_with_deeply_nested_list() {
        assert_eq!(
            ListItem::deserialize("- level 0\n - level 1\n  - level 2\n - level 1"),
            Some(ListItem::from_vec(vec![
                Paragraph::from_vec(vec![Text::new("level 0").into()]).into(),
                List::from_vec(vec![
                    ListItem::from_vec_with_context(
                        vec![
                            Paragraph::from_vec(vec![Text::new("level 1").into()]).into(),
                            List::from_vec_with_context(
                                vec![ListItem::from_vec_with_context(
                                    vec![Paragraph::from_vec(vec![Text::new("level 2").into()])
                                        .into()],
                                    Some(List::create_context(Some(1), &ListTypes::Unordered))
                                )
                                .into()],
                                Some(List::create_context(Some(1), &ListTypes::Unordered))
                            )
                            .into()
                        ],
                        Some(List::create_context(Some(0), &ListTypes::Unordered))
                    )
                    .into(),
                    ListItem::from_vec_with_context(
                        vec![Paragraph::from_vec(vec![Text::new("level 1").into()]).into()],
                        Some(List::create_context(Some(0), &ListTypes::Unordered))
                    )
                    .into()
                ])
                .into()
            ]))
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            ListItem::from_vec(vec![
                Paragraph::from_vec(vec![Text::new("foo").into()]).into()
            ])
            .serialize(),
            String::from("- foo")
        );

        assert_eq!(
            ListItem::from_vec_with_context(
                vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()],
                Some(List::create_context(Some(5), &ListTypes::Unordered))
            )
            .serialize(),
            String::from("      - foo")
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            ListItem::from_vec(vec![
                Paragraph::from_vec(vec![Text::new("foo").into()]).into()
            ])
            .len(),
            5
        );

        assert_eq!(
            ListItem::from_vec_with_context(
                vec![Paragraph::from_vec(vec![Text::new("foo").into()]).into()],
                Some(List::create_context(Some(2), &ListTypes::Unordered))
            )
            .len(),
            9
        );
    }
}
