use std::usize;

#[derive(Debug, PartialEq)]
pub enum Token {
    DoubleNewLine,
    NewLine,
    Text(String),
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    DoubleStar,
    TrippleTick,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenCurlyBracePercent,
    PercentCloseCurlyBrace,
    ExclamationMark,
    Pipe,
    Minus,
    Hashtag(usize),
}

struct Tokenizer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
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

    fn one_more(&mut self, peek_for: char, token: Token) -> Option<()> {
        if let Some(&c) = self.chars.peek() {
            if c == peek_for {
                self.chars.next();
                self.tokens.push(token);
                return Some(());
            }
        }
        None
    }

    fn two_more(&mut self, peek_for: char, token: Token) -> Option<()> {
        if let Some(&c) = self.chars.peek() {
            if c == peek_for {
                self.chars.next();
                if let Some(&c) = self.chars.peek() {
                    if c == peek_for {
                        self.chars.next();
                        self.tokens.push(token);
                        return Some(());
                    } else {
                        self.push_char(peek_for);
                        return None;
                    }
                }
            }
        }
        None
    }
    pub fn tokenize(mut self) -> Vec<Token> {
        while let Some(c) = self.chars.next() {
            match c {
                '\n' => {
                    if self.one_more('\n', Token::DoubleNewLine).is_none() {
                        self.tokens.push(Token::NewLine);
                    }
                }
                '[' => self.tokens.push(Token::OpenBracket),
                ']' => self.tokens.push(Token::CloseBracket),
                '(' => self.tokens.push(Token::OpenParen),
                ')' => self.tokens.push(Token::CloseParen),
                '*' => {
                    if self.one_more(c, Token::DoubleStar).is_none() {
                        self.push_char(c);
                    }
                }
                '`' => {
                    if self.two_more(c, Token::TrippleTick).is_none() {
                        self.push_char(c);
                    }
                }
                '{' => {
                    if self.one_more('%', Token::OpenCurlyBracePercent).is_none() {
                        self.tokens.push(Token::OpenCurlyBrace);
                    }
                }
                '}' => self.tokens.push(Token::CloseCurlyBrace),
                '%' => {
                    if self.one_more('}', Token::PercentCloseCurlyBrace).is_none() {
                        self.push_char(c);
                    }
                }
                '!' => self.tokens.push(Token::ExclamationMark),
                '|' => self.tokens.push(Token::Pipe),
                '-' => self.tokens.push(Token::Minus),
                '#' => {
                    let mut next = self.chars.peek();
                    let mut count = 1;
                    while Some(&'#') == next {
                        count += 1;
                        self.chars.next();
                        next = self.chars.peek();
                    }
                    self.tokens.push(Token::Hashtag(count));
                }
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
        let tokenizer = Tokenizer::new("H\n\nW\n\n\n[a](u)**b*`````a{{{s}a}}{%%}%!![|-]###");
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens.len(), 33);
        assert_eq!(tokens[0], Token::Text("H".to_string()));
        assert_eq!(tokens[1], Token::DoubleNewLine);
        assert_eq!(tokens[2], Token::Text("W".to_string()));
        assert_eq!(tokens[3], Token::DoubleNewLine);
        assert_eq!(tokens[4], Token::NewLine);
        assert_eq!(tokens[5], Token::OpenBracket);
        assert_eq!(tokens[6], Token::Text("a".to_string()));
        assert_eq!(tokens[7], Token::CloseBracket);
        assert_eq!(tokens[8], Token::OpenParen);
        assert_eq!(tokens[9], Token::Text("u".to_string()));
        assert_eq!(tokens[10], Token::CloseParen);
        assert_eq!(tokens[11], Token::DoubleStar);
        assert_eq!(tokens[12], Token::Text("b*".to_string()));
        assert_eq!(tokens[13], Token::TrippleTick);
        assert_eq!(tokens[14], Token::Text("``a".to_string()));
        assert_eq!(tokens[15], Token::OpenCurlyBrace);
        assert_eq!(tokens[16], Token::OpenCurlyBrace);
        assert_eq!(tokens[17], Token::OpenCurlyBrace);
        assert_eq!(tokens[18], Token::Text("s".to_string()));
        assert_eq!(tokens[19], Token::CloseCurlyBrace);
        assert_eq!(tokens[20], Token::Text("a".to_string()));
        assert_eq!(tokens[21], Token::CloseCurlyBrace);
        assert_eq!(tokens[22], Token::CloseCurlyBrace);
        assert_eq!(tokens[23], Token::OpenCurlyBracePercent);
        assert_eq!(tokens[24], Token::PercentCloseCurlyBrace);
        assert_eq!(tokens[25], Token::Text("%".to_string()));
        assert_eq!(tokens[26], Token::ExclamationMark);
        assert_eq!(tokens[27], Token::ExclamationMark);
        assert_eq!(tokens[28], Token::OpenBracket);
        assert_eq!(tokens[29], Token::Pipe);
        assert_eq!(tokens[30], Token::Minus);
        assert_eq!(tokens[31], Token::CloseBracket);
        assert_eq!(tokens[32], Token::Hashtag(3));
    }
}
