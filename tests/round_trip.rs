use yamd::deserialize;
use yamd::nodes::{
    Bold, BoldNodes, Code, CodeSpan, Collapsible, Heading, HeadingNodes, Highlight, Italic, List,
    ListItem, ListTypes, Paragraph, ParagraphNodes, ThematicBreak, Yamd, YamdNodes,
};

fn round_trip(yamd: &Yamd) {
    let serialized = yamd.to_string();
    let deserialized = deserialize(&serialized);
    assert_eq!(
        *yamd, deserialized,
        "Round-trip failed.\nSerialized:\n{:?}\nPretty:\n{}",
        serialized, serialized
    );
}

fn round_trip_block(node: YamdNodes) {
    let yamd = Yamd::new(None, vec![node]);
    round_trip(&yamd);
}

fn round_trip_inline(node: ParagraphNodes) {
    round_trip_block(Paragraph::new(vec![node]).into());
}

#[test]
fn highlight_body_with_double_bang() {
    round_trip_block(
        Highlight::new(
            Some("title"),
            Some("icon"),
            vec![Paragraph::new(vec![ParagraphNodes::from(
                "body with !! inside".to_string(),
            )])],
        )
        .into(),
    );
}

#[test]
fn paragraph_text_with_terminator() {
    round_trip_inline(ParagraphNodes::from("text\n\nmore".to_string()));
}

#[test]
fn paragraph_text_with_backslash() {
    round_trip_inline(ParagraphNodes::from("text \\ more".to_string()));
}

#[test]
fn paragraph_text_with_double_backslash() {
    round_trip_inline(ParagraphNodes::from("text \\\\ more".to_string()));
}

#[test]
fn paragraph_text_with_special_chars() {
    round_trip_inline(ParagraphNodes::from(
        "**not bold** _not italic_".to_string(),
    ));
}

#[test]
fn list_item_text_with_dash_after_newline() {
    round_trip_block(
        List::new(
            ListTypes::Unordered,
            0,
            vec![ListItem::new(
                vec![ParagraphNodes::from("first\n- second".to_string())],
                None,
            )],
        )
        .into(),
    );
}

#[test]
fn list_item_text_with_plus_after_newline() {
    round_trip_block(
        List::new(
            ListTypes::Ordered,
            0,
            vec![ListItem::new(
                vec![ParagraphNodes::from("first\n+ second".to_string())],
                None,
            )],
        )
        .into(),
    );
}

#[test]
fn empty_string_italic() {
    round_trip_inline(Italic::new("").into());
}

#[test]
fn empty_string_bold() {
    let bold = Bold::new(vec![BoldNodes::from(String::new())]);
    assert_eq!(bold.to_string(), "");
}

#[test]
fn empty_string_code_span() {
    let code_span = CodeSpan::new("");
    assert_eq!(code_span.to_string(), "");
}

#[test]
fn unicode_emoji_in_paragraph() {
    round_trip_inline(ParagraphNodes::from("hello 🤔 world 🎉".to_string()));
}

#[test]
fn unicode_emoji_in_heading() {
    round_trip_block(Heading::new(1, vec![HeadingNodes::from("hello 🤔".to_string())]).into());
}

#[test]
fn backslash_at_end_of_text() {
    round_trip_inline(ParagraphNodes::from("text ending with \\".to_string()));
}

#[test]
fn consecutive_special_chars() {
    round_trip_inline(ParagraphNodes::from("***___~~~```###!!!".to_string()));
}

#[test]
fn metadata_round_trip() {
    let yamd = Yamd::new(
        Some("key: value\ntitle: test".to_string()),
        vec![Paragraph::new(vec![ParagraphNodes::from("body".to_string())]).into()],
    );
    round_trip(&yamd);
}

#[test]
fn metadata_with_triple_dash() {
    let yamd = Yamd::new(
        Some("key: ---value".to_string()),
        vec![Paragraph::new(vec![ParagraphNodes::from("body".to_string())]).into()],
    );
    round_trip(&yamd);
}

#[test]
fn full_document_round_trip() {
    let yamd = Yamd::new(
        Some("title: test".to_string()),
        vec![
            Heading::new(1, vec![HeadingNodes::from("Title".to_string())]).into(),
            Paragraph::new(vec![
                ParagraphNodes::from("Hello ".to_string()),
                ParagraphNodes::from(Bold::new(vec![BoldNodes::from("bold".to_string())])),
                ParagraphNodes::from(" and ".to_string()),
                ParagraphNodes::from(Italic::new("italic")),
            ])
            .into(),
            Code::new("rust", "fn main() {}").into(),
            ThematicBreak::new().into(),
            List::new(
                ListTypes::Unordered,
                0,
                vec![
                    ListItem::new(vec![ParagraphNodes::from("item 1".to_string())], None),
                    ListItem::new(vec![ParagraphNodes::from("item 2".to_string())], None),
                ],
            )
            .into(),
        ],
    );
    round_trip(&yamd);
}

#[test]
fn highlight_body_starts_with_list_marker() {
    round_trip_block(
        Highlight::new(
            Some("title"),
            Some("icon"),
            vec![Paragraph::new(vec![ParagraphNodes::from(
                "- not a list".to_string(),
            )])],
        )
        .into(),
    );
}

#[test]
fn highlight_body_starts_with_heading_marker() {
    round_trip_block(
        Highlight::new(
            Some("title"),
            Some("icon"),
            vec![Paragraph::new(vec![ParagraphNodes::from(
                "# not a heading".to_string(),
            )])],
        )
        .into(),
    );
}

#[test]
fn collapsible_body_starts_with_highlight_marker() {
    round_trip_block(
        Collapsible::new(
            "title",
            vec![
                Paragraph::new(vec![ParagraphNodes::from("!! not a highlight".to_string())]).into(),
            ],
        )
        .into(),
    );
}

#[test]
fn top_level_paragraph_starts_with_list_marker() {
    round_trip_block(Paragraph::new(vec![ParagraphNodes::from("- not a list".to_string())]).into());
}

#[test]
fn top_level_paragraph_starts_with_highlight_marker() {
    round_trip_block(
        Paragraph::new(vec![ParagraphNodes::from("!! not a highlight".to_string())]).into(),
    );
}

#[test]
fn collapsible_body_starts_with_list_marker() {
    round_trip_block(
        Collapsible::new(
            "title",
            vec![Paragraph::new(vec![ParagraphNodes::from("- not a list".to_string())]).into()],
        )
        .into(),
    );
}
