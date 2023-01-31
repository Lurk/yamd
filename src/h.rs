use crate::{mdy::MdyTags, parser::Parser};

#[derive(Debug, PartialEq)]
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

impl Parser for H {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)> {
        if input.chars().nth(start_position) == Some('#') {
            let stop_position = match input[start_position..].find("\n\n") {
                Some(position) => position + start_position + 2,
                None => input.len(),
            };
            if let Some(stop) = input[start_position..stop_position].find(' ') {
                let mut level: String = input[start_position..stop_position].into();
                let text = level.split_off(stop);
                if level.chars().all(|char| char == '#') {
                    return Some((
                        Self::new(text.trim(), level.len().try_into().unwrap_or(0)),
                        stop_position,
                    ));
                }
            }
        }
        None
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
    use crate::parser::Parser;

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

    #[test]
    fn from_string() {
        assert_eq!(H::parse("## Header", 0), Some((H::new("Header", 2), 9)));
        assert_eq!(H::parse("### Head", 0), Some((H::new("Head", 3), 8)));
        assert_eq!(H::parse("not ### Head", 4), Some((H::new("Head", 3), 12)));
        assert_eq!(
            H::parse("not ### Head\n\nsome other thing", 4),
            Some((H::new("Head", 3), 14))
        );
        assert_eq!(H::parse("not a header", 0), None);
        assert_eq!(H::parse("######", 0), None);
        assert_eq!(H::parse("######also not a header", 0), None);
    }
}
