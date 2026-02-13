use crate::{
    is, join,
    lexer::TokenKind,
    op::{Op, Parser, op::Node},
};

pub fn thematic_break<'a>(p: &'a Parser) -> Option<Vec<Op<'a>>> {
    let token = p.chain(&join!(is!(t = TokenKind::Minus, c = 0, el = 5,)), false)?;

    Some(vec![
        Op::new_start(Node::ThematicBreak, vec![]),
        Op::new_value(Vec::from_iter(token)),
        Op::new_end(Node::ThematicBreak, vec![]),
    ])
}

#[cfg(test)]
mod tests {
    use crate::op::{Op, op::Node, thematic_break::thematic_break};
    use pretty_assertions::assert_eq;
    #[test]
    fn happy_path() {
        let p = "-----".into();
        assert_eq!(
            thematic_break(&p),
            Some(vec![
                Op::new_start(Node::ThematicBreak, vec![]), //
                Op::new_value(vec![p.get(0).unwrap()]),     // -----
                Op::new_end(Node::ThematicBreak, vec![]),   //
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
