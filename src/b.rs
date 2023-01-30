use crate::{i::I, p::ParagraphContent, s::S, text::Text};

#[derive(Debug)]
pub enum BContent {
    Text(Text),
    I(I),
    S(S),
}

#[derive(Debug)]
pub struct B {
    data: Vec<BContent>,
}

impl From<B> for ParagraphContent {
    fn from(value: B) -> Self {
        ParagraphContent::B(value)
    }
}

impl From<B> for String {
    fn from(value: B) -> Self {
        format!(
            "**{}**",
            value
                .data
                .into_iter()
                .map(|element| {
                    match element {
                        BContent::Text(v) => v.into(),
                        BContent::I(v) => v.into(),
                        BContent::S(v) => v.into(),
                    }
                })
                .collect::<Vec<String>>()
                .concat()
        )
    }
}

impl B {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from_vec(data: Vec<BContent>) -> Self {
        Self { data }
    }

    pub fn push<BC: Into<BContent>>(mut self, element: BC) -> Self {
        self.data.push(element.into());
        self
    }
}

impl Default for B {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{b::B, i::I, s::S, text::Text};

    #[test]
    fn only_text() {
        let b: String = B::new().push(Text::new("B as bold")).into();
        assert_eq!(b, "**B as bold**".to_string());
    }

    #[test]
    fn multilpe_entries() {
        let b: String = B::new()
            .push(Text::new("B as bold "))
            .push(I::new("Italic"))
            .push(S::new("Strikethrough"))
            .into();
        assert_eq!(b, "**B as bold *Italic*~~Strikethrough~~**".to_string());
    }

    #[test]
    fn from_vec() {
        let b: String = B::from_vec(vec![
            Text::new("B as bold ").into(),
            I::new("Italic").into(),
            S::new("Strikethrough").into(),
        ])
        .into();
        assert_eq!(b, "**B as bold *Italic*~~Strikethrough~~**".to_string());
    }
}
