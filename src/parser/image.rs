use crate::{
    lexer::{Token, TokenKind},
    nodes::{Image, Images},
};

use super::{anchor, Parser};

#[derive(PartialEq)]
enum State {
    Fail,
    Stopped,
    Idle,
}

pub(crate) fn images<Callback>(p: &mut Parser, new_line_check: Callback) -> Option<Images>
where
    Callback: Fn(&Token) -> bool,
{
    let start = p.pos();

    let mut images: Vec<Image> = vec![];
    let mut state = State::Idle;

    while let Some((t, _)) = p.peek() {
        match t.kind {
            TokenKind::Terminator => {
                break;
            }
            TokenKind::Bang => {
                state = State::Idle;
                p.next_token();
                let Some(a) = anchor(p) else {
                    state = State::Fail;
                    break;
                };

                images.push(Image::new(a.text, a.url));
            }
            TokenKind::Eol => {
                p.next_token();
            }
            _ if t.position.column == 0 && new_line_check(t) => {
                state = State::Stopped;
                break;
            }
            _ => {
                state = State::Fail;
                break;
            }
        }
    }

    if images.is_empty() || state == State::Fail {
        p.backtrack(start);
        p.flip_to_literal_at(start);
        return None;
    }

    if state != State::Stopped {
        p.next_token();
    }
    Some(Images::new(images))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::{Image, Images},
        parser::{images, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("![a](u)\n![a](u)");
        assert_eq!(
            images(&mut p, |_| false),
            Some(Images::new(vec![
                Image::new("a", "u"),
                Image::new("a", "u")
            ]))
        );
    }

    #[test]
    fn not_an_anchor() {
        let mut p = Parser::new("![a](u)\n!!foo");
        assert_eq!(images(&mut p, |_| false), None);
        assert_eq!(p.pos(), 0);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "!", Position::default()), 0))
        );
    }

    #[test]
    fn must_consume_terminator() {
        let mut p = Parser::new("![a](u)\n\n");
        images(&mut p, |_| false);
        assert_eq!(p.pos(), 8);
    }

    #[test]
    fn next_token_can_be_only_terminator() {
        let mut p = Parser::new("![a](u)\n![a](u)fasdf");
        assert_eq!(images(&mut p, |_| false), None);
        assert_eq!(p.pos(), 0);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "!", Position::default()), 0))
        );
    }

    #[test]
    fn new_line_check() {
        let mut p = Parser::new("![a](u)\n![a](u)\n ");
        assert_eq!(
            images(&mut p, |t| t.kind == TokenKind::Space),
            Some(Images::new(vec![
                Image::new("a", "u"),
                Image::new("a", "u")
            ]))
        );
    }
}
