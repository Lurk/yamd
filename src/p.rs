use crate::a::A;
use crate::b::B;
use crate::i::I;
use crate::inline_code::InlineCode;
use crate::mdy::MdyTags;
use crate::s::S;
use crate::text::Text;

#[derive(Debug)]
pub enum ParagraphTags {
    A(A),
    B(B),
    I(I),
    S(S),
    Text(Text),
    InlineCode(InlineCode),
}

#[derive(Debug)]
pub struct P {
    data: Vec<ParagraphTags>,
}

impl P {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from_vec(data: Vec<ParagraphTags>) -> Self {
        Self { data }
    }

    pub fn push<TP: Into<ParagraphTags>>(mut self, element: TP) -> Self {
        self.data.push(element.into());
        self
    }
}

impl From<P> for String {
    fn from(value: P) -> Self {
        value
            .data
            .into_iter()
            .map(|element| match element {
                ParagraphTags::A(v) => v.into(),
                ParagraphTags::B(v) => v.into(),
                ParagraphTags::I(v) => v.into(),
                ParagraphTags::S(v) => v.into(),
                ParagraphTags::Text(v) => v.into(),
                ParagraphTags::InlineCode(v) => v.into(),
            })
            .collect::<Vec<String>>()
            .concat()
    }
}

impl Default for P {
    fn default() -> Self {
        Self::new()
    }
}

impl From<P> for MdyTags {
    fn from(value: P) -> Self {
        MdyTags::P(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{b::B, deserializer::Branch, inline_code::InlineCode, text::Text};

    use super::P;

    #[test]
    fn push() {
        let p: String = P::new()
            .push(Text::new("simple text "))
            .push(B::from_vec(vec![Text::new("bold text").into()]))
            .push(InlineCode::new("let foo='bar';"))
            .into();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }

    #[test]
    fn from_vec() {
        let p: String = P::from_vec(vec![
            Text::new("simple text ").into(),
            B::from_vec(vec![Text::new("bold text").into()]).into(),
            InlineCode::new("let foo='bar';").into(),
        ])
        .into();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }
}
