use crate::{lexer::TokenKind, nodes::Embed};

use super::Parser;

pub(crate) fn embed(p: &mut Parser<'_>) -> Option<Embed> {
    let start_pos = p.pos();
    let mut kind: Option<usize> = None;

    p.next_token();
    while let Some((t, pos)) = p.peek() {
        match t.kind {
            TokenKind::Terminator if kind.is_none() => break,
            TokenKind::RightCurlyBrace if t.slice.len() == 2 => {
                p.next_token();
                if let Some(kind) = kind {
                    return Some(Embed::new(
                        p.range_to_string(start_pos + 1..kind),
                        p.range_to_string(kind + 1..pos),
                    ));
                }
            }
            TokenKind::Pipe if kind.is_none() => {
                kind.replace(pos);
                p.next_token();
            }
            _ => {
                p.next_token();
            }
        }
    }

    p.backtrack(start_pos);
    p.flip_to_literal_at(start_pos);
    None
}

#[cfg(test)]
mod tests {

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::Embed,
        parser::{embed, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("{{happy|path}}");
        assert_eq!(embed(&mut p), Some(Embed::new("happy", "path")));
    }

    #[test]
    fn terminator() {
        let mut p = Parser::new("{{\n\n|path}}");
        assert_eq!(embed(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "{{", Position::default()),
                0
            ))
        )
    }

    #[test]
    fn do_not_have_closing_token() {
        let mut p = Parser::new("{{happy|path}");
        assert_eq!(embed(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "{{", Position::default()),
                0
            ))
        );
    }

    #[test]
    fn no_pipe() {
        let mut p = Parser::new("{{happy}}");
        assert_eq!(embed(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "{{", Position::default()),
                0
            ))
        )
    }
}
