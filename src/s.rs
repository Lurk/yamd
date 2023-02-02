use crate::{
    b::BContent,
    p::ParagraphTags,
    parser::{Parser, ParserPart},
};

/// Representation of strikethrough
#[derive(Debug, PartialEq)]
pub struct S {
    text: String,
}

impl S {
    pub fn new<IS: Into<String>>(text: IS) -> Self {
        S { text: text.into() }
    }
}

impl From<S> for String {
    fn from(value: S) -> Self {
        format!("~~{}~~", value.text)
    }
}

impl From<S> for BContent {
    fn from(value: S) -> Self {
        BContent::S(value)
    }
}

impl From<S> for ParagraphTags {
    fn from(value: S) -> Self {
        ParagraphTags::S(value)
    }
}

impl Parser for S {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = Self::get_iterator(input, start_position);
        if let Some(end_position) = chars.parse_part(vec!['~', '~'], vec!['~', '~']) {
            return Some((
                S::new(
                    input[start_position + 2..end_position - 1]
                        .to_string()
                        .replace('\n', ""),
                ),
                end_position,
            ));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::S;

    #[test]
    fn happy_path() {
        let s = S::new("2+2=5");
        assert_eq!(s.text, "2+2=5".to_string());
    }

    #[test]
    fn to_string() {
        let s: String = S::new("2+2=5").into();
        assert_eq!(s, "~~2+2=5~~".to_string());
    }

    #[test]
    fn parse() {
        assert_eq!(S::parse("~~2+2=5~~", 0), Some((S::new("2+2=5"), 8)));
        assert_eq!(S::parse("not ~~is~~not", 4), Some((S::new("is"), 9)));
        assert_eq!(S::parse("~~not", 0), None);
        assert_eq!(S::parse("~~i\n\ns~~", 0), None);
        assert_eq!(S::parse("~~i\ns~~", 0), Some((S::new("is"), 6)));
    }
}
