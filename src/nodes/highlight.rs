use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, parser::Parse};

use super::paragraph::Paragraph;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Highlight {
    pub title: Option<String>,
    pub icon: Option<String>,
    pub nodes: Vec<Paragraph>,
}

impl Highlight {
    pub fn new<T: Into<String>, I: Into<String>>(
        title: Option<T>,
        icon: Option<I>,
        nodes: Vec<Paragraph>,
    ) -> Self {
        Self {
            title: title.map(|title| title.into()),
            icon: icon.map(|icon| icon.into()),
            nodes,
        }
    }
}

impl Display for Highlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = match &self.title {
            Some(title) => format!(">> {title}\n"),
            None => String::new(),
        };
        let icon = match &self.icon {
            Some(icon) => format!("> {icon}\n"),
            None => String::new(),
        };
        write!(
            f,
            ">>>\n{title}{icon}{}\n>>>",
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            title = title,
            icon = icon,
        )
    }
}

impl Parse for Highlight {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if input[current_position..].starts_with(">>>\n") {
            if let Some(end) = input[current_position + 4..].find("\n>>>") {
                let current_position = current_position + 4;
                let mut start = current_position + 4;
                let mut title = None;
                let mut icon = None;
                if input[start..end].starts_with(">> ") {
                    start += 3;
                    if let Some(local_end) = input[start..end].find('\n') {
                        title = Some(input[start..start + local_end].to_string());
                        start += local_end + 1;
                    }
                }
                if input[start..end].starts_with("> ") {
                    start += 2;
                    if let Some(local_end) = input[start..end].find('\n') {
                        icon = Some(input[start..start + local_end].to_string());
                        start += local_end + 1;
                    }
                }
                let mut nodes = vec![];
                input[start..end].split("\n\n").for_each(|node| {
                    let (node, end) = Paragraph::parse(node, 0, None)
                        .expect("Paragraph should never fail to parse");
                    nodes.push(node);
                    start += end + 2;
                });
                return Some((
                    Highlight::new(title, icon, nodes),
                    start + 4 + current_position,
                ));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::{highlight::Highlight, paragraph::Paragraph, text::Text},
        toolkit::parser::Parse,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize() {
        assert_eq!(
            Highlight::new(
                Some("h"),
                Some("i"),
                vec![
                    Paragraph::new(vec![Text::new("t").into()]),
                    Paragraph::new(vec![Text::new("t").into()])
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
                    Paragraph::new(vec![Text::new("t").into()]),
                    Paragraph::new(vec![Text::new("t").into()])
                ]
            )
            .to_string(),
            String::from(">>>\nt\n\nt\n>>>")
        );
    }

    #[test]
    fn parse() {
        assert_eq!(
            Highlight::parse(">>>\n>> h\n> i\nt\n\nt\n>>>", 0, None),
            Some((
                Highlight::new(
                    Some("h"),
                    Some("i"),
                    vec![
                        Paragraph::new(vec![Text::new("t").into()]),
                        Paragraph::new(vec![Text::new("t").into()])
                    ]
                ),
                20
            ))
        );

        assert_eq!(
            Highlight::parse(">>>\n>> h\n> i\nt\n\nt2\n>>>\n\n", 0, None),
            Some((
                Highlight::new(
                    Some("h"),
                    Some("i"),
                    vec![
                        Paragraph::new(vec![Text::new("t").into()]),
                        Paragraph::new(vec![Text::new("t2").into()])
                    ]
                ),
                26
            ))
        )
    }

    #[test]
    fn empty_highlight() {
        let highlight = Highlight::new::<String, String>(None, None, vec![]);
        assert_eq!(highlight.to_string(), ">>>\n\n>>>");
    }

    #[test]
    fn starts_with_delimeter() {
        let input = ">>>


test

test2
>>>";
        let highlight = Highlight::parse(input, 0, None).unwrap();
        assert_eq!(
            highlight,
            (
                Highlight::new::<&str, &str>(
                    None,
                    None,
                    vec![
                        Paragraph::new(vec![]).into(),
                        Paragraph::new(vec![Text::new("test").into()]).into(),
                        Paragraph::new(vec![Text::new("test2").into()]).into(),
                    ]
                ),
                20
            )
        );
    }
}
