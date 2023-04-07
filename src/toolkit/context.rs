use std::collections::HashMap;

/// Context allows to pass arbitrary amount of key/value pairs between nodes in a type safe way
///
#[derive(Debug, Clone)]
pub enum ContextValues {
    Usize(usize),
    Char(char),
}

impl From<usize> for ContextValues {
    fn from(value: usize) -> Self {
        ContextValues::Usize(value)
    }
}

impl From<char> for ContextValues {
    fn from(value: char) -> Self {
        ContextValues::Char(value)
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    inner: HashMap<String, ContextValues>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: impl Into<String>, value: impl Into<ContextValues>) {
        self.inner.insert(key.into(), value.into());
    }

    pub fn get_usize_value(&self, key: impl Into<String>) -> Option<usize> {
        if let Some(ContextValues::Usize(value)) = self.inner.get(&key.into()) {
            return Some(*value);
        }
        None
    }

    pub fn get_char_value(&self, key: impl Into<String>) -> Option<char> {
        if let Some(ContextValues::Char(value)) = self.inner.get(&key.into()) {
            return Some(*value);
        }
        None
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    use pretty_assertions::assert_eq;

    #[test]
    fn usize_value() {
        let mut ctx = Context::new();
        ctx.add("usize_value", 1);

        assert_eq!(ctx.get_usize_value("usize_value"), Some(1));
        assert_eq!(ctx.get_usize_value("not_usize_value"), None);
    }

    #[test]
    fn char_value() {
        let mut ctx = Context::new();
        ctx.add("char_value", 'c');

        assert_eq!(ctx.get_char_value("char_value"), Some('c'));
        assert_eq!(ctx.get_char_value("not_char_value"), None);
    }

    #[test]
    fn default() {
        let mut ctx = Context::default();
        ctx.add("char_value", 'c');

        assert_eq!(ctx.get_char_value("char_value"), Some('c'));
        assert_eq!(ctx.get_char_value("not_char_value"), None);
    }
}
