use crate::{
    deserializer::{Branch, Deserializer, Leaf, ParserToTags, Tokenizer},
    i::I,
    p::ParagraphTags,
    s::S,
    serializer::Serializer,
    text::Text,
};

#[derive(Debug, PartialEq)]
pub enum BTags {
    Text(Text),
    I(I),
    S(S),
}

impl Serializer for BTags {
    fn serialize(&self) -> String {
        match self {
            BTags::Text(v) => v.serialize(),
            BTags::I(v) => v.serialize(),
            BTags::S(v) => v.serialize(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct B {
    nodes: Vec<BTags>,
}

impl From<B> for ParagraphTags {
    fn from(value: B) -> Self {
        ParagraphTags::B(value)
    }
}

impl Serializer for B {
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

impl Branch<BTags> for B {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn from_vec(data: Vec<BTags>) -> Self {
        Self { nodes: data }
    }

    fn push<BC: Into<BTags>>(&mut self, element: BC) {
        self.nodes.push(element.into());
    }

    fn get_parsers() -> Vec<ParserToTags<BTags>> {
        vec![
            Box::new(|str, pos| I::parse_to_tag(str, pos)),
            Box::new(|str, pos| S::parse_to_tag(str, pos)),
        ]
    }

    fn get_fallback() -> Box<dyn Fn(&str) -> BTags> {
        Box::new(|str| Text::new(str).into())
    }
}

impl Default for B {
    fn default() -> Self {
        Self::new()
    }
}

impl Deserializer for B {
    fn deserialize(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = Tokenizer::new(input, start_position);
        if let Some(body) = chars.get_token_body(vec!['*', '*'], vec!['*', '*']) {
            let result = Self::parse_branch(body);
            return Some((result, chars.get_next_position()));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        b::B,
        deserializer::{Branch, Deserializer},
        i::I,
        s::S,
        serializer::Serializer,
        text::Text,
    };

    #[test]
    fn only_text() {
        let mut b = B::new();
        b.push(Text::new("B as bold"));
        let str = b.serialize();
        assert_eq!(str, "**B as bold**".to_string());
    }

    #[test]
    fn from_vec() {
        let b: String = B::from_vec(vec![
            Text::new("B as bold ").into(),
            I::new("Italic").into(),
            S::new("Strikethrough").into(),
        ])
        .serialize();
        assert_eq!(b, "**B as bold _Italic_~~Strikethrough~~**".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(
            B::deserialize("**b**", 0),
            Some((B::from_vec(vec![Text::new("b").into()]), 5))
        );

        assert_eq!(
            B::deserialize("**b ~~st~~ _i t_**", 0),
            Some((
                B::from_vec(vec![
                    Text::new("b ").into(),
                    S::new("st").into(),
                    Text::new(" ").into(),
                    I::new("i t").into()
                ]),
                18
            ))
        );
    }
}
