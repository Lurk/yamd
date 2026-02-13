use crate::{
    is, join,
    lexer::TokenKind,
    op::{
        Op, Parser,
        modifier::modifier,
        op::Node,
        parser::{Query, eol},
    },
    or,
};

const CODE_BLOCK_DELIMITER: Query = is!(t = TokenKind::Backtick, c = 0, el = 3,);

pub fn code<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start_pos = p.pos();
    let first_token = p.chain(
        &or!(
            join!(CODE_BLOCK_DELIMITER, is!(t = TokenKind::Eol,)),
            join!(CODE_BLOCK_DELIMITER)
        ),
        false,
    )?;

    let with_modifier = !first_token.last().is_some_and(eol);

    let mut ops = vec![Op::new_start(Node::Code, Vec::from_iter(first_token))];

    if with_modifier && let Some(modifier_ops) = modifier(p, eof) {
        ops.extend(modifier_ops);
    };

    let Some((body, end_token)) = p.advance_until(&join!(CODE_BLOCK_DELIMITER), eof) else {
        p.replace_position(start_pos);
        return None;
    };

    if p.chain(eof, true).is_some() {
        p.replace_position(start_pos);
        return None;
    }

    ops.push(Op::new_value(Vec::from_iter(body)));
    ops.push(Op::new_end(Node::Code, Vec::from_iter(end_token)));

    Some(ops)
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use crate::{
        is,
        lexer::{Position, Token, TokenKind},
        op::{Op, code::code, op::Node, parser::Query},
    };

    #[test]
    fn happy_path() {
        let p = "```rust\nprintln!(\"hello\");\n```".into();
        assert_eq!(
            code(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Code, vec![p.get(0).unwrap()]), // ```
                Op::new_start(Node::Modifier, vec![]),              //
                Op::new_value(vec![p.get(1).unwrap()]),             // rust
                Op::new_end(Node::Modifier, vec![p.get(2).unwrap()]), // \n
                Op::new_value(Vec::from_iter(p.slice(3..10))),      // println!(\"hello\");\n
                Op::new_end(Node::Code, vec![p.get(10).unwrap()]),  // ```
            ])
        );
    }

    #[test]
    fn eol_before_lang() {
        let p = "```\nprintln!(\"hello\");\n```".into();
        assert_eq!(
            code(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Code, Vec::from_iter(p.slice(0..2))), // ```\n
                Op::new_value(Vec::from_iter(p.slice(2..9))),             // println!(\"hello\");\n
                Op::new_end(Node::Code, vec![p.get(9).unwrap()]),         // ```
            ])
        );
    }

    #[test]
    fn terminator_before_lang() {
        let p = "```\n\nprintln!(\"hello\");\n```".into();
        assert_eq!(code(&p, &is!(t = TokenKind::Terminator,)), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, 0..3, Position::default()),
            ))
        )
    }

    #[test]
    fn do_not_have_closing_token() {
        let p = "```\nprintln!(\"hello\");\n``".into();
        assert_eq!(code(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, 0..3, Position::default()),
            ))
        );
    }

    #[test]
    fn terminator_in_the_middle_and_do_not_have_closing_token() {
        let p = "```\nprintln!(\"hello\");\n\n\n``".into();
        assert_eq!(code(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, 0..3, Position::default()),
            ))
        );
    }
}
