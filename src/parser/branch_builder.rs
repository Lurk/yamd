use super::Parser;

pub struct BranchBuilder<Leaves: From<String>> {
    nodes: Vec<Leaves>,
    text_start: Option<usize>,
}

impl<Leaves: From<String>> BranchBuilder<Leaves> {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            text_start: None,
        }
    }
    pub fn push<N: Into<Leaves>>(&mut self, n: Option<N>, p: &Parser, pos: usize) {
        if let Some(n) = n {
            self.consume_text(p, pos);
            self.nodes.push(n.into());
        }
    }

    pub fn start_text(&mut self, pos: usize) {
        self.text_start.get_or_insert(pos);
    }

    pub fn consume_text(&mut self, p: &Parser, end: usize) {
        if let Some(start) = self.text_start.take() {
            self.nodes.push(p.range_to_string(start..end).into());
        }
    }

    pub fn clear_text_if_shorter_than(&mut self, pos: usize, size: usize) {
        self.text_start.take_if(|start| pos - *start < size);
    }

    pub fn build<Branch: From<Vec<Leaves>>>(self) -> Option<Branch> {
        if self.nodes.is_empty() {
            return None;
        }
        Some(self.nodes.into())
    }
}
