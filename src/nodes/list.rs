use std::{fmt::Display, usize};

use serde::Serialize;

use crate::toolkit::{context::Context, parser::Parse};

use super::{list_item::ListItem, paragraph::Paragraph};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ListTypes {
    Unordered,
    Ordered,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct List {
    pub list_type: ListTypes,
    pub level: usize,
    pub nodes: Vec<ListItem>,
}

impl List {
    pub fn new(list_type: ListTypes, level: usize, nodes: Vec<ListItem>) -> Self {
        Self {
            list_type,
            level,
            nodes,
        }
    }

    fn get_text_slice_and_nested_list<'a>(&self, input: &'a str) -> (&'a str, Option<List>) {
        if let Some((left, right)) = input.split_once('\n') {
            let mut ctx = Context::new();
            ctx.add("level", self.level + 1);
            if let Some((list, consumed)) = List::parse(right, 0, Some(&ctx)) {
                if consumed == right.len() {
                    return (left, Some(list));
                }
            }
        }
        return (input, None);
    }

    fn parse_list_items(&mut self, input: &str) -> usize {
        let mut end = 2 + self.level;
        while end < input.len() {
            let list_type = match self.list_type {
                ListTypes::Unordered => '-',
                ListTypes::Ordered => '+',
            };
            let new_position = input[end..]
                .find(format!("\n{}{} ", " ".repeat(self.level), list_type).as_str())
                .map_or(input.len(), |pos| pos + end);

            let (text_slice, nested_list) =
                self.get_text_slice_and_nested_list(&input[end..new_position]);

            self.nodes.push(ListItem::new(
                self.list_type.clone(),
                self.level,
                Paragraph::parse(text_slice, 0, None)
                    .map(|(paragraph, _)| paragraph)
                    .expect("paragraph should always succeed"),
                nested_list,
            ));

            end = if new_position == input.len() {
                new_position
            } else {
                new_position + 3 + self.level
            };
        }
        end
    }
}

impl Parse for List {
    fn parse(input: &str, current_position: usize, ctx: Option<&Context>) -> Option<(Self, usize)> {
        let level = match ctx {
            Some(ctx) => ctx.get_usize_value("level").unwrap_or(0),
            None => 0,
        };

        if input[current_position..].starts_with(format!("{}- ", " ".repeat(level)).as_str()) {
            let end = input[current_position..]
                .find("\n\n")
                .map_or(input.len(), |pos| pos + current_position);
            let mut list = List::new(ListTypes::Unordered, level, vec![]);
            let end = list.parse_list_items(&input[current_position..end]);
            return Some((list, end));
        }

        if input[current_position..].starts_with(format!("{}+ ", " ".repeat(level)).as_str()) {
            let mut list = List::new(ListTypes::Ordered, level, vec![]);
            let end = list.parse_list_items(input);
            return Some((list, end));
        }

        None
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{List, ListTypes};
    use crate::{
        nodes::{list_item::ListItem, paragraph::Paragraph, text::Text},
        toolkit::{context::Context, parser::Parse},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_unordered() {
        assert_eq!(
            List {
                list_type: ListTypes::Unordered,
                level: 0,
                nodes: vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        Paragraph::new(vec![Text::new("unordered list item").into()],),
                        None
                    ),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        Paragraph::new(vec![Text::new("unordered list item").into()],),
                        None
                    )
                ],
            }
            .to_string(),
            "- unordered list item\n- unordered list item"
        );
        assert_eq!(
            List {
                list_type: ListTypes::Unordered,
                level: 0,
                nodes: vec![
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        Paragraph::new(vec![Text::new("unordered list item").into()],),
                        None
                    ),
                    ListItem::new(
                        ListTypes::Unordered,
                        0,
                        Paragraph::new(vec![Text::new("unordered list item").into()],),
                        None
                    )
                ],
            }
            .to_string(),
            "- unordered list item\n- unordered list item"
        );
    }

    #[test]
    fn serialize_ordered() {
        let list = List::new(
            ListTypes::Ordered,
            0,
            vec![
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    Paragraph::new(vec![Text::new("ordered list item").into()]),
                    None,
                ),
                ListItem::new(
                    ListTypes::Ordered,
                    0,
                    Paragraph::new(vec![Text::new("ordered list item").into()]),
                    None,
                ),
            ],
        );

        assert_eq!(list.to_string(), "+ ordered list item\n+ ordered list item");
    }

    #[test]
    fn parse_wrong_level() {
        let mut ctx = Context::new();
        ctx.add("level", 1);
        assert_eq!(List::parse("- level 0\n- level 0", 0, Some(&ctx)), None);
    }

    #[test]
    fn parse_unordered() {
        assert_eq!(
            List::parse("- level 0\n- level 0", 0, None),
            Some((
                List::new(
                    ListTypes::Unordered,
                    0,
                    vec![
                        ListItem::new(
                            ListTypes::Unordered,
                            0,
                            Paragraph::new(vec![Text::new("level 0").into()]),
                            None
                        ),
                        ListItem::new(
                            ListTypes::Unordered,
                            0,
                            Paragraph::new(vec![Text::new("level 0").into()]),
                            None
                        )
                    ],
                ),
                19
            ))
        );
    }

    #[test]
    fn parse_ordered() {
        assert_eq!(
            List::parse("+ level 0\n+ level 0", 0, None),
            Some((
                List::new(
                    ListTypes::Ordered,
                    0,
                    vec![
                        ListItem::new(
                            ListTypes::Ordered,
                            0,
                            Paragraph::new(vec![Text::new("level 0").into()]),
                            None
                        ),
                        ListItem::new(
                            ListTypes::Ordered,
                            0,
                            Paragraph::new(vec![Text::new("level 0").into()]),
                            None
                        ),
                    ],
                ),
                19
            ))
        );
    }

    #[test]
    fn parse_mixed() {
        let list = List::new(
            ListTypes::Ordered,
            0,
            vec![ListItem::new(
                ListTypes::Ordered,
                0,
                Paragraph::new(vec![Text::new("level 0").into()]),
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        Paragraph::new(vec![Text::new("level 0").into()]),
                        None,
                    )],
                )),
            )
            .into()],
        );

        assert_eq!(
            List::parse("+ level 0\n - level 0", 0, None),
            Some((list, 20))
        );
    }

    #[test]
    fn parsed_nested() {
        let list = List::new(
            ListTypes::Unordered,
            0,
            vec![ListItem::new(
                ListTypes::Unordered,
                0,
                Paragraph::new(vec![Text::new("one").into()]).into(),
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        Paragraph::new(vec![Text::new("two").into()]),
                        None,
                    )],
                )),
            )],
        );

        let input = r#"- one
 - two"#;
        assert_eq!(List::parse(input, 0, None), Some((list, 12)));
    }

    #[test]
    fn parse_nested() {
        let input = r#"- one
 - two
something"#;
        let list = List::new(
            ListTypes::Unordered,
            0,
            vec![ListItem::new(
                ListTypes::Unordered,
                0,
                Paragraph::new(vec![Text::new("one").into()]).into(),
                Some(List::new(
                    ListTypes::Unordered,
                    1,
                    vec![ListItem::new(
                        ListTypes::Unordered,
                        1,
                        Paragraph::new(vec![Text::new("two\nsomething").into()]),
                        None,
                    )],
                )),
            )],
        );

        assert_eq!(List::parse(input, 0, None), Some((list, input.len())));
    }
}
