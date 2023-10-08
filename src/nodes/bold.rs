use std::fmt::Display;

use serde::Serialize;

use crate::{
    nodes::italic::Italic,
    nodes::strikethrough::Strikethrough,
    nodes::text::Text,
    toolkit::{
        context::Context,
        deserializer::{Branch, DefinitelyNode, Deserializer, MaybeNode},
        matcher::Matcher,
        node::Node,
    },
};

#[derive(Debug, PartialEq, Serialize)]
pub enum BoldNodes {
    Text(Text),
    I(Italic),
    S(Strikethrough),
}

impl From<Text> for BoldNodes {
    fn from(value: Text) -> Self {
        BoldNodes::Text(value)
    }
}

impl From<Italic> for BoldNodes {
    fn from(value: Italic) -> Self {
        BoldNodes::I(value)
    }
}

impl From<Strikethrough> for BoldNodes {
    fn from(value: Strikethrough) -> Self {
        BoldNodes::S(value)
    }
}

impl Display for BoldNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoldNodes::Text(node) => write!(f, "{}", node),
            BoldNodes::I(node) => write!(f, "{}", node),
            BoldNodes::S(node) => write!(f, "{}", node),
        }
    }
}

impl Node for BoldNodes {
    fn len(&self) -> usize {
        match self {
            BoldNodes::Text(node) => node.len(),
            BoldNodes::I(node) => node.len(),
            BoldNodes::S(node) => node.len(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Bold {
    pub nodes: Vec<BoldNodes>,
}

impl Bold {
    pub fn new() -> Self {
        Self::new_with_nodes(vec![])
    }

    pub fn new_with_nodes(nodes: Vec<BoldNodes>) -> Self {
        Self { nodes }
    }
}

impl Branch<BoldNodes> for Bold {
    fn push<BC: Into<BoldNodes>>(&mut self, element: BC) {
        self.nodes.push(element.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<BoldNodes>> {
        vec![Italic::maybe_node(), Strikethrough::maybe_node()]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<BoldNodes>> {
        Some(Box::new(|str| Text::new(str).into()))
    }
    fn get_outer_token_length(&self) -> usize {
        4
    }
}

impl Default for Bold {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Bold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "**{}**",
            self.nodes
                .iter()
                .map(|element| { element.to_string() })
                .collect::<Vec<String>>()
                .concat()
        )
    }
}

impl Node for Bold {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

impl Deserializer for Bold {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(bold) = matcher.get_match("**", "**", false) {
            return Self::parse_branch(bold.body, Self::new());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::bold::Bold,
        nodes::italic::Italic,
        nodes::strikethrough::Strikethrough,
        nodes::text::Text,
        toolkit::{
            deserializer::{Branch, Deserializer},
            node::Node,
        },
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn only_text() {
        let mut b = Bold::new();
        b.push(Text::new("B as bold"));
        let str = b.to_string();
        assert_eq!(str, "**B as bold**".to_string());
    }

    #[test]
    fn from_vec() {
        let b: String = Bold::new_with_nodes(vec![
            Text::new("B as bold ").into(),
            Italic::new("Italic").into(),
            Strikethrough::new("Strikethrough").into(),
        ])
        .to_string();
        assert_eq!(b, "**B as bold _Italic_~~Strikethrough~~**".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Bold::deserialize("**b**"),
            Some(Bold::new_with_nodes(vec![Text::new("b").into()]))
        );

        assert_eq!(
            Bold::deserialize("**b ~~st~~ _i t_**"),
            Some(Bold::new_with_nodes(vec![
                Text::new("b ").into(),
                Strikethrough::new("st").into(),
                Text::new(" ").into(),
                Italic::new("i t").into()
            ]))
        );
    }

    #[test]
    fn len() {
        assert_eq!(Bold::new_with_nodes(vec![Text::new("T").into()]).len(), 5);
        assert_eq!(
            Bold::new_with_nodes(vec![Text::new("T").into(), Strikethrough::new("S").into()]).len(),
            10
        );
    }

    #[test]
    fn default() {
        assert_eq!(Bold::default(), Bold::new());
    }
}
