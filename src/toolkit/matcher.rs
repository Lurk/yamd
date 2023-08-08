pub struct Matcher<'input> {
    input: &'input str,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub struct Match<'input> {
    pub start_token: &'input str,
    pub body: &'input str,
    pub end_token: &'input str,
}

impl<'input> Matcher<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { input, position: 0 }
    }

    pub fn get_match(
        &mut self,
        start_sequence: &'input str,
        end_sequence: &'input str,
        match_end_of_input: bool,
    ) -> Option<Match<'input>> {
        if !match_end_of_input
            && !start_sequence.is_empty()
            && !end_sequence.is_empty()
            && start_sequence.len() == end_sequence.len()
            && start_sequence != end_sequence
        {
            return self.get_balanced_match(start_sequence, end_sequence);
        }
        self.get_unbalanced_match(start_sequence, end_sequence, match_end_of_input)
    }

    fn iterate(
        &self,
        sequence: &str,
        start_position: usize,
        fail_fast: bool,
        match_end_of_input: bool,
    ) -> Option<(usize, usize)> {
        if sequence.is_empty() {
            return Some((start_position, 0));
        } else {
            if match_end_of_input && start_position == self.input.len() {
                return Some((start_position, 0));
            }
            for index in start_position..self.input.len() {
                if self.input[index..].starts_with(sequence) {
                    return Some((index + sequence.len(), sequence.len()));
                } else if match_end_of_input && index == self.input.len() - 1 {
                    return Some((index + 1, 0));
                } else if fail_fast {
                    return None;
                }
            }
        }
        None
    }

    fn get_unbalanced_match(
        &mut self,
        start_sequence: &'input str,
        end_sequence: &'input str,
        match_end_of_input: bool,
    ) -> Option<Match<'input>> {
        if let Some((start_token_end_index, start_token_lenght)) =
            self.iterate(start_sequence, self.position, true, false)
        {
            if let Some((end_token_end_index, end_token_length)) = self.iterate(
                end_sequence,
                start_token_end_index,
                false,
                match_end_of_input,
            ) {
                self.position = end_token_end_index;
                return Some(Match {
                    start_token: &self.input
                        [(start_token_end_index - start_token_lenght)..start_token_end_index],
                    body: &self.input
                        [start_token_end_index..end_token_end_index - end_token_length],
                    end_token: &self.input
                        [(end_token_end_index - end_token_length)..end_token_end_index],
                });
            }
        }
        None
    }

    fn get_balanced_match(
        &mut self,
        start_sequence: &'input str,
        end_sequence: &'input str,
    ) -> Option<Match<'input>> {
        let mut balance = 1;
        if let Some((start_token_end_index, _)) =
            self.iterate(start_sequence, self.position, true, false)
        {
            for index in start_token_end_index..self.input.len() {
                if self.input[index..].starts_with(start_sequence) {
                    balance += 1;
                } else if self.input[index..].starts_with(end_sequence) {
                    balance -= 1;
                    if balance == 0 {
                        let end_token_end_index = index + end_sequence.len();
                        self.position = end_token_end_index;
                        return Some(Match {
                            start_token: &self.input[..start_token_end_index],
                            body: &self.input[start_token_end_index..index],
                            end_token: &self.input[index..end_token_end_index],
                        });
                    }
                }
            }
        }
        None
    }

    pub fn get_rest(&self) -> &'input str {
        &self.input[self.position..]
    }
}

#[cfg(test)]
mod tests {
    use crate::toolkit::matcher::{Match, Matcher};
    use pretty_assertions::assert_eq;

    #[test]
    fn get_match() {
        let mut matcher = Matcher::new("*italic*~~one more~~ statement");
        assert_eq!(
            matcher.get_match("*", "*", false),
            Some(Match {
                start_token: "*",
                body: "italic",
                end_token: "*"
            })
        );
        assert_eq!(
            matcher.get_match("~~", "~~", false),
            Some(Match {
                start_token: "~~",
                body: "one more",
                end_token: "~~",
            })
        );
    }

    #[test]
    fn get_match_with_end_of_input() {
        let mut matcher = Matcher::new("*italic*~~one more");
        assert_eq!(
            matcher.get_match("*", "*", false),
            Some(Match {
                start_token: "*",
                body: "italic",
                end_token: "*"
            })
        );
        assert_eq!(
            matcher.get_match("~~", "~~", true),
            Some(Match {
                start_token: "~~",
                body: "one more",
                end_token: "",
            })
        );
    }

    #[test]
    fn get_match_with_empty_start_token_and_macth_end() {
        let mut matcher = Matcher::new("t");
        assert_eq!(
            matcher.get_match("", "\n", true),
            Some(Match {
                start_token: "",
                body: "t",
                end_token: ""
            })
        )
    }

    #[test]
    fn get_match_with_empty_body() {
        let mut matcher = Matcher::new("--");
        assert_eq!(
            matcher.get_match("--", "\n", true),
            Some(Match {
                start_token: "--",
                body: "",
                end_token: ""
            })
        )
    }

    #[test]
    fn patterns_with_non_equal_length_can_not_be_balanced() {
        let mut matcher = Matcher::new("(()t");
        assert_eq!(
            matcher.get_match("(", ")t", false),
            Some(Match {
                start_token: "(",
                body: "(",
                end_token: ")t"
            })
        )
    }

    #[test]
    fn get_balanced_match() {
        let mut matcher = Matcher::new("{{}}");
        assert_eq!(
            matcher.get_match("{", "}", false),
            Some(Match {
                start_token: "{",
                body: "{}",
                end_token: "}"
            })
        );
    }
}
