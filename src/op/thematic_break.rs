use crate::{
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser},
};

fn is_dash(t: &Token) -> bool {
    t.kind == TokenKind::Minus && t.position.column == 0 && t.range.len() == 5
}

pub fn thematic_break(p: &Parser) -> Option<Vec<Op>> {
    let token = p.eat(is_dash)?;

    Some(vec![
        Op::new_start(Node::ThematicBreak, &[]),
        Op::new_value(token),
        Op::new_end(Node::ThematicBreak, &[]),
    ])
}

#[cfg(test)]
mod tests {
    use crate::op::{Node, Op, thematic_break::thematic_break};
    use pretty_assertions::assert_eq;
    #[test]
    fn happy_path() {
        let p = "-----".into();
        assert_eq!(
            thematic_break(&p),
            Some(vec![
                Op::new_start(Node::ThematicBreak, &[]), //
                Op::new_value(p.slice(0..1)),            // -----
                Op::new_end(Node::ThematicBreak, &[]),   //
            ])
        );
    }

    #[test]
    fn not_enough_dashes() {
        let p = "---".into();
        assert_eq!(thematic_break(&p), None);
        assert_eq!(p.peek(), Some((0, p.get(0).unwrap())));
    }
}
