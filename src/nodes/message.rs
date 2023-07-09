use crate::toolkit::{
    context::Context,
    deserializer::{Branch, DefinitelyNode, Deserializer, FallbackNode, MaybeNode},
    matcher::Matcher,
    node::Node,
    pattern::Quantifiers::*,
};

use super::paragraph::Paragraph;

#[derive(Debug, PartialEq)]
pub enum MessageNodes {
    Paragraph(Paragraph),
}

impl Node for MessageNodes {
    fn serialize(&self) -> String {
        match self {
            Self::Paragraph(p) => p.serialize(),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Paragraph(p) => p.len(),
        }
    }
}

impl From<Paragraph> for MessageNodes {
    fn from(p: Paragraph) -> Self {
        Self::Paragraph(p)
    }
}

#[derive(Debug, PartialEq)]
pub struct Message {
    header: Option<String>,
    icon: Option<String>,
    nodes: Vec<MessageNodes>,
    warning: bool,
    consumed_all_input: bool,
}

impl Message {
    pub fn new<S: Into<String>>(
        header: Option<S>,
        icon: Option<S>,
        warning: bool,
        consumed_all_input: bool,
    ) -> Self {
        Self::new_with_nodes(header, icon, Vec::new(), warning, consumed_all_input)
    }

    pub fn new_with_nodes<S: Into<String>>(
        header: Option<S>,
        icon: Option<S>,
        nodes: Vec<MessageNodes>,
        warning: bool,
        consumed_all_input: bool,
    ) -> Self {
        Self {
            header: header.map(|s| s.into()),
            icon: icon.map(|s| s.into()),
            nodes,
            warning,
            consumed_all_input,
        }
    }
}

impl Node for Message {
    fn serialize(&self) -> String {
        format!(
            "%%%%\n{header}{icon}{warning}{nodes}\n%%%%{end}",
            header = self
                .header
                .as_ref()
                .map_or(String::new(), |s| format!("%%% {}\n", s)),
            icon = self
                .icon
                .as_ref()
                .map_or(String::new(), |s| format!("%% {}\n", s)),
            nodes = self.nodes.iter().map(|n| n.serialize()).collect::<String>(),
            warning = if self.warning { "% \n" } else { "" },
            end = if self.consumed_all_input { "" } else { "\n\n" }
        )
    }

    fn len(&self) -> usize {
        let mut len = 10;
        if let Some(header) = &self.header {
            len += header.len() + 5;
        }
        if let Some(icon) = &self.icon {
            len += icon.len() + 4;
        }
        len += self.nodes.iter().map(|n| n.len()).sum::<usize>();
        if self.warning {
            len += 3;
        }
        if !self.consumed_all_input {
            len += 2;
        }
        len
    }
}

impl Branch<MessageNodes> for Message {
    fn push<CanBeNode: Into<MessageNodes>>(&mut self, node: CanBeNode) {
        self.nodes.push(node.into());
    }

    fn get_maybe_nodes() -> Vec<MaybeNode<MessageNodes>> {
        vec![]
    }

    fn get_fallback_node() -> Option<DefinitelyNode<MessageNodes>> {
        Some(Paragraph::fallback_node())
    }

    fn get_outer_token_length(&self) -> usize {
        let mut len = 10;
        if let Some(header) = &self.header {
            len += header.len() + 5;
        }
        if let Some(icon) = &self.icon {
            len += icon.len() + 4;
        }
        if self.warning {
            len += 3;
        }
        if !self.consumed_all_input {
            len += 2;
        }
        len
    }
}

impl Deserializer for Message {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut matcher = Matcher::new(input);
        if let Some(message) = matcher.get_match(
            &[RepeatTimes(4, '%'), Once('\n')],
            &[Once('\n'), RepeatTimes(4, '%')],
            false,
        ) {
            let mut inner_matcher = Matcher::new(message.body);
            let header = inner_matcher
                .get_match(&[RepeatTimes(3, '%'), Once(' ')], &[Once('\n')], false)
                .map(|s| s.body.to_string());
            let icon = inner_matcher
                .get_match(&[RepeatTimes(2, '%'), Once(' ')], &[Once('\n')], false)
                .map(|s| s.body.to_string());
            let warning = inner_matcher
                .get_match(&[Once('%'), Once(' ')], &[Once('\n')], false)
                .is_some();

            let consumed_all_input = matcher
                .get_match(&[RepeatTimes(2, '\n')], &[], false)
                .is_none();
            let rest = inner_matcher.get_rest();

            let container = Self::new(header, icon, warning, consumed_all_input);

            if rest.is_empty() {
                return Some(container);
            } else {
                return Self::parse_branch(rest, container);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::Message;
    use crate::{
        nodes::{bold::Bold, paragraph::Paragraph, strikethrough::Strikethrough, text::Text},
        toolkit::{deserializer::Deserializer, node::Node},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn len() {
        assert_eq!(Message::new::<&str>(None, None, false, false).len(), 12);
        assert_eq!(Message::new(Some("header"), None, false, false).len(), 23);
        assert_eq!(Message::new(None, Some("icon"), false, false).len(), 20);
        assert_eq!(
            Message::new(Some("header"), Some("icon"), false, false).len(),
            31
        );
        assert_eq!(Message::new::<&str>(None, None, true, false).len(), 15);
        assert_eq!(Message::new(Some("header"), None, true, false).len(), 26);
        assert_eq!(Message::new(None, Some("icon"), true, false).len(), 23);
        assert_eq!(
            Message::new(Some("header"), Some("icon"), true, false).len(),
            34
        );
        assert_eq!(Message::new::<&str>(None, None, false, true).len(), 10);
        assert_eq!(Message::new(Some("header"), None, false, true).len(), 21);
        assert_eq!(Message::new(None, Some("icon"), false, true).len(), 18);
        assert_eq!(
            Message::new(Some("header"), Some("icon"), false, true).len(),
            29
        );
        assert_eq!(Message::new::<&str>(None, None, true, true).len(), 13);
        assert_eq!(Message::new(Some("header"), None, true, true).len(), 24);
        assert_eq!(Message::new(None, Some("icon"), true, true).len(), 21);
        assert_eq!(
            Message::new(Some("header"), Some("icon"), true, true).len(),
            32
        );
        assert_eq!(
            Message::new_with_nodes(
                Some("header"),
                Some("icon"),
                vec![Paragraph::new_with_nodes(true, vec![Text::new("simple text").into()]).into()],
                true,
                true
            )
            .len(),
            43
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message::new::<&str>(None, None, false, false).serialize(),
            "%%%%\n\n%%%%\n\n"
        );
        assert_eq!(
            Message::new(Some("header"), None, false, false).serialize(),
            "%%%%\n%%% header\n\n%%%%\n\n"
        );
        assert_eq!(
            Message::new(None, Some("icon"), false, false).serialize(),
            "%%%%\n%% icon\n\n%%%%\n\n"
        );
        assert_eq!(
            Message::new(Some("header"), Some("icon"), false, false).serialize(),
            "%%%%\n%%% header\n%% icon\n\n%%%%\n\n"
        );
        assert_eq!(
            Message::new::<&str>(None, None, true, false).serialize(),
            "%%%%\n% \n\n%%%%\n\n"
        );
        assert_eq!(
            Message::new(Some("header"), None, true, false).serialize(),
            "%%%%\n%%% header\n% \n\n%%%%\n\n"
        );
        assert_eq!(
            Message::new(None, Some("icon"), true, false).serialize(),
            "%%%%\n%% icon\n% \n\n%%%%\n\n"
        );
        assert_eq!(
            Message::new(Some("header"), Some("icon"), true, false).serialize(),
            "%%%%\n%%% header\n%% icon\n% \n\n%%%%\n\n"
        );
        assert_eq!(
            Message::new::<&str>(None, None, false, true).serialize(),
            "%%%%\n\n%%%%"
        );
        assert_eq!(
            Message::new(Some("header"), None, false, true).serialize(),
            "%%%%\n%%% header\n\n%%%%"
        );
        assert_eq!(
            Message::new(None, Some("icon"), false, true).serialize(),
            "%%%%\n%% icon\n\n%%%%"
        );
        assert_eq!(
            Message::new(Some("header"), Some("icon"), false, true).serialize(),
            "%%%%\n%%% header\n%% icon\n\n%%%%"
        );
        assert_eq!(
            Message::new::<&str>(None, None, true, true).serialize(),
            "%%%%\n% \n\n%%%%"
        );
        assert_eq!(
            Message::new(Some("header"), None, true, true).serialize(),
            "%%%%\n%%% header\n% \n\n%%%%"
        );
    }

    #[test]
    fn deserialize_empty() {
        assert_eq!(
            Message::deserialize("%%%%\n\n%%%%\n\n"),
            Some(Message::new::<&str>(None, None, false, false))
        );
        assert_eq!(
            Message::deserialize("%%%%\n\n%%%%"),
            Some(Message::new::<&str>(None, None, false, true))
        );
    }

    #[test]
    fn deserialize_with_header() {
        assert_eq!(
            Message::deserialize("%%%%\n%%% header\n\n%%%%\n\n"),
            Some(Message::new(Some("header"), None, false, false)),
        );
        assert_eq!(
            Message::deserialize("%%%%\n%%% header\n\n%%%%"),
            Some(Message::new(Some("header"), None, false, true)),
        );
    }

    #[test]
    fn deserialize_with_icon() {
        assert_eq!(
            Message::deserialize("%%%%\n%% icon\n\n%%%%\n\n"),
            Some(Message::new(None, Some("icon"), false, false)),
        );
        assert_eq!(
            Message::deserialize("%%%%\n%% icon\n\n%%%%"),
            Some(Message::new(None, Some("icon"), false, true)),
        );
    }

    #[test]
    fn deserialize_with_header_and_icon() {
        assert_eq!(
            Message::deserialize("%%%%\n%%% header\n%% icon\n\n%%%%\n\n"),
            Some(Message::new(Some("header"), Some("icon"), false, false)),
        );
        assert_eq!(
            Message::deserialize("%%%%\n%%% header\n%% icon\n\n%%%%"),
            Some(Message::new(Some("header"), Some("icon"), false, true)),
        );
    }

    #[test]
    fn deserialize_with_header_and_icon_and_warning() {
        assert_eq!(
            Message::deserialize("%%%%\n%%% header\n%% icon\n% \n\n%%%%\n\n"),
            Some(Message::new::<&str>(
                Some("header"),
                Some("icon"),
                true,
                false
            )),
        );
        assert_eq!(
            Message::deserialize("%%%%\n%%% header\n%% icon\n% \n\n%%%%"),
            Some(Message::new::<&str>(
                Some("header"),
                Some("icon"),
                true,
                true
            )),
        );
    }

    #[test]
    fn deserialize_with_header_and_warning() {
        assert_eq!(
            Message::deserialize("%%%%\n%%% header\n% \n\n%%%%\n\n"),
            Some(Message::new::<&str>(Some("header"), None, true, false)),
        );
        assert_eq!(
            Message::deserialize("%%%%\n%%% header\n% \n\n%%%%"),
            Some(Message::new::<&str>(Some("header"), None, true, true)),
        );
    }

    #[test]
    fn deserialize_with_icon_and_warning() {
        assert_eq!(
            Message::deserialize("%%%%\n%% icon\n% \n\n%%%%\n\n"),
            Some(Message::new::<&str>(None, Some("icon"), true, false)),
        );
        assert_eq!(
            Message::deserialize("%%%%\n%% icon\n% \n\n%%%%"),
            Some(Message::new::<&str>(None, Some("icon"), true, true)),
        );
    }

    #[test]
    fn deserialize_with_warning() {
        assert_eq!(
            Message::deserialize("%%%%\n% \n\n%%%%\n\n"),
            Some(Message::new::<&str>(None, None, true, false)),
        );
        assert_eq!(
            Message::deserialize("%%%%\n% \n\n%%%%"),
            Some(Message::new::<&str>(None, None, true, true)),
        );
    }

    #[test]
    fn deserialize_with_header_and_icon_and_content() {
        assert_eq!(
            Message::deserialize("%%%%\n%%% header\n%% icon\nthis is some **content**\n\nand this is next ~~line~~\n%%%%\n\n"),
            Some(Message::new_with_nodes(
                Some("header"),
                Some("icon"),
                vec![
                    Paragraph::new_with_nodes(false, vec![
                    Text::new("this is some ".to_string()).into(),
                    Bold::new_with_nodes(vec![Text::new("content").into()]).into(),
                ]).into(),
                    Paragraph::new_with_nodes(true, vec![
                    Text::new("and this is next ").into(),
                    Strikethrough::new("line").into(),
                ]).into(),
                ],
                false,
                false
            )),
        );
    }
}
