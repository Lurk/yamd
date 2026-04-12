use crate::op::{Content, Node, Op, Parser, destination::destination, title::title};

pub fn anchor(p: &mut Parser) -> bool {
    let start = p.pos;
    let snap = p.ops.len();
    p.ops.push(Op::new_start(Node::Anchor, Content::Span(0..0)));
    if !title(p) || !destination(p) {
        p.pos = start;
        p.ops.truncate(snap);
        return false;
    }
    p.ops.push(Op::new_end(Node::Anchor, Content::Span(0..0)));
    true
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Content, Node, Op, Parser, anchor::anchor, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "[a](u)".into();
        assert!(anchor(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Anchor, Content::Span(0..0)),
                Op::new_start(Node::Title, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_end(Node::Title, p.span(2..3)),
                Op::new_start(Node::Destination, p.span(3..4)),
                Op::new_value(p.span(4..5)),
                Op::new_end(Node::Destination, p.span(5..6)),
                Op::new_end(Node::Anchor, Content::Span(0..0))
            ]
        );
    }

    #[test]
    fn title_is_not_closed() {
        let mut p: Parser = "[a".into();
        assert!(!anchor(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, 0..1, Position::default())
            ))
        )
    }

    #[test]
    fn url_is_not_closed() {
        let mut p: Parser = "[a](u".into();
        assert!(!anchor(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, 0..1, Position::default())
            ))
        )
    }

    #[test]
    fn has_nested_square_brackets() {
        let mut p: Parser = "[[a\\]l](u)".into();
        assert!(anchor(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Anchor, Content::Span(0..0)),
                Op::new_start(Node::Title, p.span(0..1)),
                Op::new_value(p.span(1..4)),
                Op::new_end(Node::Title, p.span(4..5)),
                Op::new_start(Node::Destination, p.span(5..6)),
                Op::new_value(p.span(6..7)),
                Op::new_end(Node::Destination, p.span(7..8)),
                Op::new_end(Node::Anchor, Content::Span(0..0))
            ]
        );
    }

    #[test]
    fn has_nested_paren() {
        let mut p: Parser = "[a]((u)r)".into();
        assert!(anchor(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Anchor, Content::Span(0..0)),
                Op::new_start(Node::Title, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_end(Node::Title, p.span(2..3)),
                Op::new_start(Node::Destination, p.span(3..4)),
                Op::new_value(p.span(4..8)),
                Op::new_end(Node::Destination, p.span(8..9)),
                Op::new_end(Node::Anchor, Content::Span(0..0))
            ]
        );
    }

    #[test]
    fn has_unclosed_nested_paren() {
        let mut p: Parser = "[a]((ur)t".into();
        assert!(anchor(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Anchor, Content::Span(0..0)),
                Op::new_start(Node::Title, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_end(Node::Title, p.span(2..3)),
                Op::new_start(Node::Destination, p.span(3..4)),
                Op::new_value(p.span(4..6)),
                Op::new_end(Node::Destination, p.span(6..7)),
                Op::new_end(Node::Anchor, Content::Span(0..0))
            ]
        );
    }

    #[test]
    fn has_terminator() {
        let mut p: Parser = "[[a]((u\n\n)r)".into();
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(!anchor(p));
            assert!(p.ops.is_empty());
            assert_eq!(
                p.peek(),
                Some((
                    0,
                    &Token::new(TokenKind::LeftSquareBracket, 0..1, Position::default())
                ))
            );
        });
    }

    #[test]
    fn no_paren() {
        let mut p: Parser = "[a]".into();
        assert!(!anchor(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, 0..1, Position::default())
            ))
        )
    }

    #[test]
    fn right_paren() {
        let mut p: Parser = "[a])".into();
        assert!(!anchor(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, 0..1, Position::default())
            ))
        )
    }
}
