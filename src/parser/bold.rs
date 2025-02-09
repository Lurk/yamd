use crate::{lexer::TokenKind, nodes::Bold};

use super::{italic, strikethrough, BranchBuilder, Parser};

pub(crate) fn bold(p: &mut Parser<'_>) -> Option<Bold> {
    let mut b = BranchBuilder::new();
    let start_pos = p.pos();
    p.next_token();

    while let Some((t, pos)) = p.peek() {
        match t.kind {
            TokenKind::Terminator => break,
            TokenKind::Tilde if t.slice.len() == 2 => b.push(strikethrough(p), p, pos),
            TokenKind::Underscore if t.slice.len() == 1 => b.push(italic(p), p, pos),
            TokenKind::Star if t.slice.len() == 2 => {
                b.consume_text(p, pos);
                p.next_token();
                return b.build();
            }
            _ => {
                b.start_text(pos);
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

    #[test]
    fn text_before_strikethrough() {
        let mut p = Parser::new("**text ~~happy~~ _path_**");
        assert_eq!(
            bold(&mut p),
            Some(Bold::new(vec![
                String::from("text ").into(),
                Strikethrough::new("happy").into(),
                String::from(" ").into(),
                Italic::new("path").into()
            ]))
        );
    }

    #[test]
    fn unclosed_italic() {
        let mut p = Parser::new("**_path**");
        assert_eq!(
            bold(&mut p),
            Some(Bold::new(vec![String::from("_path").into()]))
        );
    }
}
