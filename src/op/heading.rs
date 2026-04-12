use crate::{
    eat_seq,
    lexer::{Token, TokenKind},
    op::{Content, Node, Op, Parser, anchor::anchor},
};

fn is_hash(t: &Token) -> bool {
    t.kind == TokenKind::Hash && t.position.column == 0 && t.range.len() <= 6
}

fn is_space(t: &Token) -> bool {
    t.kind == TokenKind::Space && t.range.len() == 1
}

pub fn heading(p: &mut Parser) -> bool {
    let Some(start_range) = eat_seq!(p, is_hash, is_space) else {
        return false;
    };
    let start_content = p.span(start_range);
    p.ops.push(Op::new_start(Node::Heading, start_content));

    let mut text_start: Option<usize> = None;
    while let Some((pos, _)) = p.peek() {
        if p.at_eof() {
            break;
        } else {
            let before_pos = p.pos;
            let before_snap = p.ops.len();
            if anchor(p) {
                if let Some(s) = text_start.take() {
                    let content = p.span(s..before_pos);
                    p.ops.insert(before_snap, Op::new_value(content));
                }
            } else {
                text_start.get_or_insert(pos);
                p.next();
            }
        }
    }
    if let Some(s) = text_start {
        let content = p.span(s..p.pos);
        p.ops.push(Op::new_value(content));
    }
    p.ops.push(Op::new_end(Node::Heading, Content::Span(0..0)));

    true
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Content, Node, Op, Parser, heading::heading, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "## heading [a](u) text".into();
        assert!(heading(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Heading, p.span(0..2)),
                Op::new_value(p.span(2..3)),
                Op::new_start(Node::Anchor, Content::Span(0..0)),
                Op::new_start(Node::Title, p.span(3..4)),
                Op::new_value(p.span(4..5)),
                Op::new_end(Node::Title, p.span(5..6)),
                Op::new_start(Node::Destination, p.span(6..7)),
                Op::new_value(p.span(7..8)),
                Op::new_end(Node::Destination, p.span(8..9)),
                Op::new_end(Node::Anchor, Content::Span(0..0)),
                Op::new_value(p.span(9..11)),
                Op::new_end(Node::Heading, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn start_with_anchor() {
        let mut p: Parser = "## [a](u) heading".into();
        assert!(heading(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Heading, p.span(0..2)),
                Op::new_start(Node::Anchor, Content::Span(0..0)),
                Op::new_start(Node::Title, p.span(2..3)),
                Op::new_value(p.span(3..4)),
                Op::new_end(Node::Title, p.span(4..5)),
                Op::new_start(Node::Destination, p.span(5..6)),
                Op::new_value(p.span(6..7)),
                Op::new_end(Node::Destination, p.span(7..8)),
                Op::new_end(Node::Anchor, Content::Span(0..0)),
                Op::new_value(p.span(8..10)),
                Op::new_end(Node::Heading, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn broken_anchor() {
        let mut p: Parser = "## heading [a](u text".into();
        assert!(heading(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Heading, p.span(0..2)),
                Op::new_value(p.span(2..8)),
                Op::new_end(Node::Heading, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn with_terminator() {
        let mut p: Parser = "## heading\n\ntext".into();
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(heading(p));
            assert_eq!(
                p.ops,
                vec![
                    Op::new_start(Node::Heading, p.span(0..2)),
                    Op::new_value(p.span(2..3)),
                    Op::new_end(Node::Heading, Content::Span(0..0)),
                ]
            );
        });
    }

    #[test]
    fn have_no_space_before_text() {
        let mut p: Parser = "##heading\n\ntext".into();
        assert!(!heading(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Hash, 0..2, Position::default())))
        );
    }

    #[test]
    fn new_line_check() {
        let mut p: Parser = "## heading [a](u) text\n ".into();
        assert!(heading(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Heading, p.span(0..2)),
                Op::new_value(p.span(2..3)),
                Op::new_start(Node::Anchor, Content::Span(0..0)),
                Op::new_start(Node::Title, p.span(3..4)),
                Op::new_value(p.span(4..5)),
                Op::new_end(Node::Title, p.span(5..6)),
                Op::new_start(Node::Destination, p.span(6..7)),
                Op::new_value(p.span(7..8)),
                Op::new_end(Node::Destination, p.span(8..9)),
                Op::new_end(Node::Anchor, Content::Span(0..0)),
                Op::new_value(p.span(9..13)),
                Op::new_end(Node::Heading, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn only_one_token() {
        let mut p: Parser = "##".into();
        assert!(!heading(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Hash, 0..2, Position::default())))
        );
    }
}
