use crate::{
    eat_seq,
    lexer::{Token, TokenKind},
    op::{
        Content, Node, Op,
        paragraph::paragraph,
        parser::{ListKind, Parser, StopCondition},
    },
};

fn is_list_marker(t: &Token, kind: Option<ListKind>) -> bool {
    match kind {
        Some(ListKind::Unordered) => t.kind == TokenKind::Minus && t.range.len() == 1,
        Some(ListKind::Ordered) => t.kind == TokenKind::Plus && t.range.len() == 1,
        None => (t.kind == TokenKind::Minus || t.kind == TokenKind::Plus) && t.range.len() == 1,
    }
}

fn is_space(t: &Token) -> bool {
    t.kind == TokenKind::Space && t.range.len() == 1
}

fn list_kind_from_range(p: &Parser, range: &std::ops::Range<usize>) -> Option<ListKind> {
    let token_count = range.end - range.start;
    let marker_idx = if token_count == 2 {
        range.start
    } else {
        range.start + 1
    };
    ListKind::try_from(p.get(marker_idx)?).ok()
}

fn list_item(p: &mut Parser, level: usize, kind: Option<ListKind>) -> Option<ListKind> {
    if !p.at(|t: &Token| t.position.column == 0) {
        return None;
    }

    let start = p.pos;
    let snap = p.ops.len();

    let start_range = if level == 0 {
        eat_seq!(p, |t: &Token| is_list_marker(t, kind), is_space)?
    } else {
        eat_seq!(
            p,
            |t: &Token| t.kind == TokenKind::Space && t.range.len() == level,
            |t: &Token| is_list_marker(t, kind),
            is_space
        )?
    };

    let kind: ListKind = match kind {
        Some(k) => k,
        None => {
            let Some(k) = list_kind_from_range(p, &start_range) else {
                p.pos = start;
                p.ops.truncate(snap);
                return None;
            };
            k
        }
    };

    let start_content = p.span(start_range);
    p.ops.push(Op::new_start(Node::ListItem, start_content));

    p.with_eof(StopCondition::ListBoundary { level, kind }, |p| {
        paragraph(p);
    });

    if !p.at_eof() {
        list_inner(p, level + 1);
    }

    p.ops.push(Op::new_end(Node::ListItem, Content::Span(0..0)));
    Some(kind)
}

fn list_inner(p: &mut Parser, level: usize) -> Option<ListKind> {
    let start = p.pos;
    let snap = p.ops.len();
    let list_kind = list_item(p, level, None)?;

    let list_start_idx = snap;
    p.ops.insert(
        list_start_idx,
        Op::new_start(list_kind.node(), Content::Span(0..0)),
    );

    while list_item(p, level, Some(list_kind)).is_some() {}

    if level == 0 && !p.at_eof() {
        p.pos = start;
        p.ops.truncate(snap);
        return None;
    }

    p.ops
        .push(Op::new_end(list_kind.node(), Content::Span(0..0)));
    Some(list_kind)
}

pub fn list(p: &mut Parser, level: usize) -> bool {
    list_inner(p, level).is_some()
}

#[cfg(test)]
mod tests {
    use crate::op::{Content, Node, Op, list::list, parser::Parser};

    #[test]
    fn parse_unordered() {
        let mut p: Parser = "- level 0\n- level 0".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(4..6)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(6..7)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn parse_ordered() {
        let mut p: Parser = "+ level 0\n+ same level".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(4..6)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(6..7)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn parse_mixed() {
        let mut p: Parser = "+ level 0\n - level 0".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(4..7)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(7..8)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn parse_nested() {
        let mut p: Parser = "- one\n - two".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(4..7)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(7..8)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn eol() {
        let mut p: Parser = "- one\n - two\n something".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(4..7)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(7..11)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn mixed_same_level_ordered() {
        let mut p: Parser = "+ level 0\n- same level".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..7)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn mixed_same_level_unordered() {
        let mut p: Parser = "- level 0\n+ same level".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..7)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn multiple_levels_unordered() {
        let mut p: Parser =
            "- Level 0\n - Level 1\n  - Level 2\n - Level 1\n- Level 0\n - Level 1\n  - Level 2\n- Level 0".into()
        ;

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(4..7)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(7..9)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(9..12)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(12..14)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(14..17)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(17..19)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(19..21)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(21..23)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(23..26)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(26..28)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(28..31)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(31..33)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(33..35)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(35..36)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn multiple_levels_ordered() {
        let mut p: Parser =
            "+ Level 0\n + Level 1\n  + Level 2\n + Level 1\n+ Level 0\n + Level 1\n  + Level 2\n+ Level 0".into()
        ;

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(4..7)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(7..9)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(9..12)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(12..14)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(14..17)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(17..19)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(19..21)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(21..23)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(23..26)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(26..28)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(28..31)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(31..33)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(33..35)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(35..36)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn empty_body() {
        let mut p: Parser = "- ".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn no_nested_ordered_list() {
        let mut p: Parser = "+ level 0\n + ".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(4..7)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
            ]
        );
    }

    #[test]
    fn no_nested_unordered_list() {
        let mut p: Parser = "+ level 0\n - ".into();

        assert!(list(&mut p, 0));
        assert_eq!(
            p.ops,
            vec![
                Op::new_start(Node::OrderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(0..2)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_value(p.span(2..4)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_start(Node::UnorderedList, Content::Span(0..0)),
                Op::new_start(Node::ListItem, p.span(4..7)),
                Op::new_start(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::Paragraph, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::UnorderedList, Content::Span(0..0)),
                Op::new_end(Node::ListItem, Content::Span(0..0)),
                Op::new_end(Node::OrderedList, Content::Span(0..0)),
            ]
        );
    }
}
