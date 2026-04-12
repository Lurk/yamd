use crate::nodes::{
    Anchor, Bold, BoldNodes, Code, CodeSpan, Collapsible, Embed, Emphasis, Heading, HeadingNodes,
    Highlight, Image, Images, Italic, List, ListItem, ListTypes, Paragraph, ParagraphNodes,
    Strikethrough, ThematicBreak, Yamd, YamdNodes,
};
use crate::op::{Content, Node, Op, OpKind};

enum Frame {
    Yamd {
        metadata: Option<String>,
        body: Vec<YamdNodes>,
    },
    Document {
        children: Vec<YamdNodes>,
    },
    Heading {
        level: u8,
        body: Vec<HeadingNodes>,
    },
    Paragraph {
        body: Vec<ParagraphNodes>,
    },
    Bold {
        body: Vec<BoldNodes>,
    },
    Italic {
        text: String,
    },
    Strikethrough {
        text: String,
    },
    CodeSpan {
        text: String,
    },
    Emphasis {
        text: String,
    },
    Anchor {
        text: String,
        url: String,
    },
    Title {
        text: String,
    },
    Destination {
        text: String,
    },
    Image {
        alt: String,
        src: String,
    },
    Images {
        images: Vec<Image>,
    },
    Code {
        lang: String,
        code: String,
    },
    Modifier {
        text: String,
    },
    Embed {
        values: Vec<String>,
    },
    ThematicBreak,
    Highlight {
        title: Option<String>,
        icon: Option<String>,
        paragraphs: Vec<Paragraph>,
    },
    Icon {
        text: String,
    },
    Collapsible {
        title: String,
        body: Vec<YamdNodes>,
    },
    UnorderedList {
        level: usize,
        items: Vec<ListItem>,
    },
    OrderedList {
        level: usize,
        items: Vec<ListItem>,
    },
    ListItem {
        text: Vec<ParagraphNodes>,
        nested_list: Option<List>,
    },
    Metadata {
        text: String,
    },
}

impl Frame {
    fn from_node(node: &Node) -> Self {
        match node {
            Node::Document => Frame::Document {
                children: Vec::new(),
            },
            Node::Paragraph => Frame::Paragraph { body: Vec::new() },
            Node::Bold => Frame::Bold { body: Vec::new() },
            Node::Italic => Frame::Italic {
                text: String::new(),
            },
            Node::Strikethrough => Frame::Strikethrough {
                text: String::new(),
            },
            Node::CodeSpan => Frame::CodeSpan {
                text: String::new(),
            },
            Node::Emphasis => Frame::Emphasis {
                text: String::new(),
            },
            Node::Anchor => Frame::Anchor {
                text: String::new(),
                url: String::new(),
            },
            Node::Title => Frame::Title {
                text: String::new(),
            },
            Node::Destination => Frame::Destination {
                text: String::new(),
            },
            Node::Image => Frame::Image {
                alt: String::new(),
                src: String::new(),
            },
            Node::Images => Frame::Images { images: Vec::new() },
            Node::Code => Frame::Code {
                lang: String::new(),
                code: String::new(),
            },
            Node::Modifier => Frame::Modifier {
                text: String::new(),
            },
            Node::Embed => Frame::Embed { values: Vec::new() },
            Node::ThematicBreak => Frame::ThematicBreak,
            Node::Highlight => Frame::Highlight {
                title: None,
                icon: None,
                paragraphs: Vec::new(),
            },
            Node::Icon => Frame::Icon {
                text: String::new(),
            },
            Node::Collapsible => Frame::Collapsible {
                title: String::new(),
                body: Vec::new(),
            },
            Node::ListItem => Frame::ListItem {
                text: Vec::new(),
                nested_list: None,
            },
            Node::Metadata => Frame::Metadata {
                text: String::new(),
            },
            Node::Heading | Node::UnorderedList | Node::OrderedList => {
                unreachable!("use dedicated push logic for {node:?}")
            }
        }
    }
}

fn count_list_depth(stack: &[Frame]) -> usize {
    stack
        .iter()
        .filter(|f| matches!(f, Frame::UnorderedList { .. } | Frame::OrderedList { .. }))
        .count()
}

/// Converts an operation stream into the final [`Yamd`] AST.
///
/// Takes the `ops` produced by [`parse`](crate::parse) and the original `source` text.
/// Walks the operation stream using a stack of frames — each [`OpKind::Start`] pushes a frame,
/// each [`OpKind::End`] pops it and folds the result into the parent.
///
/// ```
/// let source = "# hello\n\nworld";
/// let ops = yamd::parse(source);
/// let yamd = yamd::to_yamd(&ops, source);
/// assert_eq!(yamd.body.len(), 2);
/// ```
pub fn to_yamd(ops: &[Op], source: &str) -> Yamd {
    let mut stack: Vec<Frame> = vec![Frame::Yamd {
        metadata: None,
        body: Vec::new(),
    }];

    for op in ops {
        match &op.kind {
            OpKind::Start(node) => match node {
                Node::Heading => {
                    let level = extract_heading_level(&op.content, source);
                    stack.push(Frame::Heading {
                        level,
                        body: Vec::new(),
                    });
                }
                Node::UnorderedList => {
                    let level = count_list_depth(&stack);
                    stack.push(Frame::UnorderedList {
                        level,
                        items: Vec::new(),
                    });
                }
                Node::OrderedList => {
                    let level = count_list_depth(&stack);
                    stack.push(Frame::OrderedList {
                        level,
                        items: Vec::new(),
                    });
                }
                _ => stack.push(Frame::from_node(node)),
            },
            OpKind::Value => {
                let text = op.content.to_string(source);
                let top = stack
                    .last_mut()
                    .expect("stack should not be empty on Value");
                match top {
                    Frame::Heading { body, .. } => {
                        body.push(HeadingNodes::Text(text));
                    }
                    Frame::Paragraph { body } => {
                        body.push(ParagraphNodes::Text(text));
                    }
                    Frame::Bold { body } => {
                        body.push(BoldNodes::Text(text));
                    }
                    Frame::Italic { text: t } => t.push_str(&text),
                    Frame::Strikethrough { text: t } => t.push_str(&text),
                    Frame::CodeSpan { text: t } => t.push_str(&text),
                    Frame::Emphasis { text: t } => t.push_str(&text),
                    Frame::Title { text: t } => t.push_str(&text),
                    Frame::Destination { text: t } => t.push_str(&text),
                    Frame::Modifier { text: t } => t.push_str(&text),
                    Frame::Icon { text: t } => t.push_str(&text),
                    Frame::Code { code, .. } => code.push_str(&text),
                    Frame::Embed { values } => values.push(text),
                    Frame::Metadata { text: t } => t.push_str(&text),
                    Frame::ThematicBreak => {}
                    Frame::Highlight { .. } | Frame::Document { .. } => {}
                    _ => {}
                }
            }
            OpKind::End(node) => {
                let frame = stack.pop().expect("stack should not be empty on End");
                match (node, frame) {
                    (Node::Document, Frame::Document { children }) => {
                        let top = stack.last_mut().expect("stack underflow");
                        match top {
                            Frame::Collapsible { body, .. } => {
                                *body = children;
                            }
                            Frame::Yamd { body, .. } => {
                                body.extend(children);
                            }
                            _ => {}
                        }
                    }
                    (Node::Metadata, Frame::Metadata { text }) => {
                        let metadata = text.trim_matches('\n').to_owned();
                        if let Some(Frame::Yamd { metadata: m, .. }) = stack.last_mut() {
                            *m = Some(metadata);
                        }
                    }
                    (Node::Heading, Frame::Heading { level, body }) => {
                        push_yamd_node(&mut stack, Heading::new(level, body).into());
                    }
                    (Node::Paragraph, Frame::Paragraph { mut body }) => {
                        let top = stack.last_mut().expect("stack underflow");
                        match top {
                            Frame::Highlight { paragraphs, .. } => {
                                trim_trailing_newline_from_text(&mut body);
                                paragraphs.push(Paragraph::new(body));
                            }
                            Frame::ListItem { text, .. } => {
                                trim_trailing_newline_from_text(&mut body);
                                *text = body;
                            }
                            Frame::Yamd {
                                body: yamd_body, ..
                            } => {
                                yamd_body.push(Paragraph::new(body).into());
                            }
                            Frame::Document { children } => {
                                children.push(Paragraph::new(body).into());
                            }
                            Frame::Collapsible {
                                body: coll_body, ..
                            } => {
                                coll_body.push(Paragraph::new(body).into());
                            }
                            _ => {}
                        }
                    }
                    (Node::Bold, Frame::Bold { body }) => {
                        push_into_paragraph(&mut stack, Bold::new(body).into());
                    }
                    (Node::Italic, Frame::Italic { text }) => {
                        let italic = Italic::new(text);
                        match stack.last_mut().expect("stack underflow") {
                            Frame::Paragraph { body } => body.push(italic.into()),
                            Frame::Bold { body } => body.push(italic.into()),
                            _ => {}
                        }
                    }
                    (Node::Strikethrough, Frame::Strikethrough { text }) => {
                        let st = Strikethrough::new(text);
                        match stack.last_mut().expect("stack underflow") {
                            Frame::Paragraph { body } => body.push(st.into()),
                            Frame::Bold { body } => body.push(st.into()),
                            _ => {}
                        }
                    }
                    (Node::CodeSpan, Frame::CodeSpan { text }) => {
                        push_into_paragraph(&mut stack, CodeSpan::new(text).into());
                    }
                    (Node::Emphasis, Frame::Emphasis { text }) => {
                        push_into_paragraph(&mut stack, Emphasis::new(text).into());
                    }
                    (Node::Title, Frame::Title { text }) => {
                        let top = stack.last_mut().expect("stack underflow");
                        match top {
                            Frame::Anchor { text: t, .. } => *t = text,
                            Frame::Image { alt, .. } => *alt = text,
                            _ => {}
                        }
                    }
                    (Node::Destination, Frame::Destination { text }) => {
                        let top = stack.last_mut().expect("stack underflow");
                        match top {
                            Frame::Anchor { url, .. } => *url = text,
                            Frame::Image { src, .. } => *src = text,
                            _ => {}
                        }
                    }
                    (Node::Anchor, Frame::Anchor { text, url }) => {
                        let anchor = Anchor::new(text, url);
                        let top = stack.last_mut().expect("stack underflow");
                        match top {
                            Frame::Paragraph { body } => body.push(anchor.into()),
                            Frame::Heading { body, .. } => body.push(anchor.into()),
                            _ => {}
                        }
                    }
                    (Node::Image, Frame::Image { alt, src }) => {
                        let image = Image::new(alt, src);
                        let top = stack.last_mut().expect("stack underflow");
                        match top {
                            Frame::Images { images } => images.push(image),
                            _ => push_yamd_node(&mut stack, image.into()),
                        }
                    }
                    (Node::Images, Frame::Images { images }) => {
                        if images.len() == 1 {
                            push_yamd_node(&mut stack, images.into_iter().next().unwrap().into());
                        } else {
                            push_yamd_node(&mut stack, Images::new(images).into());
                        }
                    }
                    (Node::Code, Frame::Code { lang, code }) => {
                        let code = code.trim_end_matches('\n').to_owned();
                        push_yamd_node(&mut stack, Code::new(lang, code).into());
                    }
                    (Node::Modifier, Frame::Modifier { text }) => {
                        let top = stack.last_mut().expect("stack underflow");
                        match top {
                            Frame::Code { lang, .. } => *lang = text,
                            Frame::Highlight { title, .. } => *title = Some(text),
                            Frame::Collapsible { title: t, .. } => *t = text,
                            _ => {}
                        }
                    }
                    (Node::Icon, Frame::Icon { text }) => {
                        if let Some(Frame::Highlight { icon, .. }) = stack.last_mut() {
                            *icon = Some(text);
                        }
                    }
                    (Node::Embed, Frame::Embed { values }) => {
                        let kind = values.first().cloned().unwrap_or_default();
                        let args = values.get(2).cloned().unwrap_or_default();
                        push_yamd_node(&mut stack, Embed::new(kind, args).into());
                    }
                    (Node::ThematicBreak, Frame::ThematicBreak) => {
                        push_yamd_node(&mut stack, ThematicBreak::new().into());
                    }
                    (
                        Node::Highlight,
                        Frame::Highlight {
                            title,
                            icon,
                            paragraphs,
                        },
                    ) => {
                        push_yamd_node(&mut stack, Highlight::new(title, icon, paragraphs).into());
                    }
                    (Node::Collapsible, Frame::Collapsible { title, body }) => {
                        push_yamd_node(&mut stack, Collapsible::new(title, body).into());
                    }
                    (Node::UnorderedList, Frame::UnorderedList { level, items }) => {
                        finish_list(&mut stack, List::new(ListTypes::Unordered, level, items));
                    }
                    (Node::OrderedList, Frame::OrderedList { level, items }) => {
                        finish_list(&mut stack, List::new(ListTypes::Ordered, level, items));
                    }
                    (Node::ListItem, Frame::ListItem { text, nested_list }) => {
                        let item = ListItem::new(text, nested_list);
                        if let Some(
                            Frame::UnorderedList { items, .. } | Frame::OrderedList { items, .. },
                        ) = stack.last_mut()
                        {
                            items.push(item);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    match stack.pop().expect("stack should have Yamd root") {
        Frame::Yamd { metadata, body } => Yamd::new(metadata, body),
        _ => panic!("expected Yamd frame at bottom of stack"),
    }
}

fn extract_heading_level(content: &Content, source: &str) -> u8 {
    let s = content.as_str(source);
    s.chars().take_while(|&c| c == '#').count() as u8
}

fn trim_trailing_newline_from_text(body: &mut Vec<ParagraphNodes>) {
    if let Some(ParagraphNodes::Text(t)) = body.last_mut() {
        let trimmed = t.trim_end_matches('\n');
        if trimmed.is_empty() {
            body.pop();
        } else {
            *t = trimmed.to_owned();
        }
    }
}

fn push_yamd_node(stack: &mut [Frame], node: YamdNodes) {
    let top = stack.last_mut().expect("stack underflow");
    match top {
        Frame::Yamd { body, .. } => body.push(node),
        Frame::Document { children } => children.push(node),
        Frame::Collapsible { body, .. } => body.push(node),
        _ => {}
    }
}

fn push_into_paragraph(stack: &mut [Frame], node: ParagraphNodes) {
    if let Some(Frame::Paragraph { body }) = stack.last_mut() {
        body.push(node);
    }
}

fn finish_list(stack: &mut [Frame], list: List) {
    match stack.last_mut().expect("stack underflow") {
        Frame::ListItem { nested_list, .. } => *nested_list = Some(list),
        _ => push_yamd_node(stack, list.into()),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::nodes::*;
    use crate::op::{parse, to_yamd};

    #[test]
    fn single_paragraph() {
        let input = "hello world";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![Paragraph::new(vec![String::from("hello world").into()]).into()]
            )
        );
    }

    #[test]
    fn heading_with_anchor() {
        let input = "## heading [a](u) text";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Heading::new(
                        2,
                        vec![
                            String::from("heading ").into(),
                            HeadingNodes::Anchor(Anchor::new("a", "u")),
                            String::from(" text").into(),
                        ]
                    )
                    .into()
                ]
            )
        );
    }

    #[test]
    fn bold_with_nested() {
        let input = "**~~happy~~ _path_**";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Paragraph::new(vec![ParagraphNodes::Bold(Bold::new(vec![
                        BoldNodes::Strikethrough(Strikethrough::new("happy")),
                        BoldNodes::Text(String::from(" ")),
                        BoldNodes::Italic(Italic::new("path")),
                    ]))])
                    .into()
                ]
            )
        );
    }

    #[test]
    fn code_block() {
        let input = "```rust\nlet a=1;\n```";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(None, vec![Code::new("rust", "let a=1;").into()])
        );
    }

    #[test]
    fn single_image() {
        let input = "![alt](src)";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(None, vec![Image::new("alt", "src").into()])
        );
    }

    #[test]
    fn multiple_images() {
        let input = "![a](u)\n![a2](u2)";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![Images::new(vec![Image::new("a", "u"), Image::new("a2", "u2")]).into()]
            )
        );
    }

    #[test]
    fn highlight_with_icon() {
        let input = "!! Title\n! Icon\ntext\n!!";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Highlight::new(
                        Some("Title"),
                        Some("Icon"),
                        vec![Paragraph::new(vec![String::from("text").into()])]
                    )
                    .into()
                ]
            )
        );
    }

    #[test]
    fn unordered_list_nested() {
        let input = "- one\n - two";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    List::new(
                        ListTypes::Unordered,
                        0,
                        vec![ListItem::new(
                            vec![String::from("one").into()],
                            Some(List::new(
                                ListTypes::Unordered,
                                1,
                                vec![ListItem::new(vec![String::from("two").into()], None)]
                            ))
                        )]
                    )
                    .into()
                ]
            )
        );
    }

    #[test]
    fn three_level_nested_list() {
        let input = "- L0\n - L1\n  - L2";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    List::new(
                        ListTypes::Unordered,
                        0,
                        vec![ListItem::new(
                            vec![String::from("L0").into()],
                            Some(List::new(
                                ListTypes::Unordered,
                                1,
                                vec![ListItem::new(
                                    vec![String::from("L1").into()],
                                    Some(List::new(
                                        ListTypes::Unordered,
                                        2,
                                        vec![ListItem::new(vec![String::from("L2").into()], None,)]
                                    ))
                                )]
                            ))
                        )]
                    )
                    .into()
                ]
            )
        );
    }

    #[test]
    fn embed_node() {
        let input = "{{youtube|123}}";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(None, vec![Embed::new("youtube", "123").into()])
        );
    }

    #[test]
    fn collapsible_empty() {
        let input = "{% collapsible\n\n%}";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(None, vec![Collapsible::new("collapsible", vec![]).into()])
        );
    }

    #[test]
    fn thematic_break() {
        let input = "-----";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(result, Yamd::new(None, vec![ThematicBreak::new().into()]));
    }

    #[test]
    fn metadata_test() {
        let input = "---\ntitle: test\n---\n\nhello";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                Some(String::from("title: test")),
                vec![Paragraph::new(vec![String::from("hello").into()]).into()]
            )
        );
    }

    #[test]
    fn code_span_and_emphasis() {
        let input = "`code` *em*";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Paragraph::new(vec![
                        ParagraphNodes::CodeSpan(CodeSpan::new("code")),
                        ParagraphNodes::Text(String::from(" ")),
                        ParagraphNodes::Emphasis(Emphasis::new("em")),
                    ])
                    .into()
                ]
            )
        );
    }

    #[test]
    fn full_document() {
        let input = r#"---
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
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                Some(String::from(
                    "title: test\ndate: 2022-01-01T00:00:00+02:00\nimage: image\npreview: preview\ntags:\n- tag1\n- tag2"
                )),
                vec![
                    Heading::new(1, vec![String::from("hello").into()]).into(),
                    Code::new("rust", "let a=1;").into(),
                    Paragraph::new(vec![
                        String::from("t").into(),
                        ParagraphNodes::Bold(Bold::new(vec![String::from("b").into()]))
                    ])
                    .into(),
                    Image::new('a', 'u').into(),
                    Images::new(vec![Image::new("a", "u"), Image::new("a2", "u2")]).into(),
                    Highlight::new(
                        Some("H"),
                        Some("I"),
                        vec![
                            Paragraph::new(vec![ParagraphNodes::Strikethrough(
                                Strikethrough::new("s")
                            )]),
                            Paragraph::new(vec![ParagraphNodes::Italic(Italic::new("I"))])
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
                                vec![ListItem::new(vec![String::from("two").into()], None)]
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
                                vec![ListItem::new(vec![String::from("second").into()], None)]
                            ))
                        )]
                    )
                    .into(),
                    Embed::new("youtube", "123").into(),
                    Embed::new("cloudinary_gallery", "cloud_name&tag").into(),
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
    fn multiple_fallbacks_in_a_row() {
        let input = "1\n\n2\n\n3";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Paragraph::new(vec![String::from("1").into()]).into(),
                    Paragraph::new(vec![String::from("2").into()]).into(),
                    Paragraph::new(vec![String::from("3").into()]).into(),
                ]
            )
        );
    }

    #[test]
    fn multiple_fallbacks_before_non_fallback() {
        let input = "1\n\n2\n\n3\n\n# header";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Paragraph::new(vec![String::from("1").into()]).into(),
                    Paragraph::new(vec![String::from("2").into()]).into(),
                    Paragraph::new(vec![String::from("3").into()]).into(),
                    Heading::new(1, vec![String::from("header").into()]).into(),
                ]
            )
        );
    }

    #[test]
    fn node_should_start_from_delimiter() {
        let input = "text - text";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![Paragraph::new(vec![String::from("text - text").into()]).into()]
            )
        );
    }

    #[test]
    fn collapsible_with_body() {
        let input = "{% title\n\nparagraph text\n\n%}";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Collapsible::new(
                        "title",
                        vec![Paragraph::new(vec![String::from("paragraph text").into()]).into()]
                    )
                    .into()
                ]
            )
        );
    }

    #[test]
    fn ordered_list_nested() {
        let input = "+ one\n + two";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    List::new(
                        ListTypes::Ordered,
                        0,
                        vec![ListItem::new(
                            vec![String::from("one").into()],
                            Some(List::new(
                                ListTypes::Ordered,
                                1,
                                vec![ListItem::new(vec![String::from("two").into()], None)]
                            ))
                        )]
                    )
                    .into()
                ]
            )
        );
    }

    #[test]
    fn strikethrough_in_bold() {
        let input = "**~~text~~**";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Paragraph::new(vec![ParagraphNodes::Bold(Bold::new(vec![
                        BoldNodes::Strikethrough(Strikethrough::new("text"))
                    ]))])
                    .into()
                ]
            )
        );
    }

    #[test]
    fn italic_in_bold() {
        let input = "**_text_**";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Paragraph::new(vec![ParagraphNodes::Bold(Bold::new(vec![
                        BoldNodes::Italic(Italic::new("text"))
                    ]))])
                    .into()
                ]
            )
        );
    }

    #[test]
    fn highlight_without_icon() {
        let input = "!! Title\ntext\n!!";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Highlight::new(
                        Some("Title"),
                        None::<&str>,
                        vec![Paragraph::new(vec![String::from("text").into()])]
                    )
                    .into()
                ]
            )
        );
    }

    #[test]
    fn empty_input() {
        let input = "";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(result, Yamd::new(None, vec![]));
    }

    #[test]
    fn utf8_content() {
        let input = "## 🤔\n\n[link 😉](url)";
        let ops = parse(input);
        let result = to_yamd(&ops, input);
        assert_eq!(
            result,
            Yamd::new(
                None,
                vec![
                    Heading::new(2, vec![String::from("🤔").into()]).into(),
                    Paragraph::new(vec![ParagraphNodes::Anchor(Anchor::new("link 😉", "url"))])
                        .into()
                ]
            )
        );
    }
}
