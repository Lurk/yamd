use crate::{
    p::ParagraphTags,
    parser::{Parser, ParserPart},
};

/// Representation of an anchor
#[derive(Debug, PartialEq)]
pub struct A {
    text: Option<String>,
    url: String,
}

impl A {
    pub fn new<S: Into<String>>(url: S, text: Option<String>) -> Self {
        A {
            text,
            url: url.into(),
        }
    }
}

impl From<A> for String {
    fn from(value: A) -> String {
        let text = match value.text {
            Some(text) => text,
            None => value.url.clone(),
        };
        format!("[{}]({})", text, value.url)
    }
}

impl From<A> for ParagraphTags {
    fn from(value: A) -> Self {
        ParagraphTags::A(value)
    }
}

impl Parser for A {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = input.chars().enumerate();
        if start_position != 0 {
            chars.nth(start_position - 1);
        }
        if let Some(first_part) = chars.parse_part('[', ']') {
            println!("{first_part}");
            if let Some(second_part) = chars.parse_part('(', ')') {
                println!("{second_part}");
                return Some((
                    A::new(
                        input[first_part + 2..second_part].to_string(),
                        Some(input[start_position + 1..first_part].to_string()),
                    ),
                    second_part + 1,
                ));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::A;

    #[test]
    fn happy_path() {
        let a = A::new("https://test.io", Some("nice link".to_string()));
        assert_eq!(a.text, Some("nice link".to_string()));
        assert_eq!(a.url, "https://test.io");
    }

    #[test]
    fn to_string_with_text() {
        let a: String = A::new("https://test.io", Some("nice link".to_string())).into();
        assert_eq!(a, "[nice link](https://test.io)".to_string());
    }

    #[test]
    fn to_string_without_text() {
        let a: String = A::new("https://test.io", None).into();
        assert_eq!(a, "[https://test.io](https://test.io)".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(
            A::parse("[1](2)", 0),
            Some((A::new("2", Some("1".to_string())), 6))
        )
    }
}
