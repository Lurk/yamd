use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Anchor,
    Bold,
    Code,
    CodeSpan,
    Collapsible,
    Destination,
    Document,
    Embed,
    Emphasis,
    Heading,
    Highlight,
    Icon,
    Image,
    Images,
    Italic,
    ListItem,
    Modifier,
    OrderedList,
    Paragraph,
    Strikethrough,
    Text,
    ThematicBreak,
    Title,
    UnorderedList,
}

#[derive(Debug, PartialEq)]
pub enum OpKind {
    Start(Node),
    End(Node),
    Value,
}

#[derive(Debug, PartialEq)]
pub struct Op<'a> {
    pub kind: OpKind,
    pub tokens: Vec<&'a Token<'a>>,
}

impl<'a> Op<'a> {
    pub fn new_value(tokens: Vec<&'a Token<'a>>) -> Self {
        Self {
            kind: OpKind::Value,
            tokens,
        }
    }

    pub fn new_start(node: Node, tokens: Vec<&'a Token<'a>>) -> Self {
        Self {
            kind: OpKind::Start(node),
            tokens,
        }
    }

    pub fn new_end(node: Node, tokens: Vec<&'a Token<'a>>) -> Self {
        Self {
            kind: OpKind::End(node),
            tokens,
        }
    }
}

impl From<Op<'_>> for String {
    fn from(op: Op<'_>) -> Self {
        op.tokens.iter().map(|t| t.slice).collect()
    }
}
