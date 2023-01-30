use crate::{h::H, p::P};

#[derive(Debug)]
pub enum MdyContent {
    P(P),
    H(H),
}

#[derive(Debug)]
pub struct Mdy {
    data: Vec<MdyContent>,
}

impl Mdy {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from_vec(data: Vec<MdyContent>) -> Self {
        Self { data }
    }

    pub fn push<TC: Into<MdyContent>>(mut self, element: TC) -> Self {
        self.data.push(element.into());
        self
    }
}

impl From<Mdy> for String {
    fn from(value: Mdy) -> Self {
        value
            .data
            .into_iter()
            .map(|element| match element {
                MdyContent::P(v) => v.into(),
                MdyContent::H(v) => v.into(),
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

impl Default for Mdy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{h::H, p::P, text::Text};

    use super::Mdy;

    #[test]
    fn push() {
        let t: String = Mdy::new()
            .push(H::new("header", 1))
            .push(P::new().push(Text::new("text")))
            .into();

        assert_eq!(t, "# header\n\ntext".to_string());
    }

    #[test]
    fn from_vec() {
        let t: String = Mdy::from_vec(vec![
            H::new("header", 1).into(),
            P::new().push(Text::new("text")).into(),
        ])
        .into();

        assert_eq!(t, "# header\n\ntext".to_string());
    }
}
