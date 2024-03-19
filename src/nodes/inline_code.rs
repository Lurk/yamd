use std::fmt::Display;

use serde::Serialize;

use crate::toolkit::{context::Context, parser::Parse};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct InlineCode {
    pub text: String,
}

impl InlineCode {
    pub fn new<S: Into<String>>(text: S) -> Self {
        InlineCode { text: text.into() }
    }
}

impl Display for InlineCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.text)
    }
}

impl Parse for InlineCode {
    fn parse(input: &str, current_position: usize, _: Option<&Context>) -> Option<(Self, usize)> {
        if input[current_position..].starts_with('`') {
            if let Some(end) = input[current_position + 1..].find('`') {
                return Some((
                    InlineCode::new(&input[current_position + 1..current_position + end + 1]),
                    end + 2 - current_position,
                ));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::parser::Parse;

    use super::InlineCode;
    use pretty_assertions::assert_eq;

    #[test]
    fn to_string() {
        let inline_code: String = InlineCode::new("const bar = 'baz'").to_string();
        assert_eq!(inline_code, "`const bar = 'baz'`".to_string())
    }

    #[test]
    fn from_string() {
        assert_eq!(
            InlineCode::parse("`1`", 0, None),
            Some((InlineCode::new('1'), 3))
        );
        assert_eq!(
            InlineCode::parse("`const \nfoo='bar'`", 0, None),
            Some((InlineCode::new("const \nfoo='bar'"), 18))
        );
        assert_eq!(InlineCode::parse("`a", 0, None), None);
    }
}
