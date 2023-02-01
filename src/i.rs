use crate::{
    b::BContent,
    p::ParagraphTags,
    parser::{Parser, ParserPart},
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
        format!("*{}*", value.text)
    }
}

impl From<I> for BContent {
    fn from(value: I) -> Self {
        BContent::I(value)
    }
}

impl From<I> for ParagraphTags {
    fn from(value: I) -> Self {
        ParagraphTags::I(value)
    }
}

impl Parser for I {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = input.chars().enumerate();
        if start_position != 0 {
            chars.nth(start_position - 1);
        }
        if let Some(end_postion) = chars.parse_part('*', '*') {
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
        assert_eq!(i, "*italic*".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(I::parse("*italic*", 0), Some((I::new("italic"), 8)));
        assert_eq!(I::parse("not*italic*not", 3), Some((I::new("italic"), 11)));
        assert_eq!(
            I::parse("not*it alic*not", 3),
            Some((I::new("it alic"), 12))
        );
        assert_eq!(I::parse("not italic*not", 3), None);
        assert_eq!(I::parse("*italic not", 0), None);
        assert_eq!(I::parse("*ita\nlic*", 0), Some((I::new("italic"), 9)));
        assert_eq!(I::parse("*ita\n\nlic*", 0), None);
    }
}
