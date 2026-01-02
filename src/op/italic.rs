use crate::{
    is, join,
    lexer::TokenKind,
    op::{
        op::{Node, Op, OpKind},
        parser::{Parser, Query},
    },
};

pub fn italic<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let start_token = p.chain(&join!(is!(t = TokenKind::Underscore, el = 1,)), false)?;
    let Some((body, end_token)) =
        p.advance_until(&join!(is!(t = TokenKind::Underscore, el = 1,)), eof)
    else {
        p.replace_position(start);
        return None;
    };
    let ops = vec![
        Op {
            kind: OpKind::Start(Node::Italic),
            tokens: Vec::from_iter(start_token),
        },
        Op {
            kind: OpKind::Value,
            tokens: Vec::from_iter(body),
        },
        Op {
            kind: OpKind::End(Node::Italic),
            tokens: Vec::from_iter(end_token),
        },
    ];
    Some(ops)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{
            Op, Parser,
            italic::italic,
            op::Node,
            parser::{Condition, Query},
        },
    };

    #[test]
    fn happy_path() {
        let p: Parser = "_happy_".into();
        assert_eq!(
            italic(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Italic, vec![p.get(0).unwrap()]),
                Op::new_value(vec![p.get(1).unwrap(),]),
                Op::new_end(Node::Italic, vec![p.get(2).unwrap()])
            ])
        );
    }

    #[test]
    fn no_closing_token() {
        let p: Parser = "_happy".into();
        assert_eq!(italic(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Underscore, "_", Position::default()),
            ))
        )
    }

    #[test]
    fn terminator() {
        let p: Parser = "_ha\n\nppy_".into();
        assert_eq!(
            italic(&p, &Query::Is(Condition::new().kind(TokenKind::Terminator))),
            None
        );
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Underscore, "_", Position::default())
            ))
        );
    }
}
