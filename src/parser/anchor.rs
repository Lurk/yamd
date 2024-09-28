use crate::{lexer::TokenKind, nodes::Anchor};

use super::Parser;

pub(crate) fn anchor(p: &mut Parser<'_>) -> Option<Anchor> {
    let start_pos = p.pos();
    let mut paren_count = 0;
    let mut last_right_paren_pos: Option<usize> = None;
    let mut right_square_bracket_pos: Option<usize> = None;
    while let Some((t, pos)) = p.peek() {
        match t.kind {
            TokenKind::Terminator => break,
            TokenKind::LeftSquareBracket if right_square_bracket_pos.is_none() => {
                if let Some((_, end)) =
                    p.advance_until_terminated(|t| t.kind == TokenKind::RightSquareBracket)
                {
                    right_square_bracket_pos.replace(end);
                } else {
                    break;
                }
            }
            TokenKind::LeftParenthesis if right_square_bracket_pos.is_some() => {
                p.next_token();
                paren_count += 1;
            }
            TokenKind::RightParenthesis if right_square_bracket_pos.is_some() => {
                last_right_paren_pos.replace(pos);
                p.next_token();
                paren_count -= 1;
                if paren_count == 0 {
                    return Some(Anchor::new(
                        p.range_to_string(
                            start_pos + 1..right_square_bracket_pos.expect("to be a number"),
                        ),
                        p.range_to_string(
                            right_square_bracket_pos.expect("to be a number") + 2..pos,
                        ),
                    ));
                }
            }
            _ if paren_count == 0 => {
                break;
            }
            _ => {
                p.next_token();
            }
        }
    }

    if let (Some(right_square_bracket_pos), Some(right_paren_pos)) =
        (right_square_bracket_pos, last_right_paren_pos)
    {
        return Some(Anchor::new(
            p.range_to_string(start_pos + 1..right_square_bracket_pos),
            p.range_to_string(right_square_bracket_pos + 2..right_paren_pos),
        ));
    }

    p.flip_to_literal_at(start_pos);
    p.move_to(start_pos);
    None
}

#[cfg(test)]
mod tests {

    use crate::{
        lexer::{Position, Token, TokenKind},
        nodes::Anchor,
        parser::{anchor, Parser},
    };

    #[test]
    fn happy_path() {
        let mut p = Parser::new("[a](u)");
        assert_eq!(anchor(&mut p), Some(Anchor::new("a", "u")));
    }

    #[test]
    fn alt_is_not_closed() {
        let mut p = Parser::new("[a");
        assert_eq!(anchor(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "[", Position::default()), 0))
        )
    }

    #[test]
    fn url_is_not_closed() {
        let mut p = Parser::new("[a](u");
        assert_eq!(anchor(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "[", Position::default()), 0))
        )
    }

    #[test]
    fn has_nested_square_brackets() {
        let mut p = Parser::new("[[a\\]l](u)");
        assert_eq!(anchor(&mut p), Some(Anchor::new("[a]l", "u")));
    }

    #[test]
    fn has_nested_paren() {
        let mut p = Parser::new("[a]((u)r)");
        assert_eq!(anchor(&mut p), Some(Anchor::new("a", "(u)r")));
    }

    #[test]
    fn has_unclosed_nested_paren() {
        let mut p = Parser::new("[a]((ur)t");
        assert_eq!(anchor(&mut p), Some(Anchor::new("a", "(ur")));
    }

    #[test]
    fn has_terminator() {
        let mut p = Parser::new("[[a]((u\n\n)r)");
        assert_eq!(anchor(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "[", Position::default()), 0))
        )
    }

    #[test]
    fn no_paren() {
        let mut p = Parser::new("[a]");
        assert_eq!(anchor(&mut p), None);
        assert_eq!(
            p.peek(),
            Some((&Token::new(TokenKind::Literal, "[", Position::default()), 0))
        )
    }
}
