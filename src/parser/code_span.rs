use crate::{lexer::TokenKind, nodes::CodeSpan};

use super::Parser;

pub(crate) fn code_span(p: &mut Parser) -> Option<CodeSpan> {
    p.advance_until(|t| t.kind == TokenKind::Backtick && t.slice.len() == 1)
        .map(|(start, end)| CodeSpan::new(p.range_to_string(start + 1..end)))
}

#[cfg(test)]
mod tests {

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::CodeSpan,
        parser::{code_span, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("`happy path`");
        assert_eq!(code_span(&mut p), Some(CodeSpan::new("happy path")));
    }

    #[test]
    fn terminator() {
        let mut p = Parser::new("`happy\n\npath`");
        assert_eq!(code_span(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "`", Position::default()), 0))
        )
    }

    #[test]
    fn no_rhs() {
        let mut p = Parser::new("`happy path");
        assert_eq!(code_span(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "`", Position::default()), 0))
        )
    }
}
