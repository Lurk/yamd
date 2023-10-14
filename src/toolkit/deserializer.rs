use super::{context::Context, node::Node};

pub trait Branch<BranchNodes>
where
    BranchNodes: Node,
{
    fn push<CanBeNode: Into<BranchNodes>>(&mut self, node: CanBeNode);
    fn get_maybe_nodes() -> Vec<MaybeNode<BranchNodes>>;
    fn get_fallback_node() -> Option<DefinitelyNode<BranchNodes>>;
    fn get_outer_token_length(&self) -> usize;

    fn parse_branch(input: &str, mut branch: Self) -> Option<Self>
    where
        Self: Sized + Deserializer + Node,
    {
        let mut current_position = 0;
        let mut fallback_position = 0;
        let maybe_nodes = Self::get_maybe_nodes();
        while current_position < input.len() {
            let slice = &input[current_position..];
            current_position += slice.chars().next().unwrap().len_utf8();
            if maybe_nodes.is_empty() {
                current_position = branch.fallback(slice)?;
                fallback_position = current_position;
                continue;
            }
            for parser in &maybe_nodes {
                if let Some(node) = parser(slice, branch.context()) {
                    while fallback_position != current_position - 1 {
                        fallback_position =
                            branch.fallback(&input[fallback_position..current_position - 1])?;
                    }
                    branch.push(node);
                    current_position = branch.len() - branch.get_outer_token_length();
                    fallback_position = current_position;
                }
            }
        }
        while fallback_position < input.len() {
            fallback_position = branch.fallback(&input[fallback_position..])?;
        }

        Some(branch)
    }

    fn fallback(&mut self, slice: &str) -> Option<usize>
    where
        Self: Node,
    {
        self.push(Self::get_fallback_node().map(|f| f(slice))?);
        Some(self.len() - self.get_outer_token_length())
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
