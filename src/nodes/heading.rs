use crate::{
    nodes::yamd::YamdNodes,
    sd::{
        deserializer::{Deserializer, Node, Tokenizer},
        serializer::Serializer,
    },
};

#[derive(Debug, PartialEq)]
pub struct Heading {
    level: u8,
    text: String,
}

impl Heading {
    pub fn new<S: Into<String>>(text: S, level: u8) -> Self {
        let normalized_level = match level {
            0 => 1,
            7.. => 6,
            l => l,
        };
        Heading {
            text: text.into(),
            level: normalized_level,
        }
    }
}

impl Deserializer for Heading {
    fn deserialize(input: &str) -> Option<Self> {
        let start_tokens = [
            vec!['#', ' '],
            vec!['#', '#', ' '],
            vec!['#', '#', '#', ' '],
            vec!['#', '#', '#', '#', ' '],
            vec!['#', '#', '#', '#', '#', ' '],
            vec!['#', '#', '#', '#', '#', '#', ' '],
        ];

        for (i, start_token) in start_tokens.iter().enumerate() {
            let mut tokenizer = Tokenizer::new_with_custom_hard_stop(input, vec![]);
            if let Some(body) = tokenizer.get_token_body(start_token.to_vec(), vec!['\n', '\n']) {
                return Some(Self::new(body, (i + 1).try_into().unwrap_or(1)));
            }
        }

        None
    }
}

impl Serializer for Heading {
    fn serialize(&self) -> String {
        let level = String::from('#').repeat(self.level as usize);
        format!("{} {}", level, self.text)
    }
}

impl From<Heading> for YamdNodes {
    fn from(value: Heading) -> Self {
        YamdNodes::H(value)
    }
}

impl Node for Heading {
    fn len(&self) -> usize {
        self.text.len() + self.level as usize + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::sd::{
        deserializer::{Deserializer, Node},
        serializer::Serializer,
    };

    use super::Heading;

    #[test]
    fn level_one() {
        let h = Heading::new("Header", 1).serialize();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_gt_six() {
        let h = Heading::new("Header", 7).serialize();
        assert_eq!(h, "###### Header");
        let h = Heading::new("Header", 34).serialize();
        assert_eq!(h, "###### Header");
    }

    #[test]
    fn level_eq_zero() {
        let h = Heading::new("Header", 0).serialize();
        assert_eq!(h, "# Header");
    }

    #[test]
    fn level_eq_four() {
        let h = Heading::new("Header", 4).serialize();
        assert_eq!(h, "#### Header");
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Heading::deserialize("## Header"),
            Some(Heading::new("Header", 2))
        );
        assert_eq!(
            Heading::deserialize("### Head"),
            Some(Heading::new("Head", 3))
        );
        assert_eq!(
            Heading::deserialize("### Head\n\nsome other thing"),
            Some(Heading::new("Head", 3))
        );
        assert_eq!(Heading::deserialize("not a header"), None);
        assert_eq!(Heading::deserialize("######"), None);
        assert_eq!(Heading::deserialize("######also not a header"), None);
    }

    #[test]
    fn len() {
        assert_eq!(Heading::new("h", 1).len(), 3);
        assert_eq!(Heading::new("h", 2).len(), 4);
    }
}
