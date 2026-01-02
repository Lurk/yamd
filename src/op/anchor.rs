use crate::op::{
    Op, Parser,
    destination::destination,
    op::{Node, OpKind},
    parser::Query,
    title::title,
};

pub fn anchor<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let title_ops = title(p, eof)?;
    let Some(destination_ops) = destination(p, eof) else {
        p.replace_position(start);
        return None;
    };
    let mut ops = vec![Op {
        kind: OpKind::Start(Node::Anchor),
        tokens: vec![],
    }];
    ops.extend(title_ops);
    ops.extend(destination_ops);
    ops.push(Op {
        kind: OpKind::End(Node::Anchor),
        tokens: vec![],
    });
    Some(ops)
}

#[cfg(test)]
mod tests {
    use crate::{
        is,
        lexer::{Position, Token, TokenKind},
        op::{Op, Parser, anchor::anchor, op::Node, parser::Query},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn happy_path() {
        let p: Parser = "[a](u)".into();
        assert_eq!(
            anchor(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, Vec::from_iter(p.slice(0..1))),
                Op::new_value(Vec::from_iter(p.slice(1..2))),
                Op::new_end(Node::Title, Vec::from_iter(p.slice(2..3))),
                Op::new_start(Node::Destination, Vec::from_iter(p.slice(3..4))),
                Op::new_value(Vec::from_iter(p.slice(4..5))),
                Op::new_end(Node::Destination, Vec::from_iter(p.slice(5..6))),
                Op::new_end(Node::Anchor, vec![])
            ])
        );
    }

    #[test]
    fn title_is_not_closed() {
        let p: Parser = "[a".into();
        assert_eq!(anchor(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, "[", Position::default())
            ))
        )
    }

    #[test]
    fn url_is_not_closed() {
        let p: Parser = "[a](u".into();
        assert_eq!(anchor(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, "[", Position::default())
            ))
        )
    }

    #[test]
    fn has_nested_square_brackets() {
        let p: Parser = "[[a\\]l](u)".into();
        assert_eq!(
            anchor(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, Vec::from_iter(p.slice(0..1))),
                Op::new_value(Vec::from_iter(p.slice(1..4))),
                Op::new_end(Node::Title, Vec::from_iter(p.slice(4..5))),
                Op::new_start(Node::Destination, Vec::from_iter(p.slice(5..6))),
                Op::new_value(Vec::from_iter(p.slice(6..7))),
                Op::new_end(Node::Destination, Vec::from_iter(p.slice(7..8))),
                Op::new_end(Node::Anchor, vec![])
            ])
        );
    }

    #[test]
    fn has_nested_paren() {
        let p: Parser = "[a]((u)r)".into();
        assert_eq!(
            anchor(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, Vec::from_iter(p.slice(0..1))),
                Op::new_value(Vec::from_iter(p.slice(1..2))),
                Op::new_end(Node::Title, Vec::from_iter(p.slice(2..3))),
                Op::new_start(Node::Destination, Vec::from_iter(p.slice(3..4))),
                Op::new_value(Vec::from_iter(p.slice(4..8))),
                Op::new_end(Node::Destination, Vec::from_iter(p.slice(8..9))),
                Op::new_end(Node::Anchor, vec![])
            ])
        );
    }

    #[test]
    fn has_unclosed_nested_paren() {
        let p: Parser = "[a]((ur)t".into();
        assert_eq!(
            anchor(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, Vec::from_iter(p.slice(0..1))),
                Op::new_value(Vec::from_iter(p.slice(1..2))),
                Op::new_end(Node::Title, Vec::from_iter(p.slice(2..3))),
                Op::new_start(Node::Destination, Vec::from_iter(p.slice(3..4))),
                Op::new_value(Vec::from_iter(p.slice(4..6))),
                Op::new_end(Node::Destination, Vec::from_iter(p.slice(6..7))),
                Op::new_end(Node::Anchor, vec![])
            ])
        );
    }

    #[test]
    fn has_terminator() {
        let p: Parser = "[[a]((u\n\n)r)".into();
        assert_eq!(anchor(&p, &is!(t = TokenKind::Terminator,)), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, "[", Position::default())
            ))
        )
    }

    #[test]
    fn no_paren() {
        let p: Parser = "[a]".into();
        assert_eq!(anchor(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, "[", Position::default())
            ))
        )
    }

    #[test]
    fn right_paren() {
        let p = "[a])".into();
        assert_eq!(anchor(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, "[", Position::default())
            ))
        )
    }
}
