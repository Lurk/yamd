use crate::op::{Content, Node, Op, Parser, image::image};

pub fn images(p: &mut Parser) -> bool {
    let start = p.pos;
    let snap = p.ops.len();
    let mut count = 0;
    while p.peek().is_some() {
        if image(p) {
            count += 1;
        } else if p.at_eof() {
            break;
        } else {
            p.pos = start;
            p.ops.truncate(snap);
            return false;
        }
    }

    if count == 1 {
        return true;
    } else if count > 1 {
        p.ops
            .insert(snap, Op::new_start(Node::Images, Content::Span(0..0)));
        p.ops.push(Op::new_end(Node::Images, Content::Span(0..0)));
        return true;
    }

    p.pos = start;
    p.ops.truncate(snap);
    false
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Content, Node, Op, images::images},
    };

    #[test]
    fn happy_path() {
        let mut p = "![a](u)\n![a](u)".into();
        assert!(images(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Images, Content::Span(0..0)),
                Op::new_start(Node::Image, p.span(0..1)),
                Op::new_start(Node::Title, p.span(1..2)),
                Op::new_value(p.span(2..3)),
                Op::new_end(Node::Title, p.span(3..4)),
                Op::new_start(Node::Destination, p.span(4..5)),
                Op::new_value(p.span(5..6)),
                Op::new_end(Node::Destination, p.span(6..7)),
                Op::new_end(Node::Image, p.span(7..8)),
                Op::new_start(Node::Image, p.span(8..9)),
                Op::new_start(Node::Title, p.span(9..10)),
                Op::new_value(p.span(10..11)),
                Op::new_end(Node::Title, p.span(11..12)),
                Op::new_start(Node::Destination, p.span(12..13)),
                Op::new_value(p.span(13..14)),
                Op::new_end(Node::Destination, p.span(14..15)),
                Op::new_end(Node::Image, Content::Span(0..0)),
                Op::new_end(Node::Images, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn not_an_anchor() {
        let mut p = "![a](u)\n!!foo".into();
        assert!(!images(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..1, Position::default())))
        );
    }

    #[test]
    fn next_token_can_be_only_terminator() {
        let mut p = "![a](u)\n![a](u)fasdf".into();
        assert!(!images(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..1, Position::default())))
        );
    }

    #[test]
    fn new_line_check() {
        let mut p = "![a](u)\n![a](u)\n".into();
        assert!(images(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Images, Content::Span(0..0)),
                Op::new_start(Node::Image, p.span(0..1)),
                Op::new_start(Node::Title, p.span(1..2)),
                Op::new_value(p.span(2..3)),
                Op::new_end(Node::Title, p.span(3..4)),
                Op::new_start(Node::Destination, p.span(4..5)),
                Op::new_value(p.span(5..6)),
                Op::new_end(Node::Destination, p.span(6..7)),
                Op::new_end(Node::Image, p.span(7..8)),
                Op::new_start(Node::Image, p.span(8..9)),
                Op::new_start(Node::Title, p.span(9..10)),
                Op::new_value(p.span(10..11)),
                Op::new_end(Node::Title, p.span(11..12)),
                Op::new_start(Node::Destination, p.span(12..13)),
                Op::new_value(p.span(13..14)),
                Op::new_end(Node::Destination, p.span(14..15)),
                Op::new_end(Node::Image, p.span(15..16)),
                Op::new_end(Node::Images, Content::Span(0..0)),
            ]
        );
    }
}
