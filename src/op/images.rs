use crate::op::{Node, Op, Parser, image::image};

pub fn images(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();
    let mut all_ops = vec![];
    let mut count = 0;
    while p.peek().is_some() {
        if let Some(ops) = image(p) {
            count += 1;
            all_ops.extend(ops);
        } else if p.at_eof() {
            break;
        } else {
            p.replace_position(start);
            return None;
        }
    }

    if count == 1 {
        return Some(all_ops);
    } else if count > 1 {
        let mut res = vec![Op::new_start(Node::Images, &[])];
        res.extend(all_ops);
        res.push(Op::new_end(Node::Images, &[]));
        return Some(res);
    }

    p.replace_position(start);
    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, images::images},
    };

    #[test]
    fn happy_path() {
        let p = "![a](u)\n![a](u)".into();
        assert_eq!(
            images(&p),
            Some(vec![
                Op::new_start(Node::Images, &[]),
                Op::new_start(Node::Image, p.slice(0..1)), // !
                Op::new_start(Node::Title, p.slice(1..2)), // [
                Op::new_value(p.slice(2..3)),              // a
                Op::new_end(Node::Title, p.slice(3..4)),   // ]
                Op::new_start(Node::Destination, p.slice(4..5)), // (
                Op::new_value(p.slice(5..6)),              // u
                Op::new_end(Node::Destination, p.slice(6..7)), // )
                Op::new_end(Node::Image, p.slice(7..8)),   // \n
                Op::new_start(Node::Image, p.slice(8..9)), // !
                Op::new_start(Node::Title, p.slice(9..10)), // [
                Op::new_value(p.slice(10..11)),            // a
                Op::new_end(Node::Title, p.slice(11..12)), // ]
                Op::new_start(Node::Destination, p.slice(12..13)), // (
                Op::new_value(p.slice(13..14)),            // u
                Op::new_end(Node::Destination, p.slice(14..15)), // )
                Op::new_end(Node::Image, &[]),
                Op::new_end(Node::Images, &[]),
            ])
        );
    }

    #[test]
    fn not_an_anchor() {
        let p = "![a](u)\n!!foo".into();
        assert_eq!(images(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..1, Position::default())))
        );
    }

    #[test]
    fn next_token_can_be_only_terminator() {
        let p = "![a](u)\n![a](u)fasdf".into();
        assert_eq!(images(&p), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..1, Position::default())))
        );
    }

    #[test]
    fn new_line_check() {
        let p = "![a](u)\n![a](u)\n".into();
        assert_eq!(
            images(&p),
            Some(vec![
                Op::new_start(Node::Images, &[]),
                Op::new_start(Node::Image, p.slice(0..1)), // !
                Op::new_start(Node::Title, p.slice(1..2)), // [
                Op::new_value(p.slice(2..3)),              // a
                Op::new_end(Node::Title, p.slice(3..4)),   // ]
                Op::new_start(Node::Destination, p.slice(4..5)), // (
                Op::new_value(p.slice(5..6)),              // u
                Op::new_end(Node::Destination, p.slice(6..7)), // )
                Op::new_end(Node::Image, p.slice(7..8)),   // \n
                Op::new_start(Node::Image, p.slice(8..9)), // !
                Op::new_start(Node::Title, p.slice(9..10)), // [
                Op::new_value(p.slice(10..11)),            // a
                Op::new_end(Node::Title, p.slice(11..12)), // ]
                Op::new_start(Node::Destination, p.slice(12..13)), // (
                Op::new_value(p.slice(13..14)),            // u
                Op::new_end(Node::Destination, p.slice(14..15)), // )
                Op::new_end(Node::Image, p.slice(15..16)), // \n
                Op::new_end(Node::Images, &[]),
            ])
        );
    }
}
