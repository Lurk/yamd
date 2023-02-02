use crate::{
    i::I,
    p::ParagraphTags,
    parser::{Branch, Parser, ParserPart, ParserToTags},
    s::S,
    text::Text,
};

#[derive(Debug, PartialEq)]
pub enum BContent {
    Text(Text),
    I(I),
    S(S),
}

#[derive(Debug, PartialEq)]
pub struct B {
    data: Vec<BContent>,
}

impl From<B> for ParagraphTags {
    fn from(value: B) -> Self {
        ParagraphTags::B(value)
    }
}

impl From<B> for String {
    fn from(value: B) -> Self {
        format!(
            "**{}**",
            value
                .data
                .into_iter()
                .map(|element| {
                    match element {
                        BContent::Text(v) => v.into(),
                        BContent::I(v) => v.into(),
                        BContent::S(v) => v.into(),
                    }
                })
                .collect::<Vec<String>>()
                .concat()
        )
    }
}

impl Branch<BContent> for B {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn from_vec(data: Vec<BContent>) -> Self {
        Self { data }
    }

    fn push<BC: Into<BContent>>(&mut self, element: BC) {
        self.data.push(element.into());
    }

    fn get_parsers() -> Vec<ParserToTags<BContent>> {
        vec![
            Box::new(|str, pos| I::parse_to_tag(str, pos)),
            Box::new(|str, pos| S::parse_to_tag(str, pos)),
        ]
    }

    fn get_fallback() -> Box<dyn Fn(&str) -> BContent> {
        Box::new(|str| Text::new(str).into())
    }
}

impl Default for B {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<BContent> for B {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = Self::get_iterator(input, start_position);
        if let Some(end_position) = chars.parse_part(vec!['*', '*'], vec!['*', '*']) {
            let chunk = &input[start_position + 2..end_position - 1];
            let result = Self::parse_branch(chunk);
            return Some((result, end_position + 1));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        b::B,
        i::I,
        parser::{Branch, Parser},
        s::S,
        text::Text,
    };

    #[test]
    fn only_text() {
        let mut b = B::new();
        b.push(Text::new("B as bold"));
        let str: String = b.into();
        assert_eq!(str, "**B as bold**".to_string());
    }

    #[test]
    fn from_vec() {
        let b: String = B::from_vec(vec![
            Text::new("B as bold ").into(),
            I::new("Italic").into(),
            S::new("Strikethrough").into(),
        ])
        .into();
        assert_eq!(b, "**B as bold _Italic_~~Strikethrough~~**".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(
            B::parse("**b**", 0),
            Some((B::from_vec(vec![Text::new("b").into()]), 5))
        );

        assert_eq!(
            B::parse("**b ~~st~~ _i t_**", 0),
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
