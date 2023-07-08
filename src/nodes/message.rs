use crate::toolkit::node::Node;

use super::paragraph::Paragraph;

pub struct Message {
    header: Option<String>,
    icon: Option<String>,
    nodes: Vec<Paragraph>,
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
        nodes: Vec<Paragraph>,
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

#[cfg(test)]
mod test {
    use super::Message;
    use crate::{
        nodes::{paragraph::Paragraph, text::Text},
        toolkit::node::Node,
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
                vec![Paragraph::new_with_nodes(
                    true,
                    vec![Text::new("simple text").into()]
                )],
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
}
