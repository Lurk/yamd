/// Representation of strikethrough
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
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
}
