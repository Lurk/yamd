use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, deserializer::Deserializer, matcher::Matcher, node::Node};

use super::paragraph::Paragraph;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Highlight {
    pub header: Option<String>,
    pub icon: Option<String>,
    pub nodes: Vec<Paragraph>,
}

impl Highlight {
    pub fn new<H: Into<String>, I: Into<String>>(
        header: Option<H>,
        icon: Option<I>,
        nodes: Vec<Paragraph>,
    ) -> Self {
        Self {
            header: header.map(|header| header.into()),
            icon: icon.map(|icon| icon.into()),
            nodes,
        }
    }
}

impl Display for Highlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = match &self.header {
            Some(header) => format!(">> {header}\n"),
            None => String::new(),
        };
        let icon = match &self.icon {
            Some(icon) => format!("> {icon}\n"),
            None => String::new(),
        };
        write!(
            f,
            ">>>\n{header}{icon}{}\n>>>",
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            header = header,
            icon = icon,
        )
    }
}

impl Node for Highlight {
    fn len(&self) -> usize {
        let delimiter_length = if self.nodes.len() == 0 {
            0
        } else {
            (self.nodes.len() - 1) * 2
        };
        self.nodes.iter().map(|node| node.len()).sum::<usize>()
            + delimiter_length
            + 8
            + self.header.as_ref().map_or(0, |header| header.len() + 4)
            + self.icon.as_ref().map_or(0, |icon| icon.len() + 3)
    }
}

impl Deserializer for Highlight {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut outer_matcher = Matcher::new(input);
        if let Some(highlight) = outer_matcher.get_match(">>>\n", "\n>>>", false) {
            let mut matcher = Matcher::new(highlight.body);
            let header = matcher
                .get_match(">> ", "\n", false)
                .map(|header| header.body);

            let icon = matcher.get_match("> ", "\n", false).map(|icon| icon.body);
            return Some(Self::new(
                header,
                icon,
                matcher
                    .get_rest()
                    .split("\n\n")
                    .map(|paragraph| {
                        Paragraph::deserialize(paragraph).expect("Paragraph always deserializes")
                    })
                    .collect::<Vec<Paragraph>>(),
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{highlight::Highlight, paragraph::Paragraph, text::Text},
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn len() {
        assert_eq!(
            Highlight::new(
                Some("h"),
                Some("i"),
                vec![
                    Paragraph::new(vec![Text::new("t").into()]).into(),
                    Paragraph::new(vec![Text::new("t").into()]).into()
                ]
            )
            .len(),
            21
        );
        assert_eq!(
            Highlight::new::<String, String>(
                None,
                None,
                vec![
                    Paragraph::new(vec![Text::new("t").into()]).into(),
                    Paragraph::new(vec![Text::new("t").into()]).into()
                ]
            )
            .len(),
            12
        );
    }
    #[test]
    fn serialize() {
        assert_eq!(
            Highlight::new(
                Some("h"),
                Some("i"),
                vec![
                    Paragraph::new(vec![Text::new("t").into()]).into(),
                    Paragraph::new(vec![Text::new("t").into()]).into()
                ]
            )
            .to_string(),
            String::from(">>>\n>> h\n> i\nt\n\nt\n>>>")
        );
        assert_eq!(
            Highlight::new::<String, String>(
                None,
                None,
                vec![
                    Paragraph::new(vec![Text::new("t").into()]).into(),
                    Paragraph::new(vec![Text::new("t").into()]).into()
                ]
            )
            .to_string(),
            String::from(">>>\nt\n\nt\n>>>")
        );
    }

    #[test]
    fn deserialize() {
        assert_eq!(
            Highlight::deserialize(">>>\n>> h\n> i\nt\n\nt\n>>>"),
            Some(Highlight::new(
                Some("h"),
                Some("i"),
                vec![
                    Paragraph::new(vec![Text::new("t").into()]).into(),
                    Paragraph::new(vec![Text::new("t").into()]).into()
                ]
            ))
        );

        assert_eq!(
            Highlight::deserialize(">>>\n>> h\n> i\nt\n\nt2\n>>>\n\n"),
            Some(Highlight::new(
                Some("h"),
                Some("i"),
                vec![
                    Paragraph::new(vec![Text::new("t").into()]).into(),
                    Paragraph::new(vec![Text::new("t2").into()]).into()
                ]
            ))
        )
    }

    #[test]
    fn empty_highlight() {
        let highlight = Highlight::new::<String, String>(None, None, vec![]);
        assert_eq!(highlight.len(), 8);
    }

    #[test]
    fn starts_with_delimeter() {
        let input = ">>>


test

test2
>>>";
        let highlight = Highlight::deserialize(input).unwrap();
        assert_eq!(highlight.len(), input.len());
        assert_eq!(
            highlight,
            Highlight::new::<&str, &str>(
                None,
                None,
                vec![
                    Paragraph::new(vec![]).into(),
                    Paragraph::new(vec![Text::new("test").into()]).into(),
                    Paragraph::new(vec![Text::new("test2").into()]).into(),
                ]
            )
        );
    }
}
