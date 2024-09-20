use std::fmt::Display;

use serde::Serialize;

use super::YamdNodes;

/// # Collapsible
///
/// Starts with [CollapsibleStart](type@crate::lexer::TokenKind::CollapsibleStart).
///
/// [Title](Collapsible::title) every token except
/// [Terminator](type@crate::lexer::TokenKind::Terminator) between
/// [space](type@crate::lexer::TokenKind::Space) and [EOL](type@crate::lexer::TokenKind::Eol)
///
/// [Body](Collapsible::body) Every token until [CollapsibleEnd](type@crate::lexer::TokenKind::CollapsibleEnd),
/// nested collapsible are supported.
///
/// Example:
///
/// ```text
/// {% collapsible
/// ![alt](src)
/// %}
/// ```
///
/// HTML equivalent:
///
/// ```html
/// <div class="collapsible">
///     <input type="checkbox" id="{{ node.title }}" />
///     <label for="{{ node.title }}">Or collapsible</label>
///     <div class="body">
///         <img alt="alt" src="src" />
///     </div>
/// </div>
/// ```
#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
pub struct Collapsible {
    pub title: String,
    pub body: Vec<YamdNodes>,
}

impl Collapsible {
    pub fn new<S: Into<String>>(title: S, body: Vec<YamdNodes>) -> Self {
        Self {
            body,
            title: title.into(),
        }
    }
}

impl Display for Collapsible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{% {title}\n{nodes}\n%}}",
            title = self.title,
            nodes = self
                .body
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}
