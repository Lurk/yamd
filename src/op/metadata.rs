use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_three_dashes(t: &Token) -> bool {
    t.kind == TokenKind::Minus && t.position.column == 0 && t.range.len() == 3
}

pub fn metadata(p: &mut Parser) -> bool {
    let start = p.pos;
    let Some(start_range) = p.eat(is_three_dashes) else {
        return false;
    };

    let Some((body_range, end_range)) = p.advance_until(is_three_dashes) else {
        p.pos = start;
        return false;
    };

    let start_content = p.span(start_range);
    let body_content = p.span(body_range);
    let end_content = p.span(end_range);
    p.ops.push(Op::new_start(Node::Metadata, start_content));
    p.ops.push(Op::new_value(body_content));
    p.ops.push(Op::new_end(Node::Metadata, end_content));
    true
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, metadata::metadata},
    };

    #[test]
    fn happy_path() {
        let mut p = "---\ncontent\n---".into();
        assert!(metadata(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Metadata, p.span(0..1)),
                Op::new_value(p.span(1..4)),
                Op::new_end(Node::Metadata, p.span(4..5)),
            ]
        );
    }

    #[test]
    fn no_closing_delimiter() {
        let mut p = "---\ncontent".into();
        assert!(!metadata(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Minus, 0..3, Position::default())))
        );
    }

    #[test]
    fn no_content() {
        let mut p = "---\n---".into();
        assert!(metadata(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Metadata, p.span(0..1)),
                Op::new_value(p.span(1..2)),
                Op::new_end(Node::Metadata, p.span(2..3)),
            ],
        );
    }

    #[test]
    fn only_opening_token() {
        let mut p = "---".into();
        assert!(!metadata(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Minus, 0..3, Position::default())))
        );
    }
}
