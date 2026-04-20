use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_tilde(t: &Token) -> bool {
    t.kind == TokenKind::Tilde && t.range.len() == 2
}

pub fn strikethrough(p: &mut Parser) -> bool {
    let start = p.pos;
    let Some(start_range) = p.eat(is_tilde) else {
        return false;
    };
    let Some((body_range, end_range)) = p.eat_until(is_tilde) else {
        p.pos = start;
        return false;
    };
    let start_content = p.span(start_range);
    let body_content = p.span(body_range);
    let end_content = p.span(end_range);
    p.ops
        .push(Op::new_start(Node::Strikethrough, start_content));
    p.ops.push(Op::new_value(body_content));
    p.ops.push(Op::new_end(Node::Strikethrough, end_content));
    true
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, Parser, parser::StopCondition, strikethrough::strikethrough},
    };

    #[test]
    fn happy_path() {
        let mut p: Parser = "~~happy~~".into();
        assert!(strikethrough(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Strikethrough, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_end(Node::Strikethrough, p.span(2..3)),
            ]
        );
    }

    #[test]
    fn no_closing_token() {
        let mut p: Parser = "~~happy".into();
        assert!(!strikethrough(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Tilde, 0..2, Position::default())))
        )
    }

    #[test]
    fn terminator() {
        let mut p: Parser = "~~ha\n\nppy~~".into();
        p.with_eof(StopCondition::Terminator, |p| {
            assert!(!strikethrough(p));
            assert!(p.ops.is_empty());
            assert_eq!(
                p.peek(),
                Some((0, &Token::new(TokenKind::Tilde, 0..2, Position::default())))
            );
        });
    }
}
