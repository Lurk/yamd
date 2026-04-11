use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_backtick(t: &Token) -> bool {
    t.kind == TokenKind::Backtick && t.range.len() == 1
}

pub fn code_span(p: &mut Parser) -> bool {
    let start = p.pos;
    let Some(start_range) = p.eat(is_backtick) else {
        return false;
    };
    let Some((body_range, end_range)) = p.advance_until(is_backtick) else {
        p.pos = start;
        return false;
    };
    let start_content = p.span(start_range);
    let body_content = p.span(body_range);
    let end_content = p.span(end_range);
    p.ops.push(Op::new_start(Node::CodeSpan, start_content));
    p.ops.push(Op::new_value(body_content));
    p.ops.push(Op::new_end(Node::CodeSpan, end_content));
    true
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, code_span::code_span, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "`happy path`".into();
        assert!(code_span(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::CodeSpan, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_end(Node::CodeSpan, p.span(2..3)),
            ]
        );
    }

    #[test]
    fn terminator() {
        let mut p: Parser = "`happy\n\npath`".into();
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(!code_span(p));
            assert!(p.ops.is_empty());
            assert_eq!(
                p.peek(),
                Some((
                    0,
                    &Token::new(TokenKind::Backtick, 0..1, Position::default())
                ))
            );
        });
    }

    #[test]
    fn no_rhs() {
        let mut p: Parser = "`happy path".into();
        assert!(!code_span(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Backtick, 0..1, Position::default())
            ))
        )
    }
}
