use std::usize;

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    NewLine,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Star,
    Tick,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Percent,
    ExclamationMark,
    Pipe,
    Minus,
    Hashtag,
    Space,
    Underscore,
    Tilde,
}

struct Tokenizer<'a> {
    chars: std::str::Chars<'a>,
    tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            tokens: Vec::new(),
        }
    }

    fn push_char(&mut self, c: char) {
        if let Some(Token::Text(t)) = self.tokens.last_mut() {
            t.push(c);
        } else {
            self.tokens.push(Token::Text(c.to_string()));
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        while let Some(c) = self.chars.next() {
            match c {
                '\n' => self.tokens.push(Token::NewLine),
                '[' => self.tokens.push(Token::OpenBracket),
                ']' => self.tokens.push(Token::CloseBracket),
                '(' => self.tokens.push(Token::OpenParen),
                ')' => self.tokens.push(Token::CloseParen),
                '*' => self.tokens.push(Token::Star),
                '`' => self.tokens.push(Token::Tick),
                '{' => self.tokens.push(Token::OpenCurlyBrace),
                '}' => self.tokens.push(Token::CloseCurlyBrace),
                '%' => self.tokens.push(Token::Percent),
                '!' => self.tokens.push(Token::ExclamationMark),
                '|' => self.tokens.push(Token::Pipe),
                '-' => self.tokens.push(Token::Minus),
                '#' => self.tokens.push(Token::Hashtag),
                ' ' => self.tokens.push(Token::Space),
                '_' => self.tokens.push(Token::Underscore),
                '~' => self.tokens.push(Token::Tilde),
                _ => self.push_char(c),
            }
        }
        self.tokens
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_tokenize() {
        let tokenizer = Tokenizer::new("H\n\nW\n\n\n[a](u)**b*```a{{{s}a}}{%%}%!![|-]### _");
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens.len(), 45);
        assert_eq!(tokens[0], Token::Text("H".to_string()));
        assert_eq!(tokens[1], Token::NewLine);
        assert_eq!(tokens[2], Token::NewLine);
        assert_eq!(tokens[3], Token::Text("W".to_string()));
        assert_eq!(tokens[4], Token::NewLine);
        assert_eq!(tokens[5], Token::NewLine);
        assert_eq!(tokens[6], Token::NewLine);
        assert_eq!(tokens[7], Token::OpenBracket);
        assert_eq!(tokens[8], Token::Text("a".to_string()));
        assert_eq!(tokens[9], Token::CloseBracket);
        assert_eq!(tokens[10], Token::OpenParen);
        assert_eq!(tokens[11], Token::Text("u".to_string()));
        assert_eq!(tokens[12], Token::CloseParen);
        assert_eq!(tokens[13], Token::Star);
        assert_eq!(tokens[14], Token::Star);
        assert_eq!(tokens[15], Token::Text("b".to_string()));
        assert_eq!(tokens[16], Token::Star);
        assert_eq!(tokens[17], Token::Tick);
        assert_eq!(tokens[18], Token::Tick);
        assert_eq!(tokens[19], Token::Tick);
        assert_eq!(tokens[20], Token::Text("a".to_string()));
        assert_eq!(tokens[21], Token::OpenCurlyBrace);
        assert_eq!(tokens[22], Token::OpenCurlyBrace);
        assert_eq!(tokens[23], Token::OpenCurlyBrace);
        assert_eq!(tokens[24], Token::Text("s".to_string()));
        assert_eq!(tokens[25], Token::CloseCurlyBrace);
        assert_eq!(tokens[26], Token::Text("a".to_string()));
        assert_eq!(tokens[27], Token::CloseCurlyBrace);
        assert_eq!(tokens[28], Token::CloseCurlyBrace);
        assert_eq!(tokens[29], Token::OpenCurlyBrace);
        assert_eq!(tokens[30], Token::Percent);
        assert_eq!(tokens[31], Token::Percent);
        assert_eq!(tokens[32], Token::CloseCurlyBrace);
        assert_eq!(tokens[33], Token::Percent);
        assert_eq!(tokens[34], Token::ExclamationMark);
        assert_eq!(tokens[35], Token::ExclamationMark);
        assert_eq!(tokens[36], Token::OpenBracket);
        assert_eq!(tokens[37], Token::Pipe);
        assert_eq!(tokens[38], Token::Minus);
        assert_eq!(tokens[39], Token::CloseBracket);
        assert_eq!(tokens[40], Token::Hashtag);
        assert_eq!(tokens[41], Token::Hashtag);
        assert_eq!(tokens[42], Token::Hashtag);
        assert_eq!(tokens[43], Token::Space);
        assert_eq!(tokens[44], Token::Underscore);
    }
}
