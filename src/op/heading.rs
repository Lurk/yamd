use crate::{
    is, join,
    lexer::TokenKind,
    op::{Op, Parser, anchor::anchor, op::Node, parser::Query},
};

pub fn heading<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start_token = p.chain(
        &join!(
            is!(t = TokenKind::Hash, c = 0, maxl = 6,),
            is!(t = TokenKind::Space, el = 1,)
        ),
        false,
    )?;

    let mut ops = vec![Op::new_start(Node::Heading, Vec::from_iter(start_token))];

    let mut text = Op::new_value(vec![]);
    while let Some((i, token)) = p.peek() {
        if p.chain(eof, true).is_none() {
            break;
        } else if let Some(nested_ops) = anchor(p, eof) {
            if !text.tokens.is_empty() {
                ops.push(text);
                text = Op::new_value(vec![]);
            }
            ops.extend(nested_ops);
        } else {
            text.tokens.push(token);
            p.replace_position(i + 1);
        }
    }
    if !text.tokens.is_empty() {
        ops.push(text);
    }
    ops.push(Op::new_end(Node::Heading, vec![]));

    Some(ops)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{
            Op,
            heading::heading,
            op::Node,
            parser::{Condition, Query},
        },
    };

    #[test]
    fn happy_path() {
        let p = "## heading [a](u) text".into();
        assert_eq!(
            heading(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Heading, vec![p.get(0).unwrap(), p.get(1).unwrap()],),
                Op::new_value(vec![p.get(2).unwrap()]),
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, vec![p.get(3).unwrap()]),
                Op::new_value(vec![p.get(4).unwrap()]),
                Op::new_end(Node::Title, vec![p.get(5).unwrap()]),
                Op::new_start(Node::Destination, vec![p.get(6).unwrap()]),
                Op::new_value(vec![p.get(7).unwrap()]),
                Op::new_end(Node::Destination, vec![p.get(8).unwrap()]),
                Op::new_end(Node::Anchor, vec![]),
                Op::new_value(vec![p.get(9).unwrap(), p.get(10).unwrap(),]),
                Op::new_end(Node::Heading, vec![]),
            ])
        );
    }

    #[test]
    fn start_with_anchor() {
        let p = "## [a](u) heading".into();
        assert_eq!(
            heading(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Heading, vec![p.get(0).unwrap(), p.get(1).unwrap()],),
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, vec![p.get(2).unwrap()]),
                Op::new_value(vec![p.get(3).unwrap()]),
                Op::new_end(Node::Title, vec![p.get(4).unwrap()]),
                Op::new_start(Node::Destination, vec![p.get(5).unwrap()]),
                Op::new_value(vec![p.get(6).unwrap()]),
                Op::new_end(Node::Destination, vec![p.get(7).unwrap()]),
                Op::new_end(Node::Anchor, vec![]),
                Op::new_value(vec![p.get(8).unwrap(), p.get(9).unwrap(),]),
                Op::new_end(Node::Heading, vec![]),
            ])
        );
    }

    #[test]
    fn broken_anchor() {
        let p = "## heading [a](u text".into();
        assert_eq!(
            heading(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Heading, vec![p.get(0).unwrap(), p.get(1).unwrap()],),
                Op::new_value(Vec::from_iter(p.slice(2..8))),
                Op::new_end(Node::Heading, vec![]),
            ])
        );
    }

    #[test]
    fn with_terminator() {
        let p = "## heading\n\ntext".into();
        assert_eq!(
            heading(&p, &Query::Is(Condition::new().kind(TokenKind::Terminator))),
            Some(vec![
                Op::new_start(Node::Heading, vec![p.get(0).unwrap(), p.get(1).unwrap()],),
                Op::new_value(vec![p.get(2).unwrap()]),
                Op::new_end(Node::Heading, vec![]),
            ])
        );
    }

    #[test]
    fn have_no_space_before_text() {
        let p = "##heading\n\ntext".into();
        assert_eq!(heading(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Hash, 0..2, Position::default()),))
        );
    }

    #[test]
    fn new_line_check() {
        let p = "## heading [a](u) text\n ".into();
        assert_eq!(
            heading(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Heading, vec![p.get(0).unwrap(), p.get(1).unwrap()],),
                Op::new_value(vec![p.get(2).unwrap()]),
                Op::new_start(Node::Anchor, vec![]),
                Op::new_start(Node::Title, vec![p.get(3).unwrap()]),
                Op::new_value(vec![p.get(4).unwrap()]),
                Op::new_end(Node::Title, vec![p.get(5).unwrap()]),
                Op::new_start(Node::Destination, vec![p.get(6).unwrap()]),
                Op::new_value(vec![p.get(7).unwrap()]),
                Op::new_end(Node::Destination, vec![p.get(8).unwrap()]),
                Op::new_end(Node::Anchor, vec![]),
                Op::new_value(Vec::from_iter(p.slice(9..13))),
                Op::new_end(Node::Heading, vec![]),
            ])
        );
    }

    #[test]
    fn only_one_token() {
        let p = "##".into();
        assert_eq!(heading(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Hash, 0..2, Position::default()),))
        );
    }
}
