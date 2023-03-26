use crate::nodes::{
    anchor::Anchor, bold::Bold, inline_code::InlineCode, italic::Italic,
    strikethrough::Strikethrough, text::Text,
};
use crate::toolkit::node::Node;
use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    tokenizer::{Quantifiers::Once, Matcher},
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
pub struct Paragraph {
    consumed_all_input: bool,
    nodes: Vec<ParagraphNodes>,
}

impl Paragraph {
    pub fn new(consumed_all_input: bool) -> Self {
        Self::new_with_nodes(consumed_all_input, vec![])
    }
    pub fn new_with_nodes(consumed_all_input: bool, nodes: Vec<ParagraphNodes>) -> Self {
        Self {
            consumed_all_input,
            nodes,
        }
    }
}

impl Branch<ParagraphNodes> for Paragraph {
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
        if self.consumed_all_input {
            0
        } else {
            2
        }
    }
}

impl Deserializer for Paragraph {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(body) =
            matcher.get_node_body_with_end_of_input(&[], &[Once('\n'), Once('\n')], true)
        {
            return Self::parse_branch(body, Self::new(input.len() == body.len()));
        }
        None
    }
}

impl Node for Paragraph {
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

impl FallbackNode for Paragraph {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<BranchNodes>
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
    use crate::{
        nodes::bold::Bold,
        nodes::inline_code::InlineCode,
        nodes::{anchor::Anchor, text::Text},
        toolkit::{
            deserializer::{Branch, Deserializer},
            node::Node,
        },
    };

    use super::Paragraph;

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
    fn from_vec() {
        let p: String = Paragraph::new_with_nodes(
            true,
            vec![
                Text::new("simple text ").into(),
                Bold::new_with_nodes(vec![Text::new("bold text").into()]).into(),
                InlineCode::new("let foo='bar';").into(),
            ],
        )
        .serialize();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Paragraph::new_with_nodes(false, vec![Text::new("t").into()]).serialize(),
            "t\n\n".to_string()
        )
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
}
