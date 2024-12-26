use crate::{
    lexer::{Token, TokenKind},
    nodes::{ListTypes, ThematicBreak, Yamd},
};

use super::{
    code, collapsible, embed, heading, highlight, images, list, metadata, paragraph, Parser,
};

pub(crate) fn yamd<Callback>(p: &mut Parser, f: Callback) -> Yamd
where
    Callback: Fn(&Token) -> bool,
{
    let mut yamd = Yamd::new(None, vec![]);

    while let Some((t, pos)) = p.peek() {
        match t.kind {
            TokenKind::Terminator => {
                p.next_token();
            }
            TokenKind::LeftCurlyBrace if t.slice.len() == 2 => {
                if let Some(e) = embed(p) {
                    yamd.body.push(e.into());
                }
            }
            TokenKind::Minus if t.slice.len() == 1 => {
                if let Some(l) = list(p, &ListTypes::Unordered) {
                    yamd.body.push(l.into())
                }
            }
            TokenKind::Plus if t.slice.len() == 1 => {
                if let Some(l) = list(p, &ListTypes::Ordered) {
                    yamd.body.push(l.into())
                }
            }
            TokenKind::Minus if t.slice.len() == 3 && pos == 0 => {
                yamd.metadata = metadata(p);
            }
            TokenKind::Minus if t.slice.len() == 5 => {
                yamd.body.push(ThematicBreak::new().into());
                p.next_token();
            }
            TokenKind::Hash if t.slice.len() < 7 => {
                if let Some(h) = heading(p, &f) {
                    yamd.body.push(h.into());
                }
            }
            TokenKind::Bang if t.slice.len() == 2 => {
                if let Some(h) = highlight(p) {
                    yamd.body.push(h.into());
                }
            }
            TokenKind::Bang if t.slice.len() == 1 => {
                if let Some(mut i) = images(p, &f) {
                    if i.body.len() == 1 {
                        yamd.body.push(i.body.swap_remove(0).into());
                    } else {
                        yamd.body.push(i.into());
                    }
                }
            }
            TokenKind::Backtick if t.slice.len() == 3 => {
                if let Some(c) = code(p) {
                    yamd.body.push(c.into())
                }
            }
            TokenKind::CollapsibleStart => {
                if let Some(n) = collapsible(p) {
                    yamd.body.push(n.into());
                }
            }
            _ if t.position.column == 0 && f(t) => break,
            _ => {
                if let Some(n) = paragraph(p, &f) {
                    yamd.body.push(n.into());
                }
            }
        };
    }

    yamd
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        nodes::{
            Bold, Code, Collapsible, Embed, Heading, Highlight, Image, Images, Italic, List,
            ListItem, ListTypes, Paragraph, Strikethrough, ThematicBreak, Yamd,
        },
        parser::{yamd, Parser},
    };

    const TEST_CASE: &str = r#"---
title: test
date: 2022-01-01T00:00:00+02:00
image: image
preview: preview
tags:
- tag1
- tag2
---

# hello

```rust
let a=1;
```

t**b**

![a](u)

![a](u)
![a2](u2)

!! H
! I
~~s~~

_I_
!!

-----

- one
 - two

+ first
 + second

{{youtube|123}}

{{cloudinary_gallery|cloud_name&tag}}

{% collapsible

%}

{% one more collapsible

%}

+

-

![](

```

end"#;

    #[test]
    fn parse() {
        let mut p = Parser::new(TEST_CASE);
        assert_eq!(
            yamd(&mut p, |_| false),
            Yamd::new(
                Some(String::from(
                    "title: test\ndate: 2022-01-01T00:00:00+02:00\nimage: image\npreview: preview\ntags:\n- tag1\n- tag2"
                )),
                    vec![
                        Heading::new(1, vec![String::from("hello").into()]).into(),
                        Code::new("rust", "let a=1;").into(),
                        Paragraph::new(vec![
                            String::from("t").into(),
                            Bold::new(vec![String::from("b").into()]).into()
                        ])
                        .into(),
                        Image::new('a', 'u').into(),
                        Images::new(vec![
                            Image::new("a", "u"),
                            Image::new("a2", "u2")
                        ],)
                        .into(),
                        Highlight::new(
                            Some("H"),
                            Some("I"),
                            vec![
                                Paragraph::new(vec![Strikethrough::new("s").into()]),
                                Paragraph::new(vec![Italic::new("I").into()])
                            ]
                        )
                        .into(),
                        ThematicBreak::new().into(),
                        List::new(
                            ListTypes::Unordered,
                            0,
                            vec![ListItem::new(
                                vec![String::from("one").into()],
                                Some(List::new(
                                    ListTypes::Unordered,
                                    1,
                                    vec![ListItem::new(
                                    vec![String::from("two").into()],
                                        None
                                    )]
                                ))
                            )]
                        )
                        .into(),
                        List::new(
                            ListTypes::Ordered,
                            0,
                            vec![ListItem::new(
                                vec![String::from("first").into()],
                                Some(List::new(
                                    ListTypes::Ordered,
                                    1,
                                    vec![ListItem::new(
                                    vec![String::from("second").into()],
                                        None
                                    )]
                                ))
                            )]
                        )
                        .into(),
                        Embed::new("youtube", "123",).into(),
                        Embed::new("cloudinary_gallery", "cloud_name&tag",).into(),
                        Collapsible::new("collapsible", vec![]).into(),
                        Collapsible::new("one more collapsible", vec![]).into(),
                        Paragraph::new(vec![String::from("+").into()]).into(),
                        Paragraph::new(vec![String::from("-").into()]).into(),
                        Paragraph::new(vec![String::from("![](").into()]).into(),
                        Paragraph::new(vec![String::from("```").into()]).into(),
                        Paragraph::new(vec![String::from("end").into()]).into()
                    ]
                ),
        );
    }

    #[test]
    fn default() {
        assert_eq!(Yamd::default(), Yamd::new(None, vec![]));
    }

    #[test]
    fn multiple_fallbacks_in_a_row() {
        let mut p = Parser::new("1\n\n2\n\n3");
        let expected = Yamd::new(
            None,
            vec![
                Paragraph::new(vec![String::from("1").into()]).into(),
                Paragraph::new(vec![String::from("2").into()]).into(),
                Paragraph::new(vec![String::from("3").into()]).into(),
            ],
        );
        assert_eq!(yamd(&mut p, |_| false), expected);
    }

    #[test]
    fn multiple_fallbacks_in_a_row_before_non_fallback() {
        let mut p = Parser::new("1\n\n2\n\n3\n\n# header");
        let expected = Yamd::new(
            None,
            vec![
                Paragraph::new(vec![String::from("1").into()]).into(),
                Paragraph::new(vec![String::from("2").into()]).into(),
                Paragraph::new(vec![String::from("3").into()]).into(),
                Heading::new(1, vec![String::from("header").into()]).into(),
            ],
        );
        assert_eq!(yamd(&mut p, |_| false), expected);
    }

    #[test]
    fn node_should_start_from_delimiter() {
        let mut p = Parser::new("text - text");
        let expected = Yamd::new(
            None,
            vec![Paragraph::new(vec![String::from("text - text").into()]).into()],
        );
        assert_eq!(yamd(&mut p, |_| false), expected);
    }

    #[test]
    fn last_delimiter() {
        let mut p = Parser::new("text\n\n");
        let expected = Yamd::new(
            None,
            vec![Paragraph::new(vec![String::from("text").into()]).into()],
        );
        assert_eq!(yamd(&mut p, |_| false), expected);
    }
}
