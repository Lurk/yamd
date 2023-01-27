use crate::{i::I, s::S, text::Text};

#[derive(Debug)]
pub enum BContent {
    Text(Text),
    I(I),
    S(S),
}

impl From<Text> for BContent {
    fn from(value: Text) -> Self {
        BContent::Text(value)
    }
}

impl From<I> for BContent {
    fn from(value: I) -> Self {
        BContent::I(value)
    }
}

impl From<S> for BContent {
    fn from(value: S) -> Self {
        BContent::S(value)
    }
}

#[derive(Debug)]
pub struct B {
    data: Vec<BContent>,
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
        B { data: vec![] }
    }

    pub fn from_vec(data: Vec<BContent>) -> Self {
        B { data }
    }

    fn push<BC: Into<BContent>>(mut self, element: BC) -> Self {
        self.data.push(element.into());
        self
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
