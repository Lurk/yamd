use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_star(t: &Token) -> bool {
    t.kind == TokenKind::Star && t.range.len() == 1
}

pub fn emphasis(p: &mut Parser) -> bool {
    let start = p.pos;
    let Some(start_range) = p.eat(is_star) else {
        return false;
    };
    let Some((body_range, end_range)) = p.eat_until(is_star) else {
        p.pos = start;
        return false;
    };
    let start_content = p.span(start_range);
    let body_content = p.span(body_range);
    let end_content = p.span(end_range);
    p.ops.push(Op::new_start(Node::Emphasis, start_content));
    p.ops.push(Op::new_value(body_content));
    p.ops.push(Op::new_end(Node::Emphasis, end_content));
    true
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, emphasis::emphasis, parser::StopCondition},
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "*happy*".into();
        assert!(emphasis(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Emphasis, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_end(Node::Emphasis, p.span(2..3)),
            ]
        );
    }

    #[test]
    fn no_closing_token() {
        let mut p: Parser = "*happy".into();
        assert!(!emphasis(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..1, Position::default())))
        )
    }

    #[test]
    fn terminator() {
        let mut p: Parser = "*ha\n\nppy*".into();
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(!emphasis(p));
            assert!(p.ops.is_empty());
            assert_eq!(
                p.peek(),
                Some((0, &Token::new(TokenKind::Star, 0..1, Position::default())))
            );
        });
    }
}
