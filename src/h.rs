use crate::mdy::MdyContent;

#[derive(Debug)]
pub struct H {
    level: u8,
    text: String,
}

impl H {
    pub fn new<S: Into<String>>(text: S, level: u8) -> Self {
        H {
            text: text.into(),
            level,
        }
    }
}

impl From<H> for String {
    fn from(value: H) -> Self {
        let key = String::from('#').repeat(value.level as usize);
        format!("{} {}", key, value.text)
    }
}

impl From<H> for MdyContent {
    fn from(value: H) -> Self {
        MdyContent::H(value)
    }
}

#[cfg(test)]
mod tests {
    use super::H;

    #[test]
    fn level_one() {
        let h: String = H::new("Header", 1).into();
        assert_eq!(h, "# Header");
    }
}
