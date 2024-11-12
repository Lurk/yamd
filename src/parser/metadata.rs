use crate::lexer::TokenKind;

use super::Parser;

pub(crate) fn metadata(p: &mut Parser) -> Option<String> {
    let start = p.pos();

    p.next_token();

    if let Some(t) = p.next_token() {
        if t.kind == TokenKind::Eol {
            while let Some((t, pos)) = p.peek() {
                match t.kind {
                    TokenKind::Minus if t.slice.len() == 3 && t.position.column == 0 => {
                        p.next_token();
                        return Some(p.range_to_string(start + 2..pos - 1));
                    }
                    _ => {
                        p.next_token();
                    }
                }
            }
        }
    }

    p.backtrack(start);
    p.flip_to_literal_at(start);

    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        parser::{metadata, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("---\ncan contain any\n\ntoken\n\n---");
        assert_eq!(
            metadata(&mut p),
            Some(String::from("can contain any\n\ntoken"))
        );
    }

    #[test]
    fn no_closing_token() {
        let mut p = Parser::new("---\n");
        assert_eq!(metadata(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "---", Position::default()),
                0
            ))
        )
    }

    #[test]
    fn no_eol() {
        let mut p = Parser::new("--- t\n---");
        assert_eq!(metadata(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "---", Position::default()),
                0
            ))
        )
    }

    #[test]
    fn only_one_token() {
        let mut p = Parser::new("---");
        assert_eq!(metadata(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "---", Position::default()),
                0
            ))
        )
    }
}
