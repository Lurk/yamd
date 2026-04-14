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

pub fn collapsible(p: &mut Parser) -> bool {
    let start = p.pos;
    let snap = p.ops.len();

    let Some(start_range) = eat_seq!(p, is_collapsible_start, is_space_or_eol) else {
        return false;
    };

    let start_content = p.span(start_range);
    p.ops.push(Op::new_start(Node::Collapsible, start_content));

    modifier(p);

    if !p.at(|t: &Token| t.position.column == 0) {
        p.pos = start;
        p.ops.truncate(snap);
        p.flip_to_literal(start);
        return false;
    }

    p.with_eof(StopCondition::CollapsibleEnd, |p| {
        document(p);
    });

    let end_range = eat_seq!(p, is_collapsible_end, is_eol).or_else(|| p.eat(is_collapsible_end));

    let Some(end_range) = end_range else {
        p.pos = start;
        p.ops.truncate(snap);
        p.flip_to_literal(start);
        return false;
    };

    let end_content = p.span(end_range);
    p.ops.push(Op::new_end(Node::Collapsible, end_content));
    true
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, Token, TokenKind},
        op::{Content, Node, Op, collapsible::collapsible},
    };

    #[test]
    fn happy_path() {
        let mut p = "{% Title\n# Heading\n\ntext\n\n{% nested\n![a](u)\n%}\n%}".into();
        assert!(collapsible(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Collapsible, p.span(0..2)),
                Op::new_start(Node::Modifier, Content::Span(0..0)),
                Op::new_value(p.span(2..3)),
                Op::new_end(Node::Modifier, p.span(3..4)),
                Op::new_start(Node::Document, Content::Span(0..0)),
                Op::new_start(Node::Heading, p.span(4..6)),
                Op::new_value(p.span(6..7)),
                Op::new_end(Node::Heading, Content::Span(0..0)),
                Op::new_value(p.span(7..8)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(8..9)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(9..10)),
                Op::new_start(Node::Collapsible, p.span(10..12)),
                Op::new_start(Node::Modifier, Content::Span(0..0)),
                Op::new_value(p.span(12..13)),
                Op::new_end(Node::Modifier, p.span(13..14)),
                Op::new_start(Node::Document, Content::Span(0..0)),
                Op::new_start(Node::Image, p.span(14..15)),
                Op::new_start(Node::Title, p.span(15..16)),
                Op::new_value(p.span(16..17)),
                Op::new_end(Node::Title, p.span(17..18)),
                Op::new_start(Node::Destination, p.span(18..19)),
                Op::new_value(p.span(19..20)),
                Op::new_end(Node::Destination, p.span(20..21)),
                Op::new_end(Node::Image, p.span(21..22)),
                Op::new_end(Node::Document, Content::Span(0..0)),
                Op::new_end(Node::Collapsible, p.span(22..24)),
                Op::new_end(Node::Document, Content::Span(0..0)),
                Op::new_end(Node::Collapsible, p.span(24..25)),
            ]
        );
    }

    #[test]
    fn no_title() {
        let mut p = "{%\ntext%}".into();
        assert!(!collapsible(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Literal, 0..2, Position::default()),
            ))
        );
    }

    #[test]
    fn no_end_token() {
        let mut p = "{% Title\n# Heading\n\ntext\n\n{% nested\n![a](u)\n%}\n".into();
        assert!(!collapsible(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Literal, 0..2, Position::default()),
            ))
        );
    }

    #[test]
    fn just_heading() {
        let mut p = "{% Title\n# Heading\n%}".into();
        assert!(collapsible(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Collapsible, p.span(0..2)),
                Op::new_start(Node::Modifier, Content::Span(0..0)),
                Op::new_value(p.span(2..3)),
                Op::new_end(Node::Modifier, p.span(3..4)),
                Op::new_start(Node::Document, Content::Span(0..0)),
                Op::new_start(Node::Heading, p.span(4..6)),
                Op::new_value(p.span(6..8)),
                Op::new_end(Node::Heading, Content::Span(0..0)),
                Op::new_end(Node::Document, Content::Span(0..0)),
                Op::new_end(Node::Collapsible, p.span(8..9)),
            ]
        );
    }

    #[test]
    fn only_two_tokens() {
        let mut p = "{% ".into();
        assert!(!collapsible(&mut p));
        assert!(p.ops.is_empty());
        assert_eq!(
            p.peek(),
            Some((
                0,
                &Token::new(TokenKind::Literal, 0..2, Position::default()),
            ))
        );
    }

    #[test]
    fn has_embed() {
        let mut p = "{% Title\n{{foo|bar}}\n%}".into();
        assert!(collapsible(&mut p));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::Collapsible, p.span(0..2)), // {%
                Op::new_start(Node::Modifier, Content::Span(0..0)), //
                Op::new_value(p.span(2..3)),                    // Title
                Op::new_end(Node::Modifier, p.span(3..4)),      // \n
                Op::new_start(Node::Document, Content::Span(0..0)), //
                Op::new_start(Node::Embed, p.span(4..5)),       // {{
                Op::new_value(p.span(5..6)),                    // foo
                Op::new_value(p.span(6..7)),                    // |
                Op::new_value(p.span(7..8)),                    // bar
                Op::new_end(Node::Embed, p.span(8..10)),        // }}\n
                Op::new_end(Node::Document, Content::Span(0..0)),
                Op::new_end(Node::Collapsible, p.span(10..11)), // %}
            ]
        );
    }
}
