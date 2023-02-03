use crate::{
    b::BTags,
    p::ParagraphTags,
    parser::{get_iterator, Deserializer, Leaf, ParserPart},
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

impl From<S> for BTags {
    fn from(value: S) -> Self {
        BTags::S(value)
    }
}

impl From<S> for ParagraphTags {
    fn from(value: S) -> Self {
        ParagraphTags::S(value)
    }
}

impl Leaf for S {}

impl Deserializer for S {
    fn deserialize(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = get_iterator(input, start_position);
        if let Some(end_position) = chars.get_token_end_position(vec!['~', '~'], vec!['~', '~']) {
            return Some((
                S::new(
                    input[start_position + 2..end_position - 1]
                        .to_string()
                        .replace('\n', ""),
                ),
                end_position + 1,
            ));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Deserializer;

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
        assert_eq!(S::deserialize("~~2+2=5~~", 0), Some((S::new("2+2=5"), 9)));
        assert_eq!(S::deserialize("not ~~is~~not", 4), Some((S::new("is"), 10)));
        assert_eq!(S::deserialize("~~not", 0), None);
        assert_eq!(S::deserialize("~~i\n\ns~~", 0), None);
        assert_eq!(S::deserialize("~~i\ns~~", 0), Some((S::new("is"), 7)));
    }
}
