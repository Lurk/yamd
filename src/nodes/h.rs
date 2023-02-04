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
    fn deserialize(input: &str) -> Option<Self> {
        let mut split_position: Option<usize> = None;
        let tokens = ["# ", "## ", "### ", "#### ", "##### ", "###### "];
        for (i, token) in tokens.iter().enumerate() {
            let end_position = i + 1;
            if input.len() > end_position && &&input[..end_position + 1] == token {
                split_position = Some(i + 2);
            }
        }
        if let Some(split_position) = split_position {
            let stop_position = match input.find("\n\n") {
                Some(position) => position + 2,
                None => input.len(),
            };
            let mut level: String = input[..stop_position].into();
            let text = level.split_off(split_position);
            return Some(Self::new(
                text.trim(),
                level.len().try_into().unwrap_or(0) - 1,
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

impl Node for H {
    fn len(&self) -> usize {
        self.text.len() + self.level as usize + 1
    }

    fn get_token_length(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::sd::{
        deserializer::{Deserializer, Node},
        serializer::Serializer,
    };

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
        assert_eq!(H::deserialize("## Header"), Some(H::new("Header", 2)));
        assert_eq!(H::deserialize("### Head"), Some(H::new("Head", 3)));
        assert_eq!(
            H::deserialize("### Head\n\nsome other thing"),
            Some(H::new("Head", 3))
        );
        assert_eq!(H::deserialize("not a header"), None);
        assert_eq!(H::deserialize("######"), None);
        assert_eq!(H::deserialize("######also not a header"), None);
    }

    #[test]
    fn len() {
        assert_eq!(H::new("h", 1).len(), 3);
        assert_eq!(H::new("h", 2).len(), 4);
    }
}
