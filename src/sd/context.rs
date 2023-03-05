#[derive(Debug)]
pub enum ContextValues {
    Usize(usize),
    String(String),
}

impl From<usize> for ContextValues {
    fn from(value: usize) -> Self {
        ContextValues::Usize(value)
    }
}

impl From<String> for ContextValues {
    fn from(value: String) -> Self {
        ContextValues::String(value)
    }
}

impl ContextValues {
    pub fn get_usize_value(&self) -> Option<usize> {
        if let ContextValues::Usize(value) = self {
            return Some(*value);
        }
        None
    }

    pub fn get_string_value(&self) -> Option<&String> {
        if let ContextValues::String(value) = self {
            return Some(value);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::ContextValues;

    #[test]
    fn usize_value() {
        let ctx: ContextValues = 1.into();
        assert_eq!(ctx.get_usize_value(), Some(1));
        assert_eq!(ctx.get_string_value(), None)
    }

    #[test]
    fn string_value() {
        let ctx: ContextValues = String::new().into();
        assert_eq!(ctx.get_string_value(), Some(&String::new()));
        assert_eq!(ctx.get_usize_value(), None)
    }
}
