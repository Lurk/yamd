use crate::{
    and, is, join,
    lexer::{Token, TokenKind},
    op::{
        op::{Node, Op, OpKind},
        paragraph::paragraph,
        parser::{Condition, Parser, Query},
    },
    or,
};

fn space<'a>(len: usize) -> impl Fn(&'a Token<'a>) -> bool {
    move |token: &'a Token| token.kind == TokenKind::Space && token.slice.len() == len
}

fn one_minus(token: &Token) -> bool {
    token.kind == TokenKind::Minus && token.slice.len() == 1
}

fn one_plus(token: &Token) -> bool {
    token.kind == TokenKind::Plus && token.slice.len() == 1
}

fn list_start(level: usize, query: Query, with_first_column_check: bool) -> Query {
    let q = if level == 0 {
        join!(query, is!(t = TokenKind::Space, el = 1,))
    } else {
        join!(
            is!(t = TokenKind::Space, el = level,),
            query,
            is!(t = TokenKind::Space, el = 1,)
        )
    };

    if with_first_column_check {
        and!(is!(c = 0,), q)
    } else {
        q
    }
}

#[derive(Debug, Clone, Copy)]
enum ListKind {
    Unordered,
    Ordered,
}

impl ListKind {
    fn node(&self) -> Node {
        match self {
            ListKind::Unordered => Node::UnorderedList,
            ListKind::Ordered => Node::OrderedList,
        }
    }

    fn query_is(&self) -> Query {
        let c = match self {
            ListKind::Unordered => Condition::new().kind(TokenKind::Minus),
            ListKind::Ordered => Condition::new().kind(TokenKind::Plus),
        };

        Query::Is(c.exact_len(1))
    }

    fn query_or() -> Query {
        let unordered = Condition::new().kind(TokenKind::Minus).exact_len(1);
        let ordered = Condition::new().kind(TokenKind::Plus).exact_len(1);

        or!(Query::Is(unordered), Query::Is(ordered))
    }
}

impl TryFrom<&Token<'_>> for ListKind {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        if one_minus(value) {
            Ok(ListKind::Unordered)
        } else if one_plus(value) {
            Ok(ListKind::Ordered)
        } else {
            Err(())
        }
    }
}

fn is_one_of_allowed_list_levels(current_level: usize, max_level: usize, kind: ListKind) -> Query {
    let q = (0..=max_level).fold(vec![], |mut acc, i| {
        if current_level == i {
            acc.push(list_start(i, kind.query_is(), false));
        } else {
            acc.push(list_start(i, ListKind::query_or(), false));
        }
        acc
    });

    Query::And(vec![Query::Is(Condition::new().column(0)), Query::Or(q)])
}

fn try_get_list_kind(ops: &[Op]) -> Option<ListKind> {
    let start_op = ops.first()?;
    if start_op.kind != OpKind::Start(Node::ListItem) {
        return None;
    }

    let token = start_op
        .tokens
        .get(if start_op.tokens.len() == 2 { 0 } else { 1 })?;

    ListKind::try_from(*token).ok()
}

fn list_item<'a>(
    p: &'a Parser<'a>,
    level: usize,
    kind: Option<ListKind>,
    eof: &Query,
) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let query = match kind {
        Some(k) => k.query_is(),
        None => ListKind::query_or(),
    };

    let mut ops = vec![Op {
        kind: OpKind::Start(Node::ListItem),
        tokens: Vec::from_iter(p.chain(&list_start(level, query, true), false)?),
    }];

    let kind: ListKind = match kind {
        Some(k) => k,
        None => try_get_list_kind(&ops).expect("List kind should be determined here"),
    };

    let paragraph_ops = paragraph(
        p,
        &or!(
            eof.clone(),
            is_one_of_allowed_list_levels(level, level + 1, kind)
        ),
    );

    ops.extend(paragraph_ops);

    if p.chain(eof, false).is_some() {
        ops.push(Op {
            kind: OpKind::End(Node::ListItem),
            tokens: vec![],
        });
        return Some(ops);
    } else if let Some(nested_ops) = list(p, level + 1, eof) {
        ops.extend(nested_ops);
        ops.push(Op {
            kind: OpKind::End(Node::ListItem),
            tokens: vec![],
        });
        return Some(ops);
    } else if p
        .chain(&is_one_of_allowed_list_levels(level, level, kind), true)
        .is_none()
    {
        ops.push(Op {
            kind: OpKind::End(Node::ListItem),
            tokens: vec![],
        });
        return Some(ops);
    }
    p.replace_position(start);
    None
}

pub fn list<'a>(p: &'a Parser<'a>, level: usize, eof: &Query) -> Option<Vec<Op<'a>>> {
    let start = p.pos();
    let first_list_item_ops = list_item(p, level, None, eof)?;

    let Some(list_kind) = try_get_list_kind(&first_list_item_ops) else {
        p.replace_position(start);
        return None;
    };

    let mut ops = vec![Op {
        kind: OpKind::Start(list_kind.node()),
        tokens: vec![],
    }];

    ops.extend(first_list_item_ops);

    while let Some(nested_list_item_ops) = list_item(p, level, Some(list_kind), eof) {
        ops.extend(nested_list_item_ops);
    }

    if level == 0 && p.chain(eof, false).is_none() {
        p.replace_position(start);
        return None;
    }

    ops.push(Op {
        kind: OpKind::End(list_kind.node()),
        tokens: vec![],
    });

    Some(ops)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::op::{
        Op,
        list::list,
        op::Node,
        parser::{Parser, Query},
    };

    #[test]
    fn parse_unordered() {
        let p: Parser = "- level 0\n- level 0".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::UnorderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(2..4))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(4..6))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(6..7))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
            ])
        );
    }

    #[test]
    fn parse_ordered() {
        let p: Parser = "+ level 0\n+ same level".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::OrderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(2..4))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(4..6))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(6..7))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
            ])
        );
    }

    #[test]
    fn parse_mixed() {
        let p: Parser = "+ level 0\n - level 0".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::OrderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(2..4))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::UnorderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(4..7))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(7..8))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
            ])
        );
    }

    #[test]
    fn parse_nested() {
        let p: Parser = "- one\n - two".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::UnorderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(2..4))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::UnorderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(4..7))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(7..8))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
            ])
        );
    }

    #[test]
    fn eol() {
        let p: Parser = "- one\n - two\n something".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::UnorderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(2..4))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::UnorderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(4..7))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(7..11))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
            ])
        );
    }

    #[test]
    fn mixed_same_level_ordered() {
        let p = "+ level 0\n- same level".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::OrderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(2..7))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
            ])
        );
    }

    #[test]
    fn mixed_same_level_unordered() {
        let p: Parser = "- level 0\n+ same level".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::UnorderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(2..7))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
            ])
        );
    }

    #[test]
    fn multiple_levels_unordered() {
        let p: Parser =
            "- Level 0\n - Level 1\n  - Level 2\n - Level 1\n- Level 0\n - Level 1\n  - Level 2\n- Level 0".into()
        ;

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::UnorderedList, vec![]),
                // -
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 0\n
                Op::new_value(Vec::from_iter(p.slice(2..4))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::UnorderedList, vec![]),
                //  -
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(4..7))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 1\n
                Op::new_value(Vec::from_iter(p.slice(7..9))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::UnorderedList, vec![]),
                //   -
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(9..12))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 2\n
                Op::new_value(Vec::from_iter(p.slice(12..14))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                //  -
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(14..17))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 1\n
                Op::new_value(Vec::from_iter(p.slice(17..19))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                // -
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(19..21))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 0\n
                Op::new_value(Vec::from_iter(p.slice(21..23))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::UnorderedList, vec![]),
                //  -
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(23..26))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 1\n
                Op::new_value(Vec::from_iter(p.slice(26..28))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::UnorderedList, vec![]),
                //   -
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(28..31))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 2\n
                Op::new_value(Vec::from_iter(p.slice(31..33))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                // -
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(33..35))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 0
                Op::new_value(Vec::from_iter(p.slice(35..36))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
            ])
        );
    }

    #[test]
    fn multiple_levels_ordered() {
        let p: Parser =
            "+ Level 0\n + Level 1\n  + Level 2\n + Level 1\n+ Level 0\n + Level 1\n  + Level 2\n+ Level 0".into()
        ;

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::OrderedList, vec![]),
                // +
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 0\n
                Op::new_value(Vec::from_iter(p.slice(2..4))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::OrderedList, vec![]),
                //  +
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(4..7))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 1\n
                Op::new_value(Vec::from_iter(p.slice(7..9))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::OrderedList, vec![]),
                //   +
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(9..12))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 2\n
                Op::new_value(Vec::from_iter(p.slice(12..14))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                //  +
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(14..17))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 1\n
                Op::new_value(Vec::from_iter(p.slice(17..19))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                // +
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(19..21))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 0\n
                Op::new_value(Vec::from_iter(p.slice(21..23))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::OrderedList, vec![]),
                //  +
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(23..26))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 1\n
                Op::new_value(Vec::from_iter(p.slice(26..28))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::OrderedList, vec![]),
                //   +
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(28..31))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 2\n
                Op::new_value(Vec::from_iter(p.slice(31..33))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                // +
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(33..35))),
                Op::new_start(Node::Paragraph, vec![]),
                // Level 0
                Op::new_value(Vec::from_iter(p.slice(35..36))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
            ])
        );
    }

    #[test]
    fn empty_body() {
        let p: Parser = "- ".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::UnorderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
            ])
        );
    }

    #[test]
    fn no_nested_ordered_list() {
        let p = "+ level 0\n + ".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::OrderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(2..4))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::OrderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(4..7))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
            ])
        );
    }

    #[test]
    fn no_nested_unordered_list() {
        let p: Parser = "+ level 0\n - ".into();

        assert_eq!(
            list(&p, 0, &Query::Eof),
            Some(vec![
                Op::new_start(Node::OrderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(0..2))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_value(Vec::from_iter(p.slice(2..4))),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_start(Node::UnorderedList, vec![]),
                Op::new_start(Node::ListItem, Vec::from_iter(p.slice(4..7))),
                Op::new_start(Node::Paragraph, vec![]),
                Op::new_end(Node::Paragraph, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::UnorderedList, vec![]),
                Op::new_end(Node::ListItem, vec![]),
                Op::new_end(Node::OrderedList, vec![]),
            ])
        );
    }
}
