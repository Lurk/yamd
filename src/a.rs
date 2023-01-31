use crate::p::ParagraphTags;

/// Representation of an anchor
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
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
}
