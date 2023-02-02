use crate::{
    p::ParagraphTags,
    parser::{Parser, ParserPart},
};

#[derive(Debug, PartialEq)]
pub struct InlineCode {
    text: String,
}

impl InlineCode {
    pub fn new<S: Into<String>>(text: S) -> Self {
        InlineCode { text: text.into() }
    }
}

impl From<InlineCode> for String {
    fn from(value: InlineCode) -> Self {
        format!("`{}`", value.text)
    }
}

impl From<InlineCode> for ParagraphTags {
    fn from(value: InlineCode) -> Self {
        ParagraphTags::InlineCode(value)
    }
}

impl Parser for InlineCode {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = Self::get_iterator(input, start_position);
        if let Some(end_position) = chars.parse_part(vec!['`'], vec!['`']) {
            return Some((
                InlineCode::new(
                    input[start_position + 1..end_position]
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

    use super::InlineCode;

    #[test]
    fn to_string() {
        let inline_code: String = InlineCode::new("const bar = 'baz'").into();
        assert_eq!(inline_code, "`const bar = 'baz'`".to_string())
    }

    #[test]
    fn from_string() {
        assert_eq!(InlineCode::parse("`1`", 0), Some((InlineCode::new('1'), 2)));
        assert_eq!(
            InlineCode::parse("not`1`", 3),
            Some((InlineCode::new('1'), 5))
        );
        assert_eq!(
            InlineCode::parse("`const \nfoo='bar'`", 0),
            Some((InlineCode::new("const foo='bar'"), 17))
        );
        assert_eq!(InlineCode::parse("not`a", 3), None);
        assert_eq!(InlineCode::parse("`const \n\nfoo='bar'`", 0), None);
    }
}
