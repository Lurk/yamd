use crate::{
    eat_seq,
    lexer::{Token, TokenKind},
    op::{
        Content, Node, Op, OpKind,
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

fn try_get_list_kind(ops: &[Op]) -> Option<ListKind> {
    let start_op = ops.first()?;
    if start_op.kind != OpKind::Start(Node::ListItem) {
        return None;
    }
    let Content::Source(tokens) = &start_op.content else {
        return None;
    };
    let token = tokens.get(if tokens.len() == 2 { 0 } else { 1 })?;
    ListKind::try_from(token).ok()
}

fn list_item(p: &Parser, level: usize, kind: Option<ListKind>) -> Option<Vec<Op>> {
    let _start = p.pos();

    if !p.at(|t: &Token| t.position.column == 0) {
        return None;
    }

    let start_tokens = if level == 0 {
        eat_seq!(p, |t: &Token| is_list_marker(t, kind), is_space)?
    } else {
        eat_seq!(
            p,
            |t: &Token| t.kind == TokenKind::Space && t.range.len() == level,
            |t: &Token| is_list_marker(t, kind),
            is_space
        )?
    };

    let mut ops = vec![Op::new_start(Node::ListItem, start_tokens)];

    let kind: ListKind = match kind {
        Some(k) => k,
        None => try_get_list_kind(&ops).expect("List kind should be determined here"),
    };

    {
        let _g = p.push_eof(StopCondition::ListBoundary { level, kind });
        let paragraph_ops = paragraph(p);
        ops.extend(paragraph_ops);
    }

    if p.at_eof() {
        ops.push(Op::new_end(Node::ListItem, &[]));
        return Some(ops);
    } else if let Some(nested_ops) = list(p, level + 1) {
        ops.extend(nested_ops);
        ops.push(Op::new_end(Node::ListItem, &[]));
        return Some(ops);
    }

    ops.push(Op::new_end(Node::ListItem, &[]));
    Some(ops)
}

pub fn list(p: &Parser, level: usize) -> Option<Vec<Op>> {
    let start = p.pos();
    let first_list_item_ops = list_item(p, level, None)?;

    let Some(list_kind) = try_get_list_kind(&first_list_item_ops) else {
        p.replace_position(start);
        return None;
    };

    let mut ops = vec![Op::new_start(list_kind.node(), &[])];
    ops.extend(first_list_item_ops);

    while let Some(nested_list_item_ops) = list_item(p, level, Some(list_kind)) {
        ops.extend(nested_list_item_ops);
    }

    if level == 0 && !p.at_eof() {
        p.replace_position(start);
        return None;
    }

    ops.push(Op::new_end(list_kind.node(), &[]));
    Some(ops)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::op::{Node, Op, list::list, parser::Parser};

    #[test]
    fn parse_unordered() {
        let p: Parser = "- level 0\n- level 0".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..4)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_start(Node::ListItem, p.slice(4..6)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(6..7)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
            ])
        );
    }

    #[test]
    fn parse_ordered() {
        let p: Parser = "+ level 0\n+ same level".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..4)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_start(Node::ListItem, p.slice(4..6)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(6..7)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
            ])
        );
    }

    #[test]
    fn parse_mixed() {
        let p: Parser = "+ level 0\n - level 0".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..4)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(4..7)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(7..8)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
            ])
        );
    }

    #[test]
    fn parse_nested() {
        let p: Parser = "- one\n - two".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..4)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(4..7)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(7..8)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
            ])
        );
    }

    #[test]
    fn eol() {
        let p: Parser = "- one\n - two\n something".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..4)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(4..7)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(7..11)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
            ])
        );
    }

    #[test]
    fn mixed_same_level_ordered() {
        let p = "+ level 0\n- same level".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..7)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
            ])
        );
    }

    #[test]
    fn mixed_same_level_unordered() {
        let p: Parser = "- level 0\n+ same level".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..7)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
            ])
        );
    }

    #[test]
    fn multiple_levels_unordered() {
        let p: Parser =
            "- Level 0\n - Level 1\n  - Level 2\n - Level 1\n- Level 0\n - Level 1\n  - Level 2\n- Level 0".into()
        ;

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..4)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(4..7)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(7..9)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(9..12)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(12..14)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_start(Node::ListItem, p.slice(14..17)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(17..19)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_start(Node::ListItem, p.slice(19..21)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(21..23)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(23..26)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(26..28)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(28..31)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(31..33)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_start(Node::ListItem, p.slice(33..35)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(35..36)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
            ])
        );
    }

    #[test]
    fn multiple_levels_ordered() {
        let p: Parser =
            "+ Level 0\n + Level 1\n  + Level 2\n + Level 1\n+ Level 0\n + Level 1\n  + Level 2\n+ Level 0".into()
        ;

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..4)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(4..7)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(7..9)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(9..12)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(12..14)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_start(Node::ListItem, p.slice(14..17)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(17..19)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_start(Node::ListItem, p.slice(19..21)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(21..23)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(23..26)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(26..28)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(28..31)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(31..33)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_start(Node::ListItem, p.slice(33..35)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(35..36)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
            ])
        );
    }

    #[test]
    fn empty_body() {
        let p: Parser = "- ".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
            ])
        );
    }

    #[test]
    fn no_nested_ordered_list() {
        let p = "+ level 0\n + ".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..4)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(4..7)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
            ])
        );
    }

    #[test]
    fn no_nested_unordered_list() {
        let p: Parser = "+ level 0\n - ".into();

        assert_eq!(
            list(&p, 0),
            Some(vec![
                Op::new_start(Node::OrderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(0..2)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_value(p.slice(2..4)),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_start(Node::UnorderedList, &[]),
                Op::new_start(Node::ListItem, p.slice(4..7)),
                Op::new_start(Node::Paragraph, &[]),
                Op::new_end(Node::Paragraph, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::UnorderedList, &[]),
                Op::new_end(Node::ListItem, &[]),
                Op::new_end(Node::OrderedList, &[]),
            ])
        );
    }
}
