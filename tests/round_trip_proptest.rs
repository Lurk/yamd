use proptest::prelude::*;
use yamd::deserialize;
use yamd::nodes::{
    Anchor, Bold, BoldNodes, Code, CodeSpan, Collapsible, Embed, Emphasis, Heading, HeadingNodes,
    Highlight, Image, Images, Italic, List, ListItem, ListTypes, Paragraph, ParagraphNodes,
    Strikethrough, ThematicBreak, Yamd, YamdNodes,
};

fn arb_text() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-zA-Z0-9 ]{1,20}",
        prop::collection::vec(
            prop_oneof![
                Just("*".to_string()),
                Just("**".to_string()),
                Just("_".to_string()),
                Just("~~".to_string()),
                Just("`".to_string()),
                Just("```".to_string()),
                Just("#".to_string()),
                Just("!!".to_string()),
                Just("[".to_string()),
                Just("]".to_string()),
                Just("(".to_string()),
                Just(")".to_string()),
                Just("\\".to_string()),
                Just("\n".to_string()),
                Just("|".to_string()),
                Just("}".to_string()),
                Just("{".to_string()),
                Just("%}".to_string()),
                Just("{%".to_string()),
                Just("\n- ".to_string()),
                Just("\n+ ".to_string()),
                Just("\n# ".to_string()),
                Just("\n```".to_string()),
                Just("\n!! ".to_string()),
                Just("\n---".to_string()),
                Just("\n{% ".to_string()),
                Just("\n%}".to_string()),
                Just("\n\n".to_string()),
                "[a-zA-Z0-9]{1,5}",
            ],
            1..=5
        )
        .prop_map(|v| v.join("")),
    ]
    .prop_filter("must not be empty", |s| !s.is_empty())
}

fn arb_inline_text() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-zA-Z0-9]{1,20}",
        prop::collection::vec(
            prop_oneof![
                Just("*".to_string()),
                Just("**".to_string()),
                Just("_".to_string()),
                Just("~~".to_string()),
                Just("`".to_string()),
                Just("```".to_string()),
                Just("#".to_string()),
                Just("!!".to_string()),
                Just("[".to_string()),
                Just("]".to_string()),
                Just("(".to_string()),
                Just(")".to_string()),
                Just("|".to_string()),
                Just("}".to_string()),
                Just("{".to_string()),
                Just("%}".to_string()),
                Just("{%".to_string()),
                Just("\\".to_string()),
                "[a-zA-Z0-9]{1,5}",
            ],
            1..=5
        )
        .prop_map(|v| v.join("")),
    ]
    .prop_filter(
        "must contain non-whitespace and not start with space",
        |s| !s.trim().is_empty() && !s.starts_with(' '),
    )
}

fn arb_block_text() -> impl Strategy<Value = String> {
    arb_text().prop_filter("safe for block content", |s| {
        !s.trim().is_empty() && !s.ends_with('\n') && !s.starts_with('\n')
    })
}

fn arb_list_item_text() -> impl Strategy<Value = String> {
    arb_block_text().prop_filter(
        "list items cannot start with whitespace (grammar requires exactly one space after marker)",
        |s| !s.starts_with(' '),
    )
}

fn arb_url() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-zA-Z0-9/:._-]{1,30}",
        prop::collection::vec(
            prop_oneof![
                Just("(".to_string()),
                Just(")".to_string()),
                Just("[".to_string()),
                Just("]".to_string()),
                "[a-zA-Z0-9/:._-]{1,10}",
            ],
            1..=5
        )
        .prop_map(|v| v.join("")),
    ]
    .prop_filter("must not be empty", |s| !s.is_empty())
}

proptest! {
    #[test]
    fn italic_round_trip(text in arb_inline_text()) {
        let yamd = Yamd::new(None, vec![
            Paragraph::new(vec![ParagraphNodes::from(Italic::new(&text))]).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }

    #[test]
    fn emphasis_round_trip(text in arb_inline_text()) {
        let yamd = Yamd::new(None, vec![
            Paragraph::new(vec![ParagraphNodes::from(Emphasis::new(&text))]).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }

    #[test]
    fn strikethrough_round_trip(text in arb_inline_text()) {
        let yamd = Yamd::new(None, vec![
            Paragraph::new(vec![ParagraphNodes::from(Strikethrough::new(&text))]).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }

    #[test]
    fn code_span_round_trip(text in arb_inline_text()) {
        let yamd = Yamd::new(None, vec![
            Paragraph::new(vec![ParagraphNodes::from(CodeSpan::new(&text))]).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }
}

proptest! {
    #[test]
    fn anchor_round_trip(
        text in arb_inline_text(),
        url in arb_url()
    ) {
        let yamd = Yamd::new(None, vec![
            Paragraph::new(vec![ParagraphNodes::from(Anchor::new(&text, &url))]).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }

    #[test]
    fn image_round_trip(
        alt in arb_inline_text(),
        src in arb_url()
    ) {
        let yamd = Yamd::new(None, vec![
            Image::new(&alt, &src).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }

    #[test]
    fn bold_round_trip(text in arb_inline_text()) {
        let yamd = Yamd::new(None, vec![
            Paragraph::new(vec![
                ParagraphNodes::from(Bold::new(vec![BoldNodes::from(text)])),
            ]).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }
}

proptest! {
    #[test]
    fn heading_round_trip(
        level in 1u8..=6u8,
        text in arb_inline_text()
    ) {
        let yamd = Yamd::new(None, vec![
            Heading::new(level, vec![HeadingNodes::from(text)]).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }

    #[test]
    fn code_round_trip(
        lang in arb_inline_text(),
        code in arb_block_text()
    ) {
        let yamd = Yamd::new(None, vec![
            Code::new(&lang, &code).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }

    #[test]
    fn embed_round_trip(
        kind in arb_inline_text(),
        args in arb_block_text()
    ) {
        let yamd = Yamd::new(None, vec![
            Embed::new(&kind, &args).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }
}

proptest! {
    #[test]
    fn highlight_round_trip(
        title in proptest::option::of(arb_inline_text()),
        icon in proptest::option::of(arb_inline_text()),
        body_text in arb_block_text()
    ) {
        let yamd = Yamd::new(None, vec![
            Highlight::new(
                title.as_deref(),
                icon.as_deref(),
                vec![Paragraph::new(vec![ParagraphNodes::from(body_text)])],
            ).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }

    #[test]
    fn list_round_trip(
        item1 in arb_list_item_text(),
        item2 in arb_list_item_text(),
        ordered in any::<bool>()
    ) {
        let list_type = if ordered { ListTypes::Ordered } else { ListTypes::Unordered };
        let yamd = Yamd::new(None, vec![
            List::new(
                list_type,
                0,
                vec![
                    ListItem::new(vec![ParagraphNodes::from(item1)], None),
                    ListItem::new(vec![ParagraphNodes::from(item2)], None),
                ],
            ).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }

    #[test]
    fn collapsible_round_trip(
        title in arb_inline_text(),
        body_text in arb_block_text()
    ) {
        let yamd = Yamd::new(None, vec![
            Collapsible::new(
                &title,
                vec![Paragraph::new(vec![ParagraphNodes::from(body_text)]).into()],
            ).into(),
        ]);
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }
}

fn arb_paragraph_node() -> impl Strategy<Value = ParagraphNodes> {
    prop_oneof![
        arb_inline_text().prop_map(ParagraphNodes::from),
        arb_inline_text().prop_map(|t| ParagraphNodes::from(Italic::new(t))),
        arb_inline_text().prop_map(|t| ParagraphNodes::from(Emphasis::new(t))),
        arb_inline_text().prop_map(|t| ParagraphNodes::from(Strikethrough::new(t))),
        arb_inline_text().prop_map(|t| ParagraphNodes::from(CodeSpan::new(t))),
        (arb_inline_text(), arb_url()).prop_map(|(t, u)| ParagraphNodes::from(Anchor::new(t, u))),
        "[a-zA-Z0-9]{1,20}".prop_map(|t| ParagraphNodes::from(Bold::new(vec![BoldNodes::from(t)]))),
    ]
}

fn arb_paragraph() -> impl Strategy<Value = Paragraph> {
    prop_oneof![
        arb_paragraph_node().prop_map(|n| Paragraph::new(vec![n])),
        arb_inline_text().prop_map(|t| Paragraph::new(vec![ParagraphNodes::from(t)])),
    ]
}

fn arb_yamd_node() -> impl Strategy<Value = YamdNodes> {
    prop_oneof![
        arb_paragraph().prop_map(YamdNodes::from),
        (1u8..=6u8, arb_inline_text()).prop_map(|(l, t)| Heading::new(
            l,
            vec![HeadingNodes::from(t)]
        )
        .into()),
        (arb_inline_text(), arb_url()).prop_map(|(a, s)| Image::new(a, s).into()),
        prop::collection::vec(
            (arb_inline_text(), arb_url()).prop_map(|(a, s)| Image::new(a, s)),
            2..=3
        )
        .prop_map(|v| Images::new(v).into()),
        (arb_inline_text(), arb_block_text()).prop_map(|(l, c)| Code::new(l, c).into()),
        Just(ThematicBreak::new().into()),
        (arb_inline_text(), arb_block_text()).prop_map(|(k, a)| Embed::new(k, a).into()),
    ]
}

fn arb_yamd() -> impl Strategy<Value = Yamd> {
    (
        proptest::option::of(
            "[a-zA-Z0-9: ]{1,30}".prop_filter("metadata must have non-whitespace", |s| {
                !s.trim().is_empty()
            }),
        ),
        prop::collection::vec(arb_yamd_node(), 1..=5),
    )
        .prop_map(|(metadata, body)| Yamd::new(metadata, body))
}

proptest! {
    #[test]
    fn yamd_document_round_trip(yamd in arb_yamd()) {
        let serialized = yamd.to_string();
        let deserialized = deserialize(&serialized);
        prop_assert_eq!(yamd, deserialized, "Serialized: {:?}", serialized);
    }
}
