use crate::{
    is, join,
    lexer::TokenKind,
    op::{
        op::{Node, Op},
        parser::{Parser, Query},
    },
};

pub fn emphasis<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let start_token = p.chain(&join!(is!(t = TokenKind::Star, el = 1,)), false)?;
    let Some((body, end_token)) = p.advance_until(&join!(is!(t = TokenKind::Star, el = 1,)), eof)
    else {
        p.replace_position(start);
        return None;
    };

    let ops = vec![
        Op::new_start(Node::Emphasis, Vec::from_iter(start_token)),
        Op::new_value(Vec::from_iter(body)),
        Op::new_end(Node::Emphasis, Vec::from_iter(end_token)),
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
            emphasis::emphasis,
            op::Node,
            parser::{Condition, Query},
        },
    };

    #[test]
    fn happy_path() {
        let p: Parser = "*happy*".into();
        assert_eq!(
            emphasis(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Emphasis, vec![p.get(0).unwrap()]),
                Op::new_value(vec![p.get(1).unwrap(),]),
                Op::new_end(Node::Emphasis, vec![p.get(2).unwrap()]),
            ])
        );
    }

    #[test]
    fn no_closing_token() {
        let p: Parser = "*happy".into();
        assert_eq!(emphasis(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, "*", Position::default())))
        )
    }

    #[test]
    fn terminator() {
        let p: Parser = "*ha\n\nppy*".into();
        assert_eq!(
            emphasis(&p, &Query::Is(Condition::new().kind(TokenKind::Terminator))),
            None
        );
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, "*", Position::default())))
        );
    }
}
