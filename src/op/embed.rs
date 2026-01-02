use crate::{
    is, join,
    lexer::TokenKind,
    op::{Op, Parser, op::Node, parser::Query},
};

pub fn embed<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let start_token = p.chain(
        &join!(is!(t = TokenKind::LeftCurlyBrace, c = 0, el = 2,)),
        false,
    )?;

    let Some((lhs, separator_token)) =
        p.advance_until(&join!(is!(t = TokenKind::Pipe, el = 1,)), eof)
    else {
        p.replace_position(start);
        return None;
    };

    let Some((rhs, end_token)) =
        p.advance_until(&join!(is!(t = TokenKind::RightCurlyBrace, el = 2,)), eof)
    else {
        p.replace_position(start);
        return None;
    };

    if p.chain(eof, true).is_some() {
        p.replace_position(start);
        return None;
    }

    Some(vec![
        Op::new_start(Node::Embed, Vec::from_iter(start_token)),
        Op::new_value(Vec::from_iter(lhs)),
        Op::new_value(Vec::from_iter(separator_token)),
        Op::new_value(Vec::from_iter(rhs)),
        Op::new_end(Node::Embed, Vec::from_iter(end_token)),
    ])
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{
            Op,
            embed::embed,
            op::Node,
            parser::{Condition, Query},
        },
    };

    #[test]
    fn happy_path() {
        let p = "{{happy|path}}".into();
        assert_eq!(
            embed(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Embed, vec![p.get(0).unwrap()]), // {{
                Op::new_value(vec![p.get(1).unwrap()]),              // happy
                Op::new_value(vec![p.get(2).unwrap()]),              // |
                Op::new_value(vec![p.get(3).unwrap()]),              // path
                Op::new_end(Node::Embed, vec![p.get(4).unwrap()]),   // }}
            ])
        );
    }

    #[test]
    fn terminator() {
        let p = "{{\n\n|path}}".into();
        assert_eq!(
            embed(&p, &Query::Is(Condition::new().kind(TokenKind::Terminator))),
            None
        );
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftCurlyBrace, "{{", Position::default()),
            ))
        )
    }

    #[test]
    fn do_not_have_closing_token() {
        let p = "{{happy|path}".into();
        assert_eq!(embed(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftCurlyBrace, "{{", Position::default()),
            ))
        );
    }

    #[test]
    fn no_pipe() {
        let p = "{{happy}}".into();
        assert_eq!(embed(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::LeftCurlyBrace, "{{", Position::default()),
            ))
        )
    }
}
