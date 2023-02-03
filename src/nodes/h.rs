use crate::{
    nodes::yamd::YamdNodes,
    sd::{
        deserializer::{Deserializer, Node},
        serializer::Serializer,
    },
};

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

impl Deserializer for H {
    fn deserialize(input: &str, start_position: usize) -> Option<(Self, usize)> {
        let mut split_position: Option<usize> = None;
        let tokens = ["# ", "## ", "### ", "#### ", "##### ", "###### "];
        for (i, token) in tokens.iter().enumerate() {
            let end_position = start_position + i + 1;
            if input.len() > end_position && &&input[start_position..end_position + 1] == token {
                split_position = Some(i + 2);
            }
        }
        if let Some(split_position) = split_position {
            let stop_position = match input[start_position..].find("\n\n") {
                Some(position) => position + start_position + 2,
                None => input.len(),
            };
            let mut level: String = input[start_position..stop_position].into();
            let text = level.split_off(split_position);
            return Some((
                Self::new(text.trim(), level.len().try_into().unwrap_or(0) - 1),
                stop_position,
            ));
        }
        None
    }
}

impl Serializer for H {
    fn serialize(&self) -> String {
        let level = String::from('#').repeat(self.level as usize);
        format!("{} {}", level, self.text)
    }
}

impl From<H> for YamdNodes {
    fn from(value: H) -> Self {
        YamdNodes::H(value)
    }
}

impl Node for H {}

#[cfg(test)]
mod tests {
    use crate::sd::{deserializer::Deserializer, serializer::Serializer};

    use super::H;

    #[test]
    fn level_one() {
        let h: String = H::new("Header", 1).serialize();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_gt_six() {
        let h: String = H::new("Header", 7).serialize();
        assert_eq!(h, "###### Header");
        let h: String = H::new("Header", 34).serialize();
        assert_eq!(h, "###### Header");
    }

    #[test]
    fn level_eq_zero() {
        let h: String = H::new("Header", 0).serialize();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_eq_four() {
        let h: String = H::new("Header", 4).serialize();
        assert_eq!(h, "#### Header");
    }

    #[test]
    fn from_string() {
        assert_eq!(
            H::deserialize("## Header", 0),
            Some((H::new("Header", 2), 9))
        );
        assert_eq!(H::deserialize("### Head", 0), Some((H::new("Head", 3), 8)));
        assert_eq!(
            H::deserialize("not ### Head", 4),
            Some((H::new("Head", 3), 12))
        );
        assert_eq!(
            H::deserialize("not ### Head\n\nsome other thing", 4),
            Some((H::new("Head", 3), 14))
        );
        assert_eq!(H::deserialize("not a header", 0), None);
        assert_eq!(H::deserialize("######", 0), None);
        assert_eq!(H::deserialize("######also not a header", 0), None);
    }
}