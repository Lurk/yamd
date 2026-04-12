use crate::{
    lexer::{Token, TokenKind},
    op::{Content, Node, Op, Parser},
};

fn is_dash(t: &Token) -> bool {
    t.kind == TokenKind::Minus && t.position.column == 0 && t.range.len() == 5
}

pub fn thematic_break(p: &mut Parser) -> bool {
    let Some(range) = p.eat(is_dash) else {
        return false;
    };
    let content = p.span(range);
    let empty = Content::Span(0..0);
    p.ops
        .push(Op::new_start(Node::ThematicBreak, Content::Span(0..0)));
    p.ops.push(Op::new_value(content));
    p.ops.push(Op::new_end(Node::ThematicBreak, empty));
    true
}

#[cfg(test)]
mod tests {
    use crate::op::{Content, Node, Op, thematic_break::thematic_break};

    #[test]
    fn happy_path() {
        let mut p = "-----".into();
        assert!(thematic_break(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::ThematicBreak, Content::Span(0..0)),
                Op::new_value(p.span(0..1)),
                Op::new_end(Node::ThematicBreak, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn not_enough_dashes() {
        let mut p = "---".into();
        assert!(!thematic_break(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(p.peek(), Some((0, p.get(0).unwrap())));
    }
}
