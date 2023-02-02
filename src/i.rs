use crate::{
    b::BTags,
    p::ParagraphTags,
    parser::{get_iterator, Leaf, Parser, ParserPart},
};

/// Representation of an Italic text
#[derive(Debug, PartialEq)]
pub struct I {
    text: String,
}

impl I {
    pub fn new<S: Into<String>>(text: S) -> Self {
        I { text: text.into() }
    }
}

impl From<I> for String {
    fn from(value: I) -> Self {
        format!("_{}_", value.text)
    }
}

impl From<I> for BTags {
    fn from(value: I) -> Self {
        BTags::I(value)
    }
}

impl From<I> for ParagraphTags {
    fn from(value: I) -> Self {
        ParagraphTags::I(value)
    }
}

impl Leaf for I {}

impl Parser for I {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = get_iterator(input, start_position);
        if let Some(end_postion) = chars.get_token_end_position(vec!['_'], vec!['_']) {
            return Some((
                I::new(
                    input[start_position + 1..end_postion]
                        .to_string()
                        .replace('\n', ""),
                ),
                end_postion + 1,
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::I;

    #[test]
    fn happy_path() {
        let i = I::new("italic");
        assert_eq!(i.text, "italic".to_string());
    }

    #[test]
    fn to_string() {
        let i: String = I::new("italic").into();
        assert_eq!(i, "_italic_".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(I::parse("_italic_", 0), Some((I::new("italic"), 8)));
        assert_eq!(I::parse("not_italic_not", 3), Some((I::new("italic"), 11)));
        assert_eq!(
            I::parse("not_it alic_not", 3),
            Some((I::new("it alic"), 12))
        );
        assert_eq!(I::parse("not italic_not", 3), None);
        assert_eq!(I::parse("*italic not", 0), None);
        assert_eq!(I::parse("_ita\nlic_", 0), Some((I::new("italic"), 9)));
        assert_eq!(I::parse("_ita\n\nlic_", 0), None);
    }
}
