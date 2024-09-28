use crate::{lexer::TokenKind, nodes::Italic};

use super::Parser;

pub(crate) fn italic(p: &mut Parser) -> Option<Italic> {
    p.advance_until_terminated(|t| t.kind == TokenKind::Underscore && t.slice.len() == 1)
        .map(|(start, end)| Italic::new(p.range_to_string(start + 1..end)))
}

#[cfg(test)]
mod tests {

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::Italic,
        parser::{italic, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("_happy_");
        assert_eq!(italic(&mut p), Some(Italic::new("happy")));
    }

    #[test]
    fn no_closing_token() {
        let mut p = Parser::new("_happy");
        assert_eq!(italic(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "_", Position::default()), 0))
        )
    }

    #[test]
    fn terminator() {
        let mut p = Parser::new("_ha\n\nppy_");
        assert_eq!(italic(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "_", Position::default()), 0))
        );
    }
}
