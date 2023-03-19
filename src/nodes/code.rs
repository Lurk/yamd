use crate::toolkit::{
    context::Context,
    deserializer::Deserializer,
    node::Node,
    tokenizer::{Pattern::Once, Tokenizer},
};

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

impl Node for Code {
    fn len(&self) -> usize {
        self.lang.len() + self.code.len() + 8
    }
    fn serialize(&self) -> String {
        format!("```{}\n{}\n```", self.lang, self.code)
    }
}

impl Deserializer for Code {
    fn deserialize_with_context(input: &str, _: Option<Context>) -> Option<Self> {
        let mut tokenizer = Tokenizer::new(input);
        if let Some(lang_body) =
            tokenizer.get_node_body(&[Once('`'), Once('`'), Once('`')], &[Once('\n')])
        {
            let lang_body = lang_body.to_string();
            if let Some(code_boy) =
                tokenizer.get_node_body(&[], &[Once('\n'), Once('`'), Once('`'), Once('`')])
            {
                return Some(Self::new(lang_body, code_boy.to_string()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        nodes::code::Code,
        toolkit::{deserializer::Deserializer, node::Node},
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
