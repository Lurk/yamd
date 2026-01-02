use crate::{
    is, join,
    lexer::TokenKind,
    op::{
        Op, Parser,
        document::document,
        modifier::modifier,
        op::Node,
        parser::{Query, first_column},
    },
    or,
};

fn one_collapsible_start_at_first_column(t: &crate::lexer::Token) -> bool {
    first_column(t) && t.kind == TokenKind::CollapsibleStart && t.slice.len() == 1
}

fn one_collapsible_end_at_first_column(t: &crate::lexer::Token) -> bool {
    first_column(t) && t.kind == TokenKind::CollapsibleEnd && t.slice.len() == 1
}

pub fn collapsible<'a>(p: &'a Parser<'a>, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();

    println!("parsing collapsible at position {}", start);
    let collapsible_start = p.chain(
        &join!(
            is!(t = TokenKind::CollapsibleStart, c = 0,),
            or!(is!(t = TokenKind::Space,), is!(t = TokenKind::Eol,))
        ),
        false,
    )?;

    let mut ops = vec![Op::new_start(
        Node::Collapsible,
        Vec::from_iter(collapsible_start),
    )];

    if let Some(heading) = modifier(p, eof) {
        ops.extend(heading);
    }

    if !p.is(first_column) {
        p.replace_position(start);
        return None;
    }

    println!("document at: {}", p.pos());
    ops.extend(document(
        p,
        &or!(eof.clone(), is!(t = TokenKind::CollapsibleEnd, c = 0,)),
    ));

    println!("parsed document inside collapsible");

    println!("position before searching for end: {}", p.pos());
    let Some(end) = p.chain(
        &or!(
            join!(
                is!(t = TokenKind::CollapsibleEnd, c = 0,),
                is!(t = TokenKind::Eol,)
            ),
            join!(is!(t = TokenKind::CollapsibleEnd, c = 0,))
        ),
        false,
    ) else {
        println!("Failed to find collapsible end");
        p.replace_position(start);
        return None;
    };

    println!("found collapsible end at position {}", p.pos());

    ops.push(Op::new_end(Node::Collapsible, Vec::from_iter(end)));
    Some(ops)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::op::{Op, collapsible::collapsible, op::Node, parser::Query};

    #[test]
    fn happy_path() {
        let p = "{% Title\n# Heading\n\ntext\n\n{% nested\n![a](u)\n%}\n%}".into();
        assert_eq!(
            collapsible(&p, &Query::Eof),
            Some(vec![
                Op::new_start(Node::Collapsible, Vec::from_iter(p.slice(0..2)),), // {%
                Op::new_start(Node::Modifier, vec![],),                           //
                Op::new_value(vec![p.get(2).unwrap()]),                           // Title
                Op::new_end(Node::Modifier, vec![p.get(3).unwrap()]),             // \n
                Op::new_start(Node::Document, vec![],),                           //
                Op::new_start(Node::Heading, Vec::from_iter(p.slice(4..6)),),     // #
                Op::new_value(vec![p.get(6).unwrap()]),                           // Heading
                Op::new_end(Node::Heading, vec![]),                               //
                Op::new_value(vec![p.get(7).unwrap()]),                           // \n\n
                Op::new_start(Node::Paragraph, vec![]),                           //
                Op::new_value(vec![p.get(8).unwrap()]),                           // text
                Op::new_end(Node::Paragraph, vec![]),                             //
                Op::new_value(vec![p.get(9).unwrap()]),                           // \n\n
                Op::new_start(Node::Collapsible, Vec::from_iter(p.slice(10..12)),), // {%
                Op::new_start(Node::Modifier, vec![],),                           //
                Op::new_value(vec![p.get(12).unwrap()]),                          // nested
                Op::new_end(Node::Modifier, vec![p.get(13).unwrap()]),            // \n
                Op::new_start(Node::Document, vec![],),                           //
                Op::new_start(Node::Image, vec![p.get(14).unwrap()]),             // !
                Op::new_start(Node::Title, vec![p.get(15).unwrap()]),             // [
                Op::new_value(vec![p.get(16).unwrap()]),                          // a
                Op::new_end(Node::Title, vec![p.get(17).unwrap()]),               // ]
                Op::new_start(Node::Destination, vec![p.get(18).unwrap()]),       // (
                Op::new_value(vec![p.get(19).unwrap()]),                          // u
                Op::new_end(Node::Destination, vec![p.get(20).unwrap()]),         // )
                Op::new_end(Node::Image, vec![p.get(21).unwrap()]),               // \n
                Op::new_end(Node::Document, vec![]),                              //
                Op::new_end(Node::Collapsible, Vec::from_iter(p.slice(22..24))),  // %}\n
                Op::new_end(Node::Document, vec![]),                              //
                Op::new_end(Node::Collapsible, vec![p.get(24).unwrap()]),         // %}
            ])
        );
    }

    // #[test]
    // fn no_title() {
    //     let mut p = Parser::new("{%\ntext%}");
    //     assert_eq!(collapsible(&mut p), None);
    //     assert_eq!(
    //         p.peek(),
    //         Some((
    //             &Token::new(TokenKind::Literal, "{%", Position::default()),
    //             0
    //         ))
    //     );
    // }
    //
    // #[test]
    // fn parse_empty() {
    //     let mut p = Parser::new("{% Title\n\n%}");
    //     assert_eq!(collapsible(&mut p), Some(Collapsible::new("Title", vec![])));
    // }
    //
    // #[test]
    // fn no_end_token() {
    //     let mut p = Parser::new("{% Title\n# Heading\n\ntext\n\n{% nested\n![a](u)\n%}\n");
    //     assert_eq!(collapsible(&mut p), None);
    //     assert_eq!(
    //         p.peek(),
    //         Some((
    //             &Token::new(TokenKind::Literal, "{%", Position::default()),
    //             0
    //         ))
    //     );
    // }
    //
    // #[test]
    // fn just_heading() {
    //     let mut p = Parser::new("{% Title\n# Heading\n%}");
    //     assert_eq!(
    //         collapsible(&mut p),
    //         Some(Collapsible::new(
    //             "Title",
    //             vec![Heading::new(1, vec![String::from("Heading").into()]).into(),]
    //         ))
    //     );
    // }
    //
    // #[test]
    // fn only_two_tokens() {
    //     let mut p = Parser::new("{% ");
    //     assert_eq!(collapsible(&mut p), None);
    //     assert_eq!(
    //         p.peek(),
    //         Some((
    //             &Token::new(TokenKind::Literal, "{%", Position::default()),
    //             0
    //         ))
    //     );
    // }
}
