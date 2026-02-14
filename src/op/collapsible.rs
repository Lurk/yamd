use crate::{
    eat_seq,
    lexer::{Token, TokenKind},
    op::{Node, Op, Parser, document::document, modifier::modifier, parser::StopCondition},
};

fn is_collapsible_start(t: &Token) -> bool {
    t.kind == TokenKind::CollapsibleStart && t.position.column == 0
}

fn is_collapsible_end(t: &Token) -> bool {
    t.kind == TokenKind::CollapsibleEnd && t.position.column == 0
}

fn is_space_or_eol(t: &Token) -> bool {
    t.kind == TokenKind::Space || t.kind == TokenKind::Eol
}

fn is_eol(t: &Token) -> bool {
    t.kind == TokenKind::Eol
}

pub fn collapsible(p: &Parser) -> Option<Vec<Op>> {
    let start = p.pos();

    let collapsible_start = eat_seq!(p, is_collapsible_start, is_space_or_eol)?;

    let mut ops = vec![Op::new_start(Node::Collapsible, collapsible_start)];

    if let Some(heading) = modifier(p) {
        ops.extend(heading);
    }

    if !p.at(|t: &Token| t.position.column == 0) {
        p.replace_position(start);
        return None;
    }

    {
        let _g = p.push_eof(StopCondition::CollapsibleEnd);
        ops.extend(document(p));
    }

    let end = eat_seq!(p, is_collapsible_end, is_eol).or_else(|| p.eat(is_collapsible_end));

    let Some(end) = end else {
        p.replace_position(start);
        return None;
    };

    ops.push(Op::new_end(Node::Collapsible, end));
    Some(ops)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Node, Op, collapsible::collapsible},
    };

    #[test]
    fn happy_path() {
        let p = "{% Title\n# Heading\n\ntext\n\n{% nested\n![a](u)\n%}\n%}".into();
        assert_eq!(
            collapsible(&p),
            Some(vec![
                Op::new_start(Node::Collapsible, p.slice(0..2)), // {%
                Op::new_start(Node::Modifier, &[]),              //
                Op::new_value(p.slice(2..3)),                    // Title
                Op::new_end(Node::Modifier, p.slice(3..4)),      // \n
                Op::new_start(Node::Document, &[]),              //
                Op::new_start(Node::Heading, p.slice(4..6)),     // #
                Op::new_value(p.slice(6..7)),                    // Heading
                Op::new_end(Node::Heading, &[]),                 //
                Op::new_value(p.slice(7..8)),                    // \n\n
                Op::new_start(Node::Paragraph, &[]),             //
                Op::new_value(p.slice(8..9)),                    // text
                Op::new_end(Node::Paragraph, &[]),               //
                Op::new_value(p.slice(9..10)),                   // \n\n
                Op::new_start(Node::Collapsible, p.slice(10..12)), // {%
                Op::new_start(Node::Modifier, &[]),              //
                Op::new_value(p.slice(12..13)),                  // nested
                Op::new_end(Node::Modifier, p.slice(13..14)),    // \n
                Op::new_start(Node::Document, &[]),              //
                Op::new_start(Node::Image, p.slice(14..15)),     // !
                Op::new_start(Node::Title, p.slice(15..16)),     // [
                Op::new_value(p.slice(16..17)),                  // a
                Op::new_end(Node::Title, p.slice(17..18)),       // ]
                Op::new_start(Node::Destination, p.slice(18..19)), // (
                Op::new_value(p.slice(19..20)),                  // u
                Op::new_end(Node::Destination, p.slice(20..21)), // )
                Op::new_end(Node::Image, p.slice(21..22)),       // \n
                Op::new_end(Node::Document, &[]),                //
                Op::new_end(Node::Collapsible, p.slice(22..24)), // %}\n
                Op::new_end(Node::Document, &[]),                //
                Op::new_end(Node::Collapsible, p.slice(24..25)), // %}
            ])
        );
    }

    #[test]
    fn no_title() {
        let p = "{%\ntext%}".into();
        assert_eq!(collapsible(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::CollapsibleStart, 0..2, Position::default()),
            ))
        );
    }

    #[test]
    fn no_end_token() {
        let p = "{% Title\n# Heading\n\ntext\n\n{% nested\n![a](u)\n%}\n".into();
        assert_eq!(collapsible(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::CollapsibleStart, 0..2, Position::default()),
            ))
        );
    }

    #[test]
    fn just_heading() {
        let p = "{% Title\n# Heading\n%}".into();
        assert_eq!(
            collapsible(&p),
            Some(vec![
                Op::new_start(Node::Collapsible, p.slice(0..2)), // {%
                Op::new_start(Node::Modifier, &[]),              //
                Op::new_value(p.slice(2..3)),                    // Title
                Op::new_end(Node::Modifier, p.slice(3..4)),      // \n
                Op::new_start(Node::Document, &[]),              //
                Op::new_start(Node::Heading, p.slice(4..6)),     // #
                Op::new_value(p.slice(6..8)),                    // Heading\n
                Op::new_end(Node::Heading, &[]),                 //
                Op::new_end(Node::Document, &[]),                //
                Op::new_end(Node::Collapsible, p.slice(8..9)),   // %}
            ])
        );
    }

    #[test]
    fn only_two_tokens() {
        let p = "{% ".into();
        assert_eq!(collapsible(&p), None);
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::CollapsibleStart, 0..2, Position::default()),
            ))
        );
    }
}
