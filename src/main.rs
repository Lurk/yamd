pub trait ToParagraph {
    fn to_paragraph(self) -> ParagraphContent;
}

pub struct B {
    text: String,
}

impl ToParagraph for B {
    fn to_paragraph(self) -> ParagraphContent {
        ParagraphContent::B(self)
    }
}
impl B {
    pub fn new(text: String) -> Self {
        B { text }
    }
}

pub struct A {
    text: Option<String>,
    url: String,
}

impl ToParagraph for A {
    fn to_paragraph(self) -> ParagraphContent {
        ParagraphContent::A(self)
    }
}

impl A {
    pub fn new<S: Into<String>>(url: S, text: Option<String>) -> Self {
        A {
            text,
            url: url.into(),
        }
    }
}

pub struct H {
    level: u8,
    text: String,
}

impl ToParagraph for H {
    fn to_paragraph(self) -> ParagraphContent {
        ParagraphContent::H(self)
    }
}

impl H {
    pub fn new<S: Into<String>>(text: S, level: u8) -> Self {
        H {
            text: text.into(),
            level,
        }
    }
}

pub enum ParagraphContent {
    A(A),
    B(B),
    H(H),
    Text(String),
}

pub struct P {
    data: Vec<ParagraphContent>,
}

impl ToTree for P {
    fn to_tree(self) -> TreeContent {
        TreeContent::P(self)
    }
}

impl P {
    pub fn new() -> Self {
        P { data: vec![] }
    }

    pub fn push<TP: ToParagraph>(mut self, element: TP) -> Self {
        self.data.push(element.to_paragraph());
        self
    }
}

pub enum TreeContent {
    P(P),
}

pub trait ToTree {
    fn to_tree(self) -> TreeContent;
}

struct Tree {
    data: Vec<TreeContent>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { data: vec![] }
    }

    pub fn push<TT: ToTree>(mut self, element: TT) -> Self {
        self.data.push(element.to_tree());
        self
    }
}

fn main() {
    let p = P::new().push(H::new("foo", 1)).push(A::new(
        "http://foo.bar//",
        Some("http://foo.bar//".to_string()),
    ));
    let t = Tree::new().push(p);
    println!("Hello, world!");
}
