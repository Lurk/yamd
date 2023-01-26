/// Representation of an Italic text
#[derive(Debug)]
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
        value.text
    }
}

#[cfg(test)]
mod tests {
    use super::I;

    #[test]
    fn happy_path() {
        let i = I::new("italic");
        assert_eq!(i.text, "italic".to_string());
    }

    #[test]
    fn to_string() {
        let i: String = I::new("italic").into();
        assert_eq!(i, "italic".to_string());
    }
}
