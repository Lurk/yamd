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
