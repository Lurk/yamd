use crate::{
    is, join,
    lexer::TokenKind,
    op::{
        Op, Parser,
        modifier::modifier,
        op::Node,
        paragraph::paragraph,
        parser::{Query, eol},
    },
    or,
};

fn icon<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let start_token = p.chain(
        &join!(
            is!(t = TokenKind::Bang, c = 0, el = 1,),
            is!(t = TokenKind::Space, el = 1,)
        ),
        false,
    )?;

    let Some((body, end_token)) = p.advance_until(&join!(is!(t = TokenKind::Eol,)), eof) else {
        p.replace_position(start);
        return None;
    };

    let ops = vec![
        Op::new_start(Node::Icon, Vec::from_iter(start_token)),
        Op::new_value(Vec::from_iter(body)),
        Op::new_end(Node::Icon, Vec::from_iter(end_token)),
    ];

    Some(ops)
}

pub fn highlight<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let start_token = p.chain(
        &join!(
            is!(t = TokenKind::Bang, c = 0, el = 2,),
            or!(is!(t = TokenKind::Space, el = 1,), is!(t = TokenKind::Eol,))
        ),
        false,
    )?;

    let skip_title = start_token.last().is_some_and(eol);

    let mut ops = vec![Op::new_start(Node::Highlight, Vec::from_iter(start_token))];

    if !skip_title && let Some(heading) = modifier(p, eof) {
        ops.extend(heading);
    }

    if let Some(icon_ops) = icon(p, &or!(is!(t = TokenKind::Terminator,), eof.clone())) {
        ops.extend(icon_ops);
    }

    if p.chain(&is!(c = 0,), false).is_none() {
        p.replace_position(start);
        return None;
    }

    while p.chain(eof, true).is_some() {
        if let Some(token) = p.chain(&join!(is!(t = TokenKind::Bang, c = 0, el = 2,)), false) {
            ops.push(Op::new_end(Node::Highlight, Vec::from_iter(token)));
            return Some(ops);
        } else if let Some(token) = p.chain(&join!(is!(t = TokenKind::Terminator,)), false) {
            ops.push(Op::new_value(Vec::from_iter(token)));
        }
        ops.extend(paragraph(
            p,
            &or!(
                is!(t = TokenKind::Bang, c = 0, el = 2,),
                is!(t = TokenKind::Terminator,),
                eof.clone()
            ),
        ));
    }

    p.replace_position(start);
    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{
            Op,
            highlight::highlight,
            op::Node,
            parser::{Condition, Query},
        },
    };

    #[test]
    fn happy_path() {
        let p = "!! Title\n! Icon\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(
            highlight(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Highlight, Vec::from_iter(p.slice(0..2))), // !!
                Op::new_start(Node::Modifier, vec![]),                         //
                Op::new_value(vec![p.get(2).unwrap()]),                        // Title
                Op::new_end(Node::Modifier, vec![p.get(3).unwrap()]),          // \n
                Op::new_start(Node::Icon, Vec::from_iter(p.slice(4..6))),      // !
                Op::new_value(vec![p.get(6).unwrap()]),                        // Icon
                Op::new_end(Node::Icon, vec![p.get(7).unwrap()]),              // \n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_start(Node::Italic, vec![p.get(8).unwrap()]),          // _
                Op::new_value(vec![p.get(9).unwrap()]),                        // i
                Op::new_end(Node::Italic, vec![p.get(10).unwrap()]),           // _
                Op::new_value(vec![p.get(11).unwrap()]),                       // ' '
                Op::new_start(Node::Bold, vec![p.get(12).unwrap()]),           // **
                Op::new_value(vec![p.get(13).unwrap()]),                       // b
                Op::new_end(Node::Bold, vec![p.get(14).unwrap()]),             // **
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_value(vec![p.get(15).unwrap()]),                       // \n\n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_value(vec![p.get(16).unwrap()]),                       // t
                Op::new_start(Node::Strikethrough, vec![p.get(17).unwrap()]),  // ~~
                Op::new_value(vec![p.get(18).unwrap()]),                       // s
                Op::new_end(Node::Strikethrough, vec![p.get(19).unwrap()]),    // ~~
                Op::new_value(Vec::from_iter(p.slice(20..22))),                // t\n
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_end(Node::Highlight, vec![p.get(22).unwrap()]),        // !!
            ])
        );
    }

    #[test]
    fn no_title() {
        let p = "!!\n! Icon\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(
            highlight(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Highlight, Vec::from_iter(p.slice(0..2))), // !!\n
                Op::new_start(Node::Icon, Vec::from_iter(p.slice(2..4))),      // !
                Op::new_value(vec![p.get(4).unwrap()]),                        // Icon
                Op::new_end(Node::Icon, vec![p.get(5).unwrap()]),              // \n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_start(Node::Italic, vec![p.get(6).unwrap()]),          // _
                Op::new_value(vec![p.get(7).unwrap()]),                        // i
                Op::new_end(Node::Italic, vec![p.get(8).unwrap()]),            // _
                Op::new_value(vec![p.get(9).unwrap()]),                        // ' '
                Op::new_start(Node::Bold, vec![p.get(10).unwrap()]),           // **
                Op::new_value(vec![p.get(11).unwrap()]),                       // b
                Op::new_end(Node::Bold, vec![p.get(12).unwrap()]),             // **
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_value(vec![p.get(13).unwrap()]),                       // \n\n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_value(vec![p.get(14).unwrap()]),                       // t
                Op::new_start(Node::Strikethrough, vec![p.get(15).unwrap()]),  // ~~
                Op::new_value(vec![p.get(16).unwrap()]),                       // s
                Op::new_end(Node::Strikethrough, vec![p.get(17).unwrap()]),    // ~~
                Op::new_value(Vec::from_iter(p.slice(18..20))),                // t\n
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_end(Node::Highlight, vec![p.get(20).unwrap()]),        // !!
            ])
        )
    }

    #[test]
    fn no_icon() {
        let p = "!! Title\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(
            highlight(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Highlight, Vec::from_iter(p.slice(0..2))), // !!
                Op::new_start(Node::Modifier, vec![]),                         //
                Op::new_value(vec![p.get(2).unwrap()]),                        // Title
                Op::new_end(Node::Modifier, vec![p.get(3).unwrap()]),          // \n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_start(Node::Italic, vec![p.get(4).unwrap()]),          // _
                Op::new_value(vec![p.get(5).unwrap()]),                        // i
                Op::new_end(Node::Italic, vec![p.get(6).unwrap()]),            // _
                Op::new_value(vec![p.get(7).unwrap()]),                        // ' '
                Op::new_start(Node::Bold, vec![p.get(8).unwrap()]),            // **
                Op::new_value(vec![p.get(9).unwrap()]),                        // b
                Op::new_end(Node::Bold, vec![p.get(10).unwrap()]),             // **
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_value(vec![p.get(11).unwrap()]),                       // \n\n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_value(vec![p.get(12).unwrap()]),                       // t
                Op::new_start(Node::Strikethrough, vec![p.get(13).unwrap()]),  // ~~
                Op::new_value(vec![p.get(14).unwrap()]),                       // s
                Op::new_end(Node::Strikethrough, vec![p.get(15).unwrap()]),    // ~~
                Op::new_value(Vec::from_iter(p.slice(16..18))),                // t\n
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_end(Node::Highlight, vec![p.get(18).unwrap()]),        // !!
            ])
        )
    }

    #[test]
    fn no_closing_token() {
        let p = "!! Title\n_i_ **b**\n\nt~~s~~t!!".into();
        assert_eq!(highlight(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default()),))
        );
    }

    #[test]
    fn no_space_between_start_and_title() {
        let p = "!!Title\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(highlight(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default()),))
        );
    }

    #[test]
    fn terminator_in_title() {
        let p = "!! Title\n\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(
            highlight(&p, &Query::Is(Condition::new().kind(TokenKind::Terminator))),
            None
        );
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default()),))
        );
    }

    #[test]
    fn terminator_in_icon() {
        let p = "!!\n! icon\n\n_i_ **b**\n\nt~~s~~t\n!!".into();
        assert_eq!(
            highlight(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Highlight, Vec::from_iter(p.slice(0..2))), // !!\n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_value(Vec::from_iter(p.slice(2..5))),                  // ! icon
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_value(vec![p.get(5).unwrap()]),                        // \n\n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_start(Node::Italic, vec![p.get(6).unwrap()]),          // _
                Op::new_value(vec![p.get(7).unwrap()]),                        // i
                Op::new_end(Node::Italic, vec![p.get(8).unwrap()]),            // _
                Op::new_value(vec![p.get(9).unwrap()]),                        // ' '
                Op::new_start(Node::Bold, vec![p.get(10).unwrap()]),           // **
                Op::new_value(vec![p.get(11).unwrap()]),                       // b
                Op::new_end(Node::Bold, vec![p.get(12).unwrap()]),             // **
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_value(vec![p.get(13).unwrap()]),                       // \n\n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_value(vec![p.get(14).unwrap()]),                       // t
                Op::new_start(Node::Strikethrough, vec![p.get(15).unwrap()]),  // ~~
                Op::new_value(vec![p.get(16).unwrap()]),                       // s
                Op::new_end(Node::Strikethrough, vec![p.get(17).unwrap()]),    // ~~
                Op::new_value(Vec::from_iter(p.slice(18..20))),                // t\n
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_end(Node::Highlight, vec![p.get(20).unwrap()]),        // !!
            ])
        );
    }

    #[test]
    fn no_space_before_icon() {
        let p = "!! Title\n!Icon\n!!".into();
        assert_eq!(
            highlight(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Highlight, Vec::from_iter(p.slice(0..2))), // !!
                Op::new_start(Node::Modifier, vec![]),                         //
                Op::new_value(vec![p.get(2).unwrap()]),                        // Title
                Op::new_end(Node::Modifier, vec![p.get(3).unwrap()]),          // \n
                Op::new_start(Node::Paragraph, vec![]),                        //
                Op::new_value(Vec::from_iter(p.slice(4..7))),                  // !Icon\n
                Op::new_end(Node::Paragraph, vec![]),                          //
                Op::new_end(Node::Highlight, vec![p.get(7).unwrap()]),         // !!
            ])
        );
    }

    #[test]
    fn only_one_token() {
        let p = "!!".into();
        assert_eq!(highlight(&p, &Query::Eof), None);
        assert_eq!(
            p.peek(),
            Some((0, &Token::new(TokenKind::Bang, 0..2, Position::default()),))
        );
    }
}
