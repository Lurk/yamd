use crate::op::{Op, Parser, image::image, op::Node, parser::Query};

pub fn images<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let mut all_ops = vec![];
    let mut count = 0;
    while p.peek().is_some() {
        if let Some(ops) = image(p, eof) {
            count += 1;
            all_ops.extend(ops);
        } else if p.chain(eof, true).is_none() {
            break;
        } else {
            println!("breaking on eol check");
            p.replace_position(start);
            return None;
        }
    }

    if count == 1 {
        return Some(all_ops);
    } else if count > 1 {
        let mut res = vec![Op::new_start(Node::Images, vec![])];
        res.extend(all_ops);
        res.push(Op::new_end(Node::Images, vec![]));
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
        op::{Op, images::images, op::Node, parser::Query},
    };

    #[test]
    fn happy_path() {
        let p = "![a](u)\n![a](u)".into();
        assert_eq!(
            images(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Images, vec![]),
                Op::new_start(Node::Image, vec![p.get(0).unwrap()]), // !
                Op::new_start(Node::Title, vec![p.get(1).unwrap()]), // [
                Op::new_value(vec![p.get(2).unwrap()]),              // a
                Op::new_end(Node::Title, vec![p.get(3).unwrap()]),   // ]
                Op::new_start(Node::Destination, vec![p.get(4).unwrap()]), // (
                Op::new_value(vec![p.get(5).unwrap()]),              // u
                Op::new_end(Node::Destination, vec![p.get(6).unwrap()]), // )
                Op::new_end(Node::Image, vec![p.get(7).unwrap()]),   // \n
                Op::new_start(Node::Image, vec![p.get(8).unwrap()]), // !
                Op::new_start(Node::Title, vec![p.get(9).unwrap()]), // [
                Op::new_value(vec![p.get(10).unwrap()]),             // a
                Op::new_end(Node::Title, vec![p.get(11).unwrap()]),  // ]
                Op::new_start(Node::Destination, vec![p.get(12).unwrap()]), // (
                Op::new_value(vec![p.get(13).unwrap()]),             // u
                Op::new_end(Node::Destination, vec![p.get(14).unwrap()]), // )
                Op::new_end(Node::Image, vec![]),
                Op::new_end(Node::Images, vec![]),
            ])
        );
    }

    #[test]
    fn not_an_anchor() {
        let p = "![a](u)\n!!foo".into();
        assert_eq!(images(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..1, Position::default())))
        );
    }

    #[test]
    fn next_token_can_be_only_terminator() {
        let p = "![a](u)\n![a](u)fasdf".into();
        assert_eq!(images(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..1, Position::default())))
        );
    }

    #[test]
    fn new_line_check() {
        let p = "![a](u)\n![a](u)\n".into();
        assert_eq!(
            images(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Images, vec![]),
                Op::new_start(Node::Image, vec![p.get(0).unwrap()]), // !
                Op::new_start(Node::Title, vec![p.get(1).unwrap()]), // [
                Op::new_value(vec![p.get(2).unwrap()]),              // a
                Op::new_end(Node::Title, vec![p.get(3).unwrap()]),   // ]
                Op::new_start(Node::Destination, vec![p.get(4).unwrap()]), // (
                Op::new_value(vec![p.get(5).unwrap()]),              // u
                Op::new_end(Node::Destination, vec![p.get(6).unwrap()]), // )
                Op::new_end(Node::Image, vec![p.get(7).unwrap()]),   // \n
                Op::new_start(Node::Image, vec![p.get(8).unwrap()]), // !
                Op::new_start(Node::Title, vec![p.get(9).unwrap()]), // [
                Op::new_value(vec![p.get(10).unwrap()]),             // a
                Op::new_end(Node::Title, vec![p.get(11).unwrap()]),  // ]
                Op::new_start(Node::Destination, vec![p.get(12).unwrap()]), // (
                Op::new_value(vec![p.get(13).unwrap()]),             // u
                Op::new_end(Node::Destination, vec![p.get(14).unwrap()]), // )
                Op::new_end(Node::Image, vec![p.get(15).unwrap()]),  // \n
                Op::new_end(Node::Images, vec![]),
            ])
        );
    }
}
