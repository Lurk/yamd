use crate::a::A;
use crate::b::B;
use crate::i::I;
use crate::inline_code::InlineCode;
use crate::mdy::MdyContent;
use crate::s::S;
use crate::text::Text;

#[derive(Debug)]
pub enum ParagraphContent {
    A(A),
    B(B),
    I(I),
    S(S),
    Text(Text),
    InlineCode(InlineCode),
}

#[derive(Debug)]
pub struct P {
    data: Vec<ParagraphContent>,
}

impl P {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from_vec(data: Vec<ParagraphContent>) -> Self {
        Self { data }
    }

    pub fn push<TP: Into<ParagraphContent>>(mut self, element: TP) -> Self {
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
                ParagraphContent::A(v) => v.into(),
                ParagraphContent::B(v) => v.into(),
                ParagraphContent::I(v) => v.into(),
                ParagraphContent::S(v) => v.into(),
                ParagraphContent::Text(v) => v.into(),
                ParagraphContent::InlineCode(v) => v.into(),
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

impl From<P> for MdyContent {
    fn from(value: P) -> Self {
        MdyContent::P(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{b::B, inline_code::InlineCode, text::Text};

    use super::P;

    #[test]
    fn push() {
        let p: String = P::new()
            .push(Text::new("simple text "))
            .push(B::new().push(Text::new("bold text")))
            .push(InlineCode::new("let foo='bar';"))
            .into();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }

    #[test]
    fn from_vec() {
        let p: String = P::from_vec(vec![
            Text::new("simple text ").into(),
            B::new().push(Text::new("bold text")).into(),
            InlineCode::new("let foo='bar';").into(),
        ])
        .into();

        assert_eq!(p, "simple text **bold text**`let foo='bar';`".to_string());
    }
}
