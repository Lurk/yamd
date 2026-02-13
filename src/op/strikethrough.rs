use crate::op::parser::Parser;
use crate::{
    is, join,
    lexer::TokenKind,
    op::{
        op::{Node, Op, OpKind},
        parser::Query,
    },
};

pub fn strikethrough<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let start_token = p.chain(&join!(is!(t = TokenKind::Tilde, el = 2,)), false)?;
    let Some((body, end_token)) = p.advance_until(&join!(is!(t = TokenKind::Tilde, el = 2,)), eof)
    else {
        p.replace_position(start);
        return None;
    };
    let ops = vec![
        Op {
            kind: OpKind::Start(Node::Strikethrough),
            tokens: Vec::from_iter(start_token),
        },
        Op {
            kind: OpKind::Value,
            tokens: Vec::from_iter(body),
        },
        Op {
            kind: OpKind::End(Node::Strikethrough),
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
            Op,
            op::Node,
            parser::{Condition, Query},
            strikethrough::strikethrough,
        },
    };

    #[test]
    fn happy_path() {
        let p = "~~happy~~".into();
        assert_eq!(
            strikethrough(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Strikethrough, vec![p.get(0).unwrap()]),
                Op::new_value(vec![p.get(1).unwrap(),]),
                Op::new_end(Node::Strikethrough, vec![p.get(2).unwrap()])
            ])
        );
    }

    #[test]
    fn no_closing_token() {
        let p = "~~happy".into();
        assert_eq!(strikethrough(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Tilde, 0..2, Position::default())))
        )
    }

    #[test]
    fn terminator() {
        let p = "~~ha\n\nppy~~".into();
        assert_eq!(
            strikethrough(&p, &Query::Is(Condition::new().kind(TokenKind::Terminator))),
            None
        );
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Tilde, 0..2, Position::default()),))
        )
    }
}
