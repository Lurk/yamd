use crate::{lexer::TokenKind, nodes::Emphasis};

use super::Parser;

pub(crate) fn emphasis(p: &mut Parser) -> Option<Emphasis> {
    p.advance_or_backtrack(|t| t.kind == TokenKind::Star && t.slice.len() == 1)
        .map(|(start, end)| Emphasis::new(p.range_to_string(start + 1..end)))
}

#[cfg(test)]
mod tests {

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::Emphasis,
        parser::{emphasis, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("*happy*");
        assert_eq!(emphasis(&mut p), Some(Emphasis::new("happy")));
    }

    #[test]
    fn no_closing_token() {
        let mut p = Parser::new("*happy");
        assert_eq!(emphasis(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "*", Position::default()), 0))
        )
    }

    #[test]
    fn terminator() {
        let mut p = Parser::new("*ha\n\nppy*");
        assert_eq!(emphasis(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "*", Position::default()), 0))
        );
    }
}
