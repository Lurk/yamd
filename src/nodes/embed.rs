use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, parser::Parse};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Embed {
    pub args: String,
    pub kind: String,
}

impl Embed {
    pub fn new<S: Into<String>>(kind: S, args: S) -> Self {
        Self {
            kind: kind.into(),
            args: args.into(),
        }
    }
}

impl Display for Embed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{{{}|{}}}}}", self.kind, self.args)
    }
}

impl Parse for Embed {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if input[current_position..].starts_with("{{") {
            if let Some(middle) = input[current_position + 2..].find('|') {
                if let Some(end) = input[current_position + middle + 1..].find("}}") {
                    return Some((
                        Embed::new(
                            &input[current_position + 2..current_position + middle],
                            &input[current_position + middle + 1
                                ..current_position + middle + 1 + end],
                        ),
                        2 + middle + 1 + end + 2 - current_position,
                    ));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{nodes::embed::Embed, toolkit::parser::Parse};

    #[test]
    fn serializer() {
        assert_eq!(
            Embed::new("youtube", "https://www.youtube.com/embed/wsfdjlkjsdf",).to_string(),
            "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}"
        );
    }

    #[test]
    fn parse() {
        assert_eq!(
            Embed::parse(
                "{{youtube|https://www.youtube.com/embed/wsfdjlkjsdf}}",
                0,
                None
            ),
            Some((
                Embed::new("youtube", "https://www.youtube.com/embed/wsfdjlkjsdf",),
                49
            ))
        );
    }

    #[test]
    fn failed_parse() {
        assert_eq!(Embed::parse("{{youtube}}", 0, None), None);
    }
}
