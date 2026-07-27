//! The AST types produced by [`deserialize`](crate::deserialize). Every node is a plain struct
//! that implements [`Display`](std::fmt::Display) for round-tripping back to markdown.
//!
//! Start with [`Yamd`], the document root; its body is a `Vec<`[`YamdNodes`]`>`.
//!
//! # Block nodes
//!
//! Variants of [`YamdNodes`]:
//!
//! - [`Paragraph`] — a run of inline nodes ([`ParagraphNodes`])
//! - [`Heading`] — `#`..`######`, with inline content ([`HeadingNodes`])
//! - [`List`] — ordered or unordered, see [`ListTypes`], made up of [`ListItem`]s
//! - [`Code`] — fenced code block
//! - [`Image`] / [`Images`] — a single image, or a group of images
//! - [`Highlight`] — a callout/admonition block
//! - [`Collapsible`] — a `<details>`-style disclosure block
//! - [`ThematicBreak`] — `---`
//! - [`Embed`] — an embedded external resource
//!
//! # Inline nodes
//!
//! Variants of [`ParagraphNodes`] and [`HeadingNodes`]:
//!
//! - [`Anchor`] — a link
//! - [`Bold`] ([`BoldNodes`]) / [`Italic`] / [`Emphasis`] / [`Strikethrough`] — inline text styling
//! - [`CodeSpan`] — inline code
//! - `String` — plain text
//!
//! With the `serde` feature enabled, every node above is also
//! [`Serialize`](serde::Serialize)/[`Deserialize`](serde::Deserialize).

mod anchor;
mod bold;
mod code;
mod code_span;
mod collapsible;
mod embed;
mod emphasis;
mod heading;
mod highlight;
mod image;
mod images;
mod italic;
mod list;
mod list_item;
mod paragraph;
mod strikethrough;
mod thematic_break;
mod yamd;

pub use anchor::Anchor;
pub use bold::{Bold, BoldNodes};
pub use code::Code;
pub use code_span::CodeSpan;
pub use collapsible::Collapsible;
pub use embed::Embed;
pub use emphasis::Emphasis;
pub use heading::{Heading, HeadingNodes};
pub use highlight::Highlight;
pub use image::Image;
pub use images::Images;
pub use italic::Italic;
pub use list::{List, ListTypes};
pub use list_item::ListItem;
pub use paragraph::{Paragraph, ParagraphNodes};
pub use strikethrough::Strikethrough;
pub use thematic_break::ThematicBreak;
pub use yamd::{Yamd, YamdNodes};
