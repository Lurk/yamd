use crate::{
    is, join,
    lexer::TokenKind,
    op::{
        italic::italic,
        op::{Node, Op},
        parser::{Parser, Query},
        strikethrough::strikethrough,
    },
};

pub fn bold<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let query = join!(is!(t = TokenKind::Star, el = 2,));
    let start_token = p.chain(&query, false)?;
    let mut ops = vec![Op::new_start(Node::Bold, Vec::from_iter(start_token))];

    let mut text = Op::new_value(vec![]);
    let mut end_token = None;

    while let Some((_, token)) = p.peek() {
        if p.chain(eof, false).is_some() {
            p.replace_position(start);
            return None;
        } else if let Some(token) = p.chain(&query, false) {
            end_token.replace(token);
            break;
        } else if let Some(nested_ops) = strikethrough(p, eof) {
            if !text.tokens.is_empty() {
                ops.push(text);
                text = Op::new_value(vec![]);
            }
            ops.extend(nested_ops);
        } else if let Some(nested_ops) = italic(p, eof) {
            if !text.tokens.is_empty() {
                ops.push(text);
                text = Op::new_value(vec![]);
            }
            ops.extend(nested_ops);
        } else {
            text.tokens.push(token);
            p.next();
        }
    }

    if let Some(end_token) = end_token {
        if !text.tokens.is_empty() {
            ops.push(text);
        }

        ops.push(Op::new_end(Node::Bold, Vec::from_iter(end_token)));

        return Some(ops);
    }

    p.replace_position(start);
    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        is,
        lexer::{Position, Token, TokenKind},
        op::{Op, Parser, bold::bold, op::Node, parser::Query},
    };

    #[test]
    fn happy_path() {
        let p: Parser = "**~~happy~~ _path_**".into();
        assert_eq!(
            bold(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Bold, vec![p.get(0).unwrap()]),
                Op::new_start(Node::Strikethrough, vec![p.get(1).unwrap()]),
                Op::new_value(vec![p.get(2).unwrap()]),
                Op::new_end(Node::Strikethrough, vec![p.get(3).unwrap()]),
                Op::new_value(vec![p.get(4).unwrap()]),
                Op::new_start(Node::Italic, vec![p.get(5).unwrap()]),
                Op::new_value(vec![p.get(6).unwrap()]),
                Op::new_end(Node::Italic, vec![p.get(7).unwrap()]),
                Op::new_end(Node::Bold, vec![p.get(8).unwrap()]),
            ])
        );
    }

    #[test]
    fn terminator() {
        let p: Parser = "**~~happy~~ _path_\n\n**".into();
        assert_eq!(bold(&p, &is!(t = TokenKind::Terminator,)), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..2, Position::default())))
        )
    }

    #[test]
    fn end_of_input() {
        let p: Parser = "**~~happy~~ _path_".into();
        assert_eq!(bold(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..2, Position::default())))
        )
    }

    #[test]
    fn end_of_input_in_nested() {
        let p: Parser = "**~~happy _path_".into();
        assert_eq!(bold(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Star, 0..2, Position::default())))
        );
    }

    #[test]
    fn text_before_strikethrough() {
        let p: Parser = "**text ~~happy~~ _path_**".into();
        assert_eq!(
            bold(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Bold, vec![p.get(0).unwrap()]),
                Op::new_value(Vec::from_iter(p.slice(1..2))),
                Op::new_start(Node::Strikethrough, vec![p.get(2).unwrap()]),
                Op::new_value(vec![p.get(3).unwrap()]),
                Op::new_end(Node::Strikethrough, vec![p.get(4).unwrap()]),
                Op::new_value(vec![p.get(5).unwrap()]),
                Op::new_start(Node::Italic, vec![p.get(6).unwrap()]),
                Op::new_value(vec![p.get(7).unwrap()]),
                Op::new_end(Node::Italic, vec![p.get(8).unwrap()]),
                Op::new_end(Node::Bold, vec![p.get(9).unwrap()]),
            ])
        );
    }

    #[test]
    fn unclosed_italic() {
        let p: Parser = "**_path**".into();
        assert_eq!(
            bold(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Bold, vec![p.get(0).unwrap()]),
                Op::new_value(Vec::from_iter(p.slice(1..3))),
                Op::new_end(Node::Bold, vec![p.get(3).unwrap()]),
            ])
        );
    }
}
