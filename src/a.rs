use crate::{
    deserializer::{Deserializer, Leaf, Tokenizer},
    p::ParagraphTags,
};

/// Representation of an anchor
#[derive(Debug, PartialEq)]
pub struct A {
    text: String,
    url: String,
}

impl A {
    pub fn new<S: Into<String>>(url: S, text: S) -> Self {
        A {
            text: text.into(),
            url: url.into(),
        }
    }
}

impl From<A> for String {
    fn from(value: A) -> String {
        format!("[{}]({})", value.text, value.url)
    }
}

impl From<A> for ParagraphTags {
    fn from(value: A) -> Self {
        ParagraphTags::A(value)
    }
}

impl Leaf for A {}

impl Deserializer for A {
    fn deserialize(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut chars = Tokenizer::new(input, start_position);
        if let Some(first_part) = chars.get_token_body(vec!['['], vec![']']) {
            let first_part = first_part.to_string();
            if let Some(second_part) = chars.get_token_body(vec!['('], vec![')']) {
                return Some((
                    A::new(second_part.to_string(), first_part),
                    chars.get_next_position(),
                ));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::deserializer::Deserializer;

    use super::A;

    #[test]
    fn happy_path() {
        let a = A::new("https://test.io", "nice link");
        assert_eq!(a.text, "nice link");
        assert_eq!(a.url, "https://test.io");
    }

    #[test]
    fn to_string_with_text() {
        let a: String = A::new("https://test.io", "nice link").into();
        assert_eq!(a, "[nice link](https://test.io)".to_string());
    }

    #[test]
    fn from_string() {
        assert_eq!(A::deserialize("[1](2)", 0), Some((A::new("2", "1"), 6)))
    }
}
