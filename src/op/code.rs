use crate::{
    eat_seq,
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser, modifier::modifier, parser::eol},
};

fn is_backtick3(t: &Token) -> bool {
    t.kind == TokenKind::Backtick && t.position.column == 0 && t.range.len() == 3
}

fn is_eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

pub fn code(p: &Parser) -> Option<Vec<Op>> {
    let start_pos = p.pos();

    let first_token = eat_seq!(p, is_backtick3, is_eol).or_else(|| p.eat(is_backtick3))?;

    let with_modifier = !first_token.last().is_some_and(eol);

    let mut ops = vec![Op::new_start(Node::Code, first_token)];

    if with_modifier && let Some(modifier_ops) = modifier(p) {
        ops.extend(modifier_ops);
    };

    let Some((body, end_token)) = p.advance_until(is_backtick3) else {
        p.replace_position(start_pos);
        return None;
    };

    if !p.at_block_boundary() {
        p.replace_position(start_pos);
        return None;
    }

    ops.push(Op::new_value(body));
    ops.push(Op::new_end(Node::Code, end_token));

    Some(ops)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, code::code, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let p = "```rust\nprintln!(\"hello\");\n```".into();
        assert_eq!(
            code(&p),
            Some(vec![
                Op::new_start(Node::Code, p.slice(0..1)),   // ```
                Op::new_start(Node::Modifier, &[]),         //
                Op::new_value(p.slice(1..2)),               // rust
                Op::new_end(Node::Modifier, p.slice(2..3)), // \n
                Op::new_value(p.slice(3..10)),              // println!(\"hello\");\n
                Op::new_end(Node::Code, p.slice(10..11)),   // ```
            ])
        );
    }

    #[test]
    fn eol_before_lang() {
        let p = "```\nprintln!(\"hello\");\n```".into();
        assert_eq!(
            code(&p),
            Some(vec![
                Op::new_start(Node::Code, p.slice(0..2)), // ```\n
                Op::new_value(p.slice(2..9)),             // println!(\"hello\");\n
                Op::new_end(Node::Code, p.slice(9..10)),  // ```
            ])
        );
    }

    #[test]
    fn terminator_before_lang() {
        let p: Parser = "```\n\nprintln!(\"hello\");\n```".into();
        let _g = p.push_eof(StopCondition::Terminator);
        assert_eq!(code(&p), None);
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
        assert_eq!(code(&p), None);
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
        assert_eq!(code(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, 0..3, Position::default()),
            ))
        );
    }
}
