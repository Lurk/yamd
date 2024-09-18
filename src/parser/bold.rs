use crate::{lexer::TokenKind, nodes::Bold};

use super::{italic, strikethrough, Parser};

pub(crate) fn bold(p: &mut Parser<'_>) -> Option<Bold> {
    let mut b = Bold::new(vec![]);
    let mut text_start: Option<usize> = None;
    let start_pos = p.pos();
    p.next_token();

    while let Some((t, pos)) = p.peek() {
        match t.kind {
            TokenKind::Terminator => break,
            TokenKind::Tilde if t.slice.len() == 2 => {
                if let Some(s) = strikethrough(p) {
                    if let Some(start) = text_start.take() {
                        b.body.push(p.range_to_string(start..pos).into());
                    }
                    b.body.push(s.into());
                }
            }
            TokenKind::Underscore if t.slice.len() == 1 => {
                if let Some(i) = italic(p) {
                    if let Some(start) = text_start.take() {
                        b.body.push(p.range_to_string(start..pos).into());
                    }
                    b.body.push(i.into());
                }
            }
            TokenKind::Star if t.slice.len() == 2 => {
                if let Some(start) = text_start.take() {
                    b.body.push(p.range_to_string(start..pos).into());
                }

                p.next_token();
                return Some(b);
            }
            _ => {
                text_start.get_or_insert(pos);
                p.next_token();
            }
        }
    }

    p.move_to(start_pos);
    p.flip_to_literal_at(start_pos);
    None
}

#[cfg(test)]
mod tests {

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::{Bold, Italic, Strikethrough},
        parser::{bold, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("**~~happy~~ _path_**");
        assert_eq!(
            bold(&mut p),
            Some(Bold::new(vec![
                Strikethrough::new("happy").into(),
                String::from(" ").into(),
                Italic::new("path").into()
            ]))
        );
    }

    #[test]
    fn terminator() {
        let mut p = Parser::new("**~~happy~~ _path_\n\n**");
        assert_eq!(bold(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "**", Position::default()),
                0
            ))
        )
    }

    #[test]
    fn end_of_input() {
        let mut p = Parser::new("**~~happy~~ _path_");
        assert_eq!(bold(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "**", Position::default()),
                0
            ))
        )
    }

    #[test]
    fn end_of_input_in_nested() {
        let mut p = Parser::new("**~~happy _path_");
        assert_eq!(bold(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((
                &Token::new(TokenKind::Literal, "**", Position::default()),
                0
            ))
        );
    }
}
