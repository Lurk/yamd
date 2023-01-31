pub trait Parser {
    fn parse(input: &str, start_position: usize) -> Option<(Self, usize)>
    where
        Self: Sized;
}
