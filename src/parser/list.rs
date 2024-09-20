use crate::{
    lexer::TokenKind,
    nodes::{List, ListItem, ListTypes},
};

use super::{paragraph, Parser};

pub(crate) fn list(p: &mut Parser) -> Option<List> {
    let mut state: State = State::Idle;
    let start = p.pos();
    while let Some((t, _)) = p.peek() {
        match t.kind {
            TokenKind::Minus => {
                state = State::NextLevelUnordered;
                p.next_token();
            }
            TokenKind::Plus => {
                state = State::NextLevelOrdered;
                p.next_token();
            }
            TokenKind::Space if state == State::NextLevelUnordered => {
                let Some(l) = parse_list(p, &ListTypes::Unordered, 0) else {
                    break;
                };
                return Some(l);
            }
            TokenKind::Space if state == State::NextLevelOrdered => {
                let Some(l) = parse_list(p, &ListTypes::Ordered, 0) else {
                    break;
                };
                return Some(l);
            }
            _ => {
                break;
            }
        }
    }
    p.move_to(start);
    p.flip_to_literal_at(start);
    None
}

#[derive(PartialEq)]
enum State {
    NextLevel,
    NextLevelOrdered,
    NextLevelUnordered,
    SameLevel,
    SameLevelCommit,
    PreviousLevel,
    PreviousLevelCommit,
    Idle,
}

fn parse_list(p: &mut Parser<'_>, list_type: &ListTypes, level: usize) -> Option<List> {
    let start_pos = p.pos();
    let mut list = List::new(list_type.clone(), level, vec![]);
    let mut list_item = ListItem::new(list_type.clone(), level, vec![], None);
    let mut state: State = State::Idle;

    p.next_token();

    while let Some((t, pos)) = p.peek() {
        match t.kind {
            TokenKind::Terminator => break,
            TokenKind::Space if state == State::Idle && t.slice.len() < level => {
                state = State::PreviousLevel;
                p.next_token();
            }
            TokenKind::Space if state == State::Idle && t.slice.len() == level => {
                state = State::SameLevel;
                p.next_token();
            }
            TokenKind::Space if state == State::Idle && t.slice.len() == level + 1 => {
                state = State::NextLevel;
                p.next_token();
            }
            TokenKind::Minus if t.slice.len() == 1 && state == State::NextLevel => {
                state = State::NextLevelUnordered;
                p.next_token();
            }
            TokenKind::Plus if t.slice.len() == 1 && state == State::NextLevel => {
                state = State::NextLevelOrdered;
                p.next_token();
            }
            TokenKind::Minus
                if t.slice.len() == 1
                    && state == State::SameLevel
                    && list_type == &ListTypes::Unordered =>
            {
                state = State::SameLevelCommit;
                p.next_token();
            }
            TokenKind::Plus
                if t.slice.len() == 1
                    && state == State::SameLevel
                    && list_type == &ListTypes::Ordered =>
            {
                state = State::SameLevelCommit;
                p.next_token();
            }
            TokenKind::Minus if t.slice.len() == 1 && list_type == &ListTypes::Unordered => {
                state = State::SameLevelCommit;
                p.next_token();
            }

            TokenKind::Plus if t.slice.len() == 1 && list_type == &ListTypes::Ordered => {
                state = State::SameLevelCommit;
                p.next_token();
            }
            TokenKind::Minus if t.slice.len() == 1 && state == State::PreviousLevel => {
                state = State::PreviousLevelCommit;
                p.next_token();
            }
            TokenKind::Plus if t.slice.len() == 1 && state == State::PreviousLevel => {
                state = State::PreviousLevelCommit;
                p.next_token();
            }
            TokenKind::Space if state == State::NextLevelUnordered => {
                state = State::Idle;
                if let Some(nested_list) = parse_list(p, &ListTypes::Unordered, level + 1) {
                    list_item.nested_list.replace(nested_list);
                    list.body.push(list_item);
                    list_item = ListItem::new(list_type.clone(), level, vec![], None);
                } else {
                    p.next_token();
                }
            }
            TokenKind::Space if state == State::NextLevelOrdered => {
                state = State::Idle;
                if let Some(nested_list) = parse_list(p, &ListTypes::Ordered, level + 1) {
                    list_item.nested_list.replace(nested_list);
                    list.body.push(list_item);
                    list_item = ListItem::new(list_type.clone(), level, vec![], None);
                } else {
                    p.next_token();
                }
            }
            TokenKind::Space if state == State::SameLevelCommit => {
                state = State::Idle;
                list.body.push(list_item);
                list_item = ListItem::new(list_type.clone(), level, vec![], None);
                p.next_token();
            }
            TokenKind::Space if state == State::PreviousLevelCommit => {
                // back to EOL so Previous level can decide what to do.
                p.move_to(pos - 3);
                break;
            }
            _ => {
                state = State::Idle;
                if let Some(mut n) = paragraph(p, |t| {
                    t.kind == TokenKind::Space
                        || t.kind == TokenKind::Plus
                        || t.kind == TokenKind::Minus
                }) {
                    list_item.text.append(&mut n.body);
                }
            }
        }
    }

    if !list_item.text.is_empty() {
        list.body.push(list_item);
    }

    if list.body.is_empty() {
        p.move_to(start_pos);
        p.flip_to_literal_at(start_pos);
        return None;
    }
    Some(list)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        nodes::{List, ListItem, ListTypes},
        parser::{list, Parser},
    };

    #[test]
    fn parse_unordered() {
        let mut p = Parser::new("- level 0\n- level 0");

        assert_eq!(
            list(&mut p),
            Some(List::new(
                ListTypes::Unordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        vec![String::from("level 0").into()],
                        None
                    ),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        vec![String::from("level 0").into()],
                        None
                    )
                ],
            ))
        );
    }

    #[test]
    fn parse_ordered() {
        let mut p = Parser::new("+ level 0\n+ same level");

        assert_eq!(
            list(&mut p),
            Some(List::new(
                ListTypes::Ordered,
                0,
                vec![
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        vec![String::from("level 0").into()],
                        None
                    ),
                    ListItem::new(
                        ListTypes::Ordered,
                        0,
                        vec![String::from("same level").into()],
                        None
                    ),
                ],
            ))
        );
    }

    #[test]
    fn parse_mixed() {
        let mut p = Parser::new("+ level 0\n - level 0");

        let list_node = List::new(
            ListTypes::Ordered,
            0,
            vec![ListItem::new(
                ListTypes::Ordered,
                0,
                vec![String::from("level 0").into()],
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        vec![String::from("level 0").into()],
                        None,
                    )],
                )),
            )
            .into()],
        );

        assert_eq!(list(&mut p), Some(list_node));
    }

    #[test]
    fn parse_nested() {
        let input = r#"- one
 - two"#;
        let mut p = Parser::new(input);

        let list_node = List::new(
            ListTypes::Unordered,
            0,
            vec![ListItem::new(
                ListTypes::Unordered,
                0,
                vec![String::from("one").into()],
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        vec![String::from("two").into()],
                        None,
                    )],
                )),
            )],
        );

        assert_eq!(list(&mut p), Some(list_node));
    }

    #[test]
    fn eol() {
        let input = r#"- one
 - two
something"#;
        let mut p = Parser::new(input);

        let list_node = List::new(
            ListTypes::Unordered,
            0,
            vec![ListItem::new(
                ListTypes::Unordered,
                0,
                vec![String::from("one").into()],
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        vec![String::from("two\nsomething").into()],
                        None,
                    )],
                )),
            )],
        );

        assert_eq!(list(&mut p), Some(list_node));
    }

    #[test]
    fn mixed_same_level_ordered() {
        let mut p = Parser::new("+ level 0\n- same level");

        assert_eq!(
            list(&mut p),
            Some(List::new(
                ListTypes::Ordered,
                0,
                vec![ListItem::new(
                    ListTypes::Ordered,
                    0,
                    vec![
                        String::from("level 0").into(),
                        String::from("- same level").into()
                    ],
                    None
                ),],
            ))
        );
    }

    #[test]
    fn mixed_same_level_unordered() {
        let mut p = Parser::new("- level 0\n+ same level");

        assert_eq!(
            list(&mut p),
            Some(List::new(
                ListTypes::Unordered,
                0,
                vec![ListItem::new(
                    ListTypes::Unordered,
                    0,
                    vec![
                        String::from("level 0").into(),
                        String::from("+ same level").into()
                    ],
                    None
                ),],
            ))
        );
    }
}
