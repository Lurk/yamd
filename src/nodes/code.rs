use crate::sd::{
    deserializer::{Deserializer, Node, Tokenizer},
    serializer::Serializer,
};

use super::yamd::YamdNodes;

#[derive(Debug, PartialEq)]
pub struct Code {
    lang: String,
    code: String,
}

impl Code {
    pub fn new<S: Into<String>>(lang: S, code: S) -> Self {
        Self {
            lang: lang.into(),
            code: code.into(),
        }
    }
}

impl Serializer for Code {
    fn serialize(&self) -> String {
        format!("```{}\n{}\n```", self.lang, self.code)
    }
}

impl Node for Code {
    fn len(&self) -> usize {
        self.lang.len() + self.code.len() + 8
    }
}

impl Deserializer for Code {
    fn deserialize(input: &str) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(lang_body) = tokenizer.get_token_body(vec!['`', '`', '`'], vec!['\n']) {
            let lang_body = lang_body.to_string();
            if let Some(code_boy) = tokenizer.get_token_body(vec![], vec!['\n', '`', '`', '`']) {
                return Some(Self::new(lang_body, code_boy.to_string()));
            }
        }
        None
    }
}

impl From<Code> for YamdNodes {
    fn from(value: Code) -> Self {
        YamdNodes::Code(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::code::Code,
        sd::{
            deserializer::{Deserializer, Node},
            serializer::Serializer,
        },
    };

    #[test]
    fn serialize() {
        assert_eq!(
            Code::new("rust", "let foo:usize=1;").serialize(),
            String::from("```rust\nlet foo:usize=1;\n```")
        );
    }

    #[test]
    fn len() {
        assert_eq!(Code::new('r', 'b').len(), 10);
    }

    #[test]
    fn deserializer() {
        assert_eq!(
            Code::deserialize("```rust\nlet a=1;\n```"),
            Some(Code::new("rust", "let a=1;"))
        );
    }
}
