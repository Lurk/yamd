use crate::op::{Node, Op, Parser, destination::destination, title::title};

pub fn anchor(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let title_ops = title(p)?;
    let Some(destination_ops) = destination(p) else {
        p.replace_position(start);
        return None;
    };
    let mut ops = vec![Op::new_start(Node::Anchor, &[])];
    ops.extend(title_ops);
    ops.extend(destination_ops);
    ops.push(Op::new_end(Node::Anchor, &[]));
    Some(ops)
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, anchor::anchor, parser::StopCondition},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn happy_path() {
        let p: Parser = "[a](u)".into();
        assert_eq!(
            anchor(&p),
            Some(vec![
                Op::new_start(Node::Anchor, &[]),
                Op::new_start(Node::Title, p.slice(0..1)),
                Op::new_value(p.slice(1..2)),
                Op::new_end(Node::Title, p.slice(2..3)),
                Op::new_start(Node::Destination, p.slice(3..4)),
                Op::new_value(p.slice(4..5)),
                Op::new_end(Node::Destination, p.slice(5..6)),
                Op::new_end(Node::Anchor, &[])
            ])
        );
    }

    #[test]
    fn title_is_not_closed() {
        let p: Parser = "[a".into();
        assert_eq!(anchor(&p), None);
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
        let p: Parser = "[a](u".into();
        assert_eq!(anchor(&p), None);
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
        let p: Parser = "[[a\\]l](u)".into();
        assert_eq!(
            anchor(&p),
            Some(vec![
                Op::new_start(Node::Anchor, &[]),
                Op::new_start(Node::Title, p.slice(0..1)),
                Op::new_value(p.slice(1..4)),
                Op::new_end(Node::Title, p.slice(4..5)),
                Op::new_start(Node::Destination, p.slice(5..6)),
                Op::new_value(p.slice(6..7)),
                Op::new_end(Node::Destination, p.slice(7..8)),
                Op::new_end(Node::Anchor, &[])
            ])
        );
    }

    #[test]
    fn has_nested_paren() {
        let p: Parser = "[a]((u)r)".into();
        assert_eq!(
            anchor(&p),
            Some(vec![
                Op::new_start(Node::Anchor, &[]),
                Op::new_start(Node::Title, p.slice(0..1)),
                Op::new_value(p.slice(1..2)),
                Op::new_end(Node::Title, p.slice(2..3)),
                Op::new_start(Node::Destination, p.slice(3..4)),
                Op::new_value(p.slice(4..8)),
                Op::new_end(Node::Destination, p.slice(8..9)),
                Op::new_end(Node::Anchor, &[])
            ])
        );
    }

    #[test]
    fn has_unclosed_nested_paren() {
        let p: Parser = "[a]((ur)t".into();
        assert_eq!(
            anchor(&p),
            Some(vec![
                Op::new_start(Node::Anchor, &[]),
                Op::new_start(Node::Title, p.slice(0..1)),
                Op::new_value(p.slice(1..2)),
                Op::new_end(Node::Title, p.slice(2..3)),
                Op::new_start(Node::Destination, p.slice(3..4)),
                Op::new_value(p.slice(4..6)),
                Op::new_end(Node::Destination, p.slice(6..7)),
                Op::new_end(Node::Anchor, &[])
            ])
        );
    }

    #[test]
    fn has_terminator() {
        let p: Parser = "[[a]((u\n\n)r)".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(anchor(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, 0..1, Position::default())
            ))
        )
    }

    #[test]
    fn no_paren() {
        let p: Parser = "[a]".into();
        assert_eq!(anchor(&p), None);
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
        let p = "[a])".into();
        assert_eq!(anchor(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftSquareBracket, 0..1, Position::default())
            ))
        )
    }
}
