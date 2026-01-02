use crate::{
    is, join,
    lexer::TokenKind,
    op::{Op, Parser, op::Node, parser::Query},
};

pub fn code_span<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let query = join!(is!(t = TokenKind::Backtick, el = 1,));
    let start_token = p.chain(&query, false)?;
    let Some((body, end_token)) = p.advance_until(&query, eof) else {
        p.replace_position(start);
        return None;
    };

    let ops = vec![
        Op::new_start(Node::CodeSpan, Vec::from_iter(start_token)),
        Op::new_value(Vec::from_iter(body)),
        Op::new_end(Node::CodeSpan, Vec::from_iter(end_token)),
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
            code_span::code_span,
            op::Node,
            parser::{Condition, Query},
        },
    };

    #[test]
    fn happy_path() {
        let p: Parser = "`happy path`".into();
        assert_eq!(
            code_span(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::CodeSpan, vec![p.get(0).unwrap()]),
                Op::new_value(vec![p.get(1).unwrap(),]),
                Op::new_end(Node::CodeSpan, vec![p.get(2).unwrap()]),
            ])
        );
    }

    #[test]
    fn terminator() {
        let p: Parser = "`happy\n\npath`".into();
        assert_eq!(
            code_span(&p, &Query::Is(Condition::new().kind(TokenKind::Terminator))),
            None
        );
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, "`", Position::default())
            ))
        )
    }

    #[test]
    fn no_rhs() {
        let p: Parser = "`happy path".into();
        assert_eq!(code_span(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, "`", Position::default())
            ))
        )
    }
}
