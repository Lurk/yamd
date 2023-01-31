use crate::mdy::MdyTags;

#[derive(Debug)]
pub struct H {
    level: u8,
    text: String,
}

impl H {
    pub fn new<S: Into<String>>(text: S, level: u8) -> Self {
        let normalized_level = match level {
            0 => 1,
            7.. => 6,
            l => l,
        };
        H {
            text: text.into(),
            level: normalized_level,
        }
    }
}

impl From<H> for String {
    fn from(value: H) -> Self {
        let key = String::from('#').repeat(value.level as usize);
        format!("{} {}", key, value.text)
    }
}

impl From<H> for MdyTags {
    fn from(value: H) -> Self {
        MdyTags::H(value)
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

    #[test]
    fn level_gt_six() {
        let h: String = H::new("Header", 7).into();
        assert_eq!(h, "###### Header");
        let h: String = H::new("Header", 34).into();
        assert_eq!(h, "###### Header");
    }

    #[test]
    fn level_eq_zero() {
        let h: String = H::new("Header", 0).into();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_eq_four() {
        let h: String = H::new("Header", 4).into();
        assert_eq!(h, "#### Header");
    }
}
