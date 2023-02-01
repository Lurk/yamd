use std::{iter::Enumerate, str::Chars};

pub trait Parser {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)>
    where
        Self: Sized;
}

pub trait ParserPart {
    fn parse_part(&mut self, start_char: char, end_char: char) -> Option<usize>;
}

impl<'a> ParserPart for Enumerate<Chars<'a>> {
    fn parse_part(&mut self, start_char: char, end_char: char) -> Option<usize> {
        if let Some((_, char)) = self.next() {
            if char == start_char {
                let mut already_seen = false;
                for (index, char) in self.by_ref() {
                    // while let Some((index, char)) = self.next() {
                    match char {
                        c if c == end_char => return Some(index),
                        '\n' => {
                            if already_seen {
                                return None;
                            }
                            already_seen = true;
                        }
                        _ => {
                            already_seen = false;
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::ParserPart;

    #[test]
    fn parse_part() {
        let mut c = "test of *italic**one more* statement".chars().enumerate();
        c.nth(7);
        assert_eq!(c.parse_part('*', '*'), Some(15));
        assert_eq!(c.parse_part('*', '*'), Some(25));
    }
}
