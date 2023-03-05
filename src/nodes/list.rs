use crate::sd::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode, Node},
    serializer::Serializer,
    tokenizer::{
        Pattern::{Once, ZerroOrMore},
        Tokenizer,
    },
};

use super::unordered_list_item::ListItem;

#[derive(Debug, PartialEq)]
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
}

impl Serializer for ListNodes {
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
    t: ListTypes,
    level: usize,
    nodes: Vec<ListNodes>,
}

impl List {
    fn get_t_from_context(ctx: &Option<Context>) -> ListTypes {
        if let Some(ctx) = ctx {
            if let Some(t) = ctx.get_char_value("t") {
                if t == '+' {
                    return ListTypes::Ordered;
                }
            }
        }
        ListTypes::Unordered
    }

    fn get_level_from_context(ctx: &Option<Context>) -> usize {
        if let Some(ctx) = ctx {
            if let Some(level) = ctx.get_usize_value("level") {
                return level;
            }
        }
        0
    }
}

impl Node for List {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum()
    }
    fn context(&self) -> Option<Context> {
        let mut ctx = Context::new();
        ctx.add(
            "t",
            match self.t {
                ListTypes::Unordered => '-',
                ListTypes::Ordered => '+',
            },
        );
        ctx.add("level", self.level);
        Some(ctx)
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

impl Deserializer for List {
    fn deserialize(input: &str, ctx: Option<Context>) -> Option<Self> {
        if ctx.is_none() {
            let mut ctx = Context::new();
            let tokenizer = Tokenizer::new(input);
            if tokenizer
                .get_body_start_position(vec![ZerroOrMore('\n'), ZerroOrMore(' '), Once('-')])
                .is_some()
            {
                ctx.add("t", '-');
                return Self::parse_branch(input, &Some(ctx));
            } else if tokenizer
                .get_body_start_position(vec![ZerroOrMore('\n'), ZerroOrMore(' '), Once('+')])
                .is_some()
            {
                ctx.add("t", '+');
                return Self::parse_branch(input, &Some(ctx));
            }
        } else {
            return Self::parse_branch(input, &ctx);
        }
        None
    }
}

impl Branch<ListNodes> for List {
    fn new(ctx: &Option<Context>) -> Self {
        Self {
            t: Self::get_t_from_context(ctx),
            nodes: vec![],
            level: Self::get_level_from_context(ctx),
        }
    }

    fn push<CanBeNode: Into<ListNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into())
    }

    fn from_vec(nodes: Vec<ListNodes>, ctx: Option<Context>) -> Self {
        Self {
            t: Self::get_t_from_context(&ctx),
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
        nodes::{paragraph::Paragraph, text::Text, unordered_list_item::ListItem},
        sd::{context::Context, deserializer::Branch, serializer::Serializer},
    };

    use super::{List, ListTypes};

    #[test]
    fn serialize_unordered() {
        let list = List {
            t: ListTypes::Unordered,
            level: 0,
            nodes: vec![ListItem::from_vec(
                vec![
                    Paragraph::from_vec(vec![Text::new("unordered list item").into()], None).into(),
                ],
                None,
            )
            .into()],
        };

        assert_eq!(list.serialize(), "- unordered list item");
    }
}
