pub use crate::op::op::Op;
pub use crate::op::parser::Parser;
use crate::op::parser::Query;

mod anchor;
mod bold;
mod code;
mod code_span;
mod collapsible;
mod destination;
mod document;
mod embed;
mod emphasis;
mod heading;
mod highlight;
mod image;
mod images;
mod italic;
mod list;
mod modifier;
mod op;
mod paragraph;
pub mod parser;
mod strikethrough;
mod thematic_break;
mod title;

pub fn parse<'a>(parser: &'a Parser) -> Vec<Op<'a>> {
    document::document(parser, &Query::Eof)
}
