use crate::{h::H, p::P};

#[derive(Debug)]
pub enum TreeContent {
    P(P),
    H(H),
}

impl From<P> for TreeContent {
    fn from(value: P) -> Self {
        TreeContent::P(value)
    }
}

impl From<H> for TreeContent {
    fn from(value: H) -> Self {
        TreeContent::H(value)
    }
}

#[derive(Debug)]
pub struct Tree {
    data: Vec<TreeContent>,
}

impl Tree {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from_vec(data: Vec<TreeContent>) -> Self {
        Self { data }
    }

    pub fn push<TC: Into<TreeContent>>(mut self, element: TC) -> Self {
        self.data.push(element.into());
        self
    }
}

impl From<Tree> for String {
    fn from(value: Tree) -> Self {
        value
            .data
            .into_iter()
            .map(|element| match element {
                TreeContent::P(v) => v.into(),
                TreeContent::H(v) => v.into(),
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{h::H, p::P, text::Text};

    use super::Tree;

    #[test]
    fn push() {
        let t: String = Tree::new()
            .push(H::new("header", 1))
            .push(P::new().push(Text::new("text")))
            .into();

        assert_eq!(t, "# header\n\ntext".to_string());
    }

    #[test]
    fn from_vec() {
        let t: String = Tree::from_vec(vec![
            H::new("header", 1).into(),
            P::new().push(Text::new("text")).into(),
        ])
        .into();

        assert_eq!(t, "# header\n\ntext".to_string());
    }
}
