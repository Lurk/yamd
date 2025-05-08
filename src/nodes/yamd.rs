use std::fmt::Display;

use serde::Serialize;

use super::{
    Code, Collapsible, Embed, Heading, Highlight, Image, Images, List, Paragraph, ThematicBreak,
};

#[derive(Debug, PartialEq, Serialize, Clone, Eq)]
#[serde(tag = "type", content = "value")]
pub enum YamdNodes {
    Pargargaph(Paragraph),
    Heading(Heading),
    Image(Image),
    Images(Images),
    Code(Code),
    List(List),
    Highlight(Highlight),
    ThematicBreak(ThematicBreak),
    Embed(Embed),
    Collapsible(Collapsible),
}

impl From<Paragraph> for YamdNodes {
    fn from(value: Paragraph) -> Self {
        YamdNodes::Pargargaph(value)
    }
}

impl From<Heading> for YamdNodes {
    fn from(value: Heading) -> Self {
        YamdNodes::Heading(value)
    }
}

impl From<Image> for YamdNodes {
    fn from(value: Image) -> Self {
        YamdNodes::Image(value)
    }
}

impl From<Code> for YamdNodes {
    fn from(value: Code) -> Self {
        YamdNodes::Code(value)
    }
}

impl From<List> for YamdNodes {
    fn from(value: List) -> Self {
        YamdNodes::List(value)
    }
}

impl From<Images> for YamdNodes {
    fn from(value: Images) -> Self {
        YamdNodes::Images(value)
    }
}

impl From<Highlight> for YamdNodes {
    fn from(value: Highlight) -> Self {
        YamdNodes::Highlight(value)
    }
}

impl From<ThematicBreak> for YamdNodes {
    fn from(value: ThematicBreak) -> Self {
        YamdNodes::ThematicBreak(value)
    }
}

impl From<Embed> for YamdNodes {
    fn from(value: Embed) -> Self {
        YamdNodes::Embed(value)
    }
}

impl From<Collapsible> for YamdNodes {
    fn from(value: Collapsible) -> Self {
        YamdNodes::Collapsible(value)
    }
}

impl Display for YamdNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YamdNodes::Pargargaph(node) => write!(f, "{}", node),
            YamdNodes::Heading(node) => write!(f, "{}", node),
            YamdNodes::Image(node) => write!(f, "{}", node),
            YamdNodes::Images(node) => write!(f, "{}", node),
            YamdNodes::Code(node) => write!(f, "{}", node),
            YamdNodes::List(node) => write!(f, "{}", node),
            YamdNodes::Highlight(node) => write!(f, "{}", node),
            YamdNodes::ThematicBreak(node) => write!(f, "{}", node),
            YamdNodes::Embed(node) => write!(f, "{}", node),
            YamdNodes::Collapsible(node) => write!(f, "{}", node),
        }
    }
}

/// # Yamd
///
/// [Metadata](Yamd::metadata) is optional Frontmatter.
///
/// Can be only in the beginning of the document surrounded by [Minus](type@crate::lexer::TokenKind::Minus)
/// of length 3 followed by [EOL](type@crate::lexer::TokenKind::Eol) and [EOL](type@crate::lexer::TokenKind::Eol)
/// followed by [Minus](type@crate::lexer::TokenKind::Minus) of length 3. Can contain any string that is
/// parsable by the consumer.
///
/// For example toml:
///
/// ```text
/// ---
/// title: "Yamd"
/// tags:
/// - software
/// - rust
/// ---
/// ```
///
/// [Body](Yamd::body) can contain one or more:
///
/// - [Paragraph]
/// - [Heading]
/// - [Image]
/// - [Images]
/// - [Code]
/// - [List]
/// - [Highlight]
/// - [ThematicBreak]
/// - [Embed]
/// - [Collapsible]
///
/// Separated by [Terminator](type@crate::lexer::TokenKind::Terminator).
///
/// Example:
///
/// ~~~markdown
/// Yamd can contain a Paragraph. Or a
///
/// # Heading
///
/// Or one image:
///
/// ![alt](src)
///
/// Or code:
///
/// ```rust
/// let a="or a code block";
/// ```
///
/// Or unordered list:
///
/// - Level 0
///  - Level 1
///
/// Or ordered list:
/// + Level 0
/// + Level 0
///  + Level 1
///
/// It also can have a thematic break:
///
/// -----
///
/// Or embed:
///
/// {{youtube|url}}
///
/// Or multiple images combined into gallery. There is no 1:1 match for that in HTML, and multiple
/// ways to do it depending on how it will be rendered:
///
/// ![alt](src)
/// ![alt](src)
///
/// Or a highlight:
///
/// >> Highlight title
/// > warning
/// There is no 1:1 equivalent to a highlight in HTML.
///
/// Highlight body can contain multiple paragraphs.
/// >>
///
/// {% Or collapsible
/// Which is also does not have 1:1 equivalent in HTML
/// %}
/// ~~~
///
/// HTML equivalent:
///
/// ```html
/// <p>Yamd can contain a Paragraph. Or a</p>
/// <h1>Heading</h1>
/// <p>Or one image:</p>
/// <img src="url" alt="alt"/>
/// <p>Or code:</p>
/// <pre><code>let a="or a code block";</code></pre>
/// <p>Or unordered list:</p>
/// <ul>
///     <li>
///         Level 0
///         <ul>
///             <li>Level 1</li>
///         </ul>
///     </li>
/// </ul>
/// <p>Or ordered list:</p>
/// <ol>
///     <li>Level 0</li>
///     <li>
///         Level 1
///         <ol>
///             <li>Level 1</li>
///         </ol>
///     </li>
/// </ol>
/// <p>It also can have a thematic break:</p>
/// <hr/>
/// <p>Or embed:</p>
/// <iframe class="youtube" src="url" />
/// <p>Or multiple images combined into gallery, there is no 1:1 match for that in HTML, and multiple
/// ways to do it depending on how it will be rendered:</p>
/// <div class="images">
///     <img src="url" alt="alt"/>
///     <img src="url" alt="alt"/>
/// </div>
/// <p>Or a highlight:</p>
/// <div class="highlight">
///     <div class="icon warning"></div>
///     <div class="body">
///         <h3>Highlight title</h3>
///         <p>There is no 1:1 equivalent to a highlight in HTML.</p>
///         <p>Highlight body can contain multiple paragraphs.</p>
///     </div>
/// </div>
/// <div class="collapsible">
///     <input type="checkbox" id="{{ node.title }}" />
///     <label for="{{ node.title }}">Or collapsible</label>
///     <div class="body">
///         <p>Which is also does not have 1:1 equivalent in HTML</p>
///     </div>
/// </div>
/// ```
///

#[derive(Debug, PartialEq, Serialize, Clone, Default, Eq)]
pub struct Yamd {
    pub metadata: Option<String>,
    pub body: Vec<YamdNodes>,
}

impl Yamd {
    pub fn new(metadata: Option<String>, body: Vec<YamdNodes>) -> Self {
        Self { metadata, body }
    }
}

impl Display for Yamd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(metadata) = &self.metadata {
            writeln!(f, "---")?;
            writeln!(f, "{}", metadata)?;
            write!(f, "---\n\n")?;
        }

        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<_>>()
                .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::{Collapsible, Highlight, Paragraph, Yamd};

    #[test]
    fn test_yamd() {
        let yamd = Yamd::new(
            Some("title: \"Yamd\"".to_string()),
            vec![
                Paragraph::new(vec!["paragraph".to_string().into()]).into(),
                Highlight::new(
                    Some("Highlight"),
                    Some("warning"),
                    vec![Paragraph::new(vec!["body".to_string().into()])],
                )
                .into(),
                Collapsible::new(
                    "Or collapsible",
                    vec![Paragraph::new(vec!["body".to_string().into()]).into()],
                )
                .into(),
            ],
        );

        assert_eq!(
            yamd.to_string(),
            "---\ntitle: \"Yamd\"\n---\n\nparagraph\n\n!! Highlight\n! warning\nbody\n!!\n\n{% Or collapsible\nbody\n%}"
        );
    }
}
