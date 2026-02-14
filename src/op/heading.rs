use crate::{
    eat_seq,
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser, anchor::anchor},
};

fn is_hash(t: &Token) -> bool {
    t.kind == TokenKind::Hash && t.position.column == 0 && t.range.len() <= 6
}

fn is_space(t: &Token) -> bool {
    t.kind == TokenKind::Space && t.range.len() == 1
}

pub fn heading(p: &Parser) -> Option<Vec<Op>> {
    let start_token = eat_seq!(p, is_hash, is_space)?;

    let mut ops = vec![Op::new_start(Node::Heading, start_token)];

    let mut text_start: Option<usize> = None;
    while let Some((pos, _)) = p.peek() {
        if p.at_eof() {
            break;
        } else if let Some(nested_ops) = anchor(p) {
            if let Some(s) = text_start {
                ops.push(Op::new_value(p.slice(s..pos)));
                text_start = None;
            }
            ops.extend(nested_ops);
        } else {
            text_start.get_or_insert(pos);
            p.next();
        }
    }
    if let Some(s) = text_start {
        ops.push(Op::new_value(p.slice(s..p.pos())));
    }
    ops.push(Op::new_end(Node::Heading, &[]));

    Some(ops)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, heading::heading, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let p = "## heading [a](u) text".into();
        assert_eq!(
            heading(&p),
            Some(vec![
                Op::new_start(Node::Heading, p.slice(0..2)),
                Op::new_value(p.slice(2..3)),
                Op::new_start(Node::Anchor, &[]),
                Op::new_start(Node::Title, p.slice(3..4)),
                Op::new_value(p.slice(4..5)),
                Op::new_end(Node::Title, p.slice(5..6)),
                Op::new_start(Node::Destination, p.slice(6..7)),
                Op::new_value(p.slice(7..8)),
                Op::new_end(Node::Destination, p.slice(8..9)),
                Op::new_end(Node::Anchor, &[]),
                Op::new_value(p.slice(9..11)),
                Op::new_end(Node::Heading, &[]),
            ])
        );
    }

    #[test]
    fn start_with_anchor() {
        let p = "## [a](u) heading".into();
        assert_eq!(
            heading(&p),
            Some(vec![
                Op::new_start(Node::Heading, p.slice(0..2)),
                Op::new_start(Node::Anchor, &[]),
                Op::new_start(Node::Title, p.slice(2..3)),
                Op::new_value(p.slice(3..4)),
                Op::new_end(Node::Title, p.slice(4..5)),
                Op::new_start(Node::Destination, p.slice(5..6)),
                Op::new_value(p.slice(6..7)),
                Op::new_end(Node::Destination, p.slice(7..8)),
                Op::new_end(Node::Anchor, &[]),
                Op::new_value(p.slice(8..10)),
                Op::new_end(Node::Heading, &[]),
            ])
        );
    }

    #[test]
    fn broken_anchor() {
        let p = "## heading [a](u text".into();
        assert_eq!(
            heading(&p),
            Some(vec![
                Op::new_start(Node::Heading, p.slice(0..2)),
                Op::new_value(p.slice(2..8)),
                Op::new_end(Node::Heading, &[]),
            ])
        );
    }

    #[test]
    fn with_terminator() {
        let p: Parser = "## heading\n\ntext".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(
            heading(&p),
            Some(vec![
                Op::new_start(Node::Heading, p.slice(0..2)),
                Op::new_value(p.slice(2..3)),
                Op::new_end(Node::Heading, &[]),
            ])
        );
    }

    #[test]
    fn have_no_space_before_text() {
        let p = "##heading\n\ntext".into();
        assert_eq!(heading(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Hash, 0..2, Position::default())))
        );
    }

    #[test]
    fn new_line_check() {
        let p = "## heading [a](u) text\n ".into();
        assert_eq!(
            heading(&p),
            Some(vec![
                Op::new_start(Node::Heading, p.slice(0..2)),
                Op::new_value(p.slice(2..3)),
                Op::new_start(Node::Anchor, &[]),
                Op::new_start(Node::Title, p.slice(3..4)),
                Op::new_value(p.slice(4..5)),
                Op::new_end(Node::Title, p.slice(5..6)),
                Op::new_start(Node::Destination, p.slice(6..7)),
                Op::new_value(p.slice(7..8)),
                Op::new_end(Node::Destination, p.slice(8..9)),
                Op::new_end(Node::Anchor, &[]),
                Op::new_value(p.slice(9..13)),
                Op::new_end(Node::Heading, &[]),
            ])
        );
    }

    #[test]
    fn only_one_token() {
        let p = "##".into();
        assert_eq!(heading(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Hash, 0..2, Position::default())))
        );
    }
}
