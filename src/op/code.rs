use crate::{
    lexer::{Token, TokenKind},
    op::{
        Node, Op, Parser,
        modifier::modifier,
        parser::{eat_seq, eol},
    },
};

fn is_backtick3(t: &Token) -> bool {
    t.kind == TokenKind::Backtick && t.position.column == 0 && t.range.len() == 3
}

fn is_eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

pub fn code(p: &mut Parser) -> bool {
    let start_pos = p.pos;
    let snap = p.ops.len();

    let first_range = eat_seq!(p, is_backtick3, is_eol).or_else(|| p.eat(is_backtick3));
    let Some(first_range) = first_range else {
        return false;
    };

    let with_modifier = !p.get(first_range.end - 1).is_some_and(eol);

    let first_content = p.span(first_range);
    p.ops.push(Op::new_start(Node::Code, first_content));

    if with_modifier {
        modifier(p);
    }

    let Some((body_range, close_range)) = p.eat_until(is_backtick3) else {
        p.pos = start_pos;
        p.ops.truncate(snap);
        return false;
    };

    let end_range = if let Some(eol_range) = p.eat(is_eol) {
        close_range.start..eol_range.end
    } else if p.at_block_boundary() {
        close_range
    } else {
        p.pos = start_pos;
        p.ops.truncate(snap);
        return false;
    };

    let body_content = p.span(body_range);
    let end_content = p.span(end_range);
    p.ops.push(Op::new_value(body_content));
    p.ops.push(Op::new_end(Node::Code, end_content));

    true
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, code::code, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "```rust\nprintln!(\"hello\");\n```".into();
        assert!(code(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Code, p.span(0..1)),
                Op::new_start(Node::Modifier, p.span(0..0)),
                Op::new_value(p.span(1..2)),
                Op::new_end(Node::Modifier, p.span(2..3)),
                Op::new_value(p.span(3..10)),
                Op::new_end(Node::Code, p.span(10..11)),
            ]
        );
    }

    #[test]
    fn eol_before_lang() {
        let mut p: Parser = "```\nprintln!(\"hello\");\n```".into();
        assert!(code(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Code, p.span(0..2)),
                Op::new_value(p.span(2..9)),
                Op::new_end(Node::Code, p.span(9..10)),
            ]
        );
    }

    #[test]
    fn terminator_before_lang() {
        let mut p: Parser = "```\n\nprintln!(\"hello\");\n```".into();
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(!code(p));
            assert!(p.ops.is_empty());
            assert_eq!(
                p.peek(),
                Some((
                    0,
                    &Token::new(TokenKind::Backtick, 0..3, Position::default()),
                ))
            );
        });
    }

    #[test]
    fn do_not_have_closing_token() {
        let mut p: Parser = "```\nprintln!(\"hello\");\n``".into();
        assert!(!code(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, 0..3, Position::default()),
            ))
        );
    }

    #[test]
    fn trailing_eol_consumed() {
        let mut p: Parser = "```\ncode\n```\n".into();
        assert!(code(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Code, p.span(0..2)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Code, p.span(4..6)),
            ]
        );
        assert!(p.at_eof());
    }

    #[test]
    fn closing_backticks_not_at_block_boundary() {
        let mut p: Parser = "```\ncode\n```extra".into();
        assert!(!code(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(p.pos, 0);
    }

    #[test]
    fn terminator_in_the_middle_and_do_not_have_closing_token() {
        let mut p: Parser = "```\nprintln!(\"hello\");\n\n\n``".into();
        assert!(!code(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, 0..3, Position::default()),
            ))
        );
    }
}
