use std::fmt::Display;

use super::{context::Context, node::Node};

pub trait Branch<BranchNodes>
where
    BranchNodes: Node + Display,
{
    fn push<CanBeNode: Into<BranchNodes>>(&mut self, node: CanBeNode);
    fn get_maybe_nodes() -> Vec<MaybeNode<BranchNodes>>;
    fn get_fallback_node() -> Option<DefinitelyNode<BranchNodes>>;
    fn get_outer_token_length(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn parse_branch(input: &str, delimeter: &str, mut branch: Self) -> Option<Self>
    where
        Self: Sized + Deserializer + Node,
    {
        let mut current_position = 0;
        let mut fallback_position = 0;
        let maybe_nodes = Self::get_maybe_nodes();
        while current_position < input.len() {
            let slice = &input[current_position..];
            let char_len = slice.chars().next().unwrap().len_utf8();
            current_position += char_len;
            for parser in &maybe_nodes {
                if let Some(node) = parser(slice, branch.context()) {
                    while fallback_position != current_position - char_len {
                        fallback_position = branch.fallback(
                            &input[fallback_position..current_position - char_len],
                            delimeter,
                        );
                    }
                    branch.push(node);
                    current_position = branch.len() - branch.get_outer_token_length();
                    fallback_position = current_position;
                    if !delimeter.is_empty() && input[current_position..].starts_with(delimeter) {
                        current_position += delimeter.len();
                        fallback_position = current_position;
                    }

                    break;
                }
            }
        }
        while fallback_position < input.len() {
            if Self::get_fallback_node().is_none() {
                return None;
            }
            fallback_position = branch.fallback(&input[fallback_position..], delimeter);
        }
        Some(branch)
    }

    fn fallback(&mut self, slice: &str, delimeter: &str) -> usize
    where
        Self: Node,
    {
        let node = Self::get_fallback_node()
            .map(|f| f(slice))
            .expect("Fallback node should always be available");
        let node_len = node.len();
        self.push(node);
        let mut fallback_position = self.len() - self.get_outer_token_length();
        if !delimeter.is_empty() && slice[node_len..].starts_with(delimeter) {
            fallback_position += delimeter.len();
        }
        fallback_position
    }
}

pub type MaybeNode<BranchNodes> = Box<dyn Fn(&str, Option<Context>) -> Option<BranchNodes>>;
pub type DefinitelyNode<BranchNodes> = Box<dyn Fn(&str) -> BranchNodes>;

pub trait FallbackNode {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<BranchNodes>
    where
        Self: Into<BranchNodes>;
}

pub trait Deserializer {
    fn deserialize_with_context(input: &str, ctx: Option<Context>) -> Option<Self>
    where
        Self: Sized;

    fn deserialize(input: &str) -> Option<Self>
    where
        Self: Sized,
    {
        Self::deserialize_with_context(input, None)
    }
}
