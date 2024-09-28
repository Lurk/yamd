use crate::{lexer::TokenKind, nodes::Strikethrough};

use super::Parser;

pub(crate) fn strikethrough(p: &mut Parser) -> Option<Strikethrough> {
    p.advance_until_terminated(|t| t.kind == TokenKind::Tilde && t.slice.len() == 2)
        .map(|(start, end)| Strikethrough::new(p.range_to_string(start + 1..end)))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::Strikethrough,
        parser::{strikethrough, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("~~happy~~");
        assert_eq!(strikethrough(&mut p), Some(Strikethrough::new("happy")));
    }

    #[test]
    fn no_closing_token() {
        let mut p = Parser::new("~~happy");
        assert_eq!(strikethrough(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "~~", Position::default()),
                0
            ))
        )
    }

    #[test]
    fn terminator() {
        let mut p = Parser::new("~~ha\n\nppy~~");
        assert_eq!(strikethrough(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "~~", Position::default()),
                0
            ))
        )
    }
}
