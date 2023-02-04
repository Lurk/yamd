use crate::{
    nodes::italic::Italic,
    nodes::p::ParagraphNode,
    nodes::s::S,
    nodes::text::Text,
    sd::deserializer::{Branch, Deserializer, MaybeNode, Node, Tokenizer},
    sd::{deserializer::FallbackNode, serializer::Serializer},
};

#[derive(Debug, PartialEq)]
pub enum BoldNodes {
    Text(Text),
    I(Italic),
    S(S),
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

impl Serializer for BoldNodes {
    fn serialize(&self) -> String {
        match self {
            BoldNodes::Text(v) => v.serialize(),
            BoldNodes::I(v) => v.serialize(),
            BoldNodes::S(v) => v.serialize(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Bold {
    nodes: Vec<BoldNodes>,
}

impl From<Bold> for ParagraphNode {
    fn from(value: Bold) -> Self {
        ParagraphNode::B(value)
    }
}

impl Serializer for Bold {
    fn serialize(&self) -> String {
        format!(
            "**{}**",
            self.nodes
                .iter()
                .map(|element| { element.serialize() })
                .collect::<Vec<String>>()
                .concat()
        )
    }
}

impl Branch<BoldNodes> for Bold {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn from_vec(data: Vec<BoldNodes>) -> Self {
        Self { nodes: data }
    }

    fn push<BC: Into<BoldNodes>>(&mut self, element: BC) {
        self.nodes.push(element.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<BoldNodes>> {
        vec![Box::new(Italic::maybe_node), Box::new(S::maybe_node)]
    }

    fn get_fallback_node() -> FallbackNode<BoldNodes> {
        Box::new(|str| Text::new(str).into())
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

impl Node for Bold {
    fn len(&self) -> usize {
        self.nodes.iter().map(|node| node.len()).sum::<usize>() + self.get_outer_token_length()
    }
}

impl Deserializer for Bold {
    fn deserialize(input: &str) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(body) = tokenizer.get_token_body(vec!['*', '*'], vec!['*', '*']) {
            return Some(Self::parse_branch(body));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::bold::Bold,
        nodes::italic::Italic,
        nodes::s::S,
        nodes::text::Text,
        sd::deserializer::{Branch, Deserializer},
        sd::{deserializer::Node, serializer::Serializer},
    };

    #[test]
    fn only_text() {
        let mut b = Bold::new();
        b.push(Text::new("B as bold"));
        let str = b.serialize();
        assert_eq!(str, "**B as bold**".to_string());
    }

    #[test]
    fn from_vec() {
        let b: String = Bold::from_vec(vec![
            Text::new("B as bold ").into(),
            Italic::new("Italic").into(),
            S::new("Strikethrough").into(),
        ])
        .serialize();
        assert_eq!(b, "**B as bold _Italic_~~Strikethrough~~**".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Bold::deserialize("**b**"),
            Some(Bold::from_vec(vec![Text::new("b").into()]))
        );

        assert_eq!(
            Bold::deserialize("**b ~~st~~ _i t_**"),
            Some(Bold::from_vec(vec![
                Text::new("b ").into(),
                S::new("st").into(),
                Text::new(" ").into(),
                Italic::new("i t").into()
            ]))
        );
    }

    #[test]
    fn len() {
        assert_eq!(Bold::from_vec(vec![Text::new("T").into()]).len(), 5);
        assert_eq!(
            Bold::from_vec(vec![Text::new("T").into(), S::new("S").into()]).len(),
            10
        );
    }
}
