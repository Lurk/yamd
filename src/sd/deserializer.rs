pub trait Branch<BranchNodes>
where
    BranchNodes: Node,
{
    fn new() -> Self;
    fn push<CanBeNode: Into<BranchNodes>>(&mut self, node: CanBeNode);
    fn from_vec(nodes: Vec<BranchNodes>) -> Self;
    fn get_maybe_nodes() -> Vec<MaybeNode<BranchNodes>>;
    fn get_fallback_node() -> Option<DefinitelyNode<BranchNodes>>;
    fn get_outer_token_length(&self) -> usize;

    fn parse_branch(input: &str) -> Option<Self>
    where
        Self: Sized + Deserializer + Node,
    {
        let mut branch = Self::new();
        let mut current_position = 0;
        let mut fallback_position = 0;
        let fallback_node = Self::get_fallback_node();
        let maybe_nodes = Self::get_maybe_nodes();
        while current_position < input.len() {
            let slice = &input[current_position..];
            current_position += 1;
            for parser in &maybe_nodes {
                if let Some(node) = parser(slice) {
                    if fallback_position != current_position - 1 {
                        match &fallback_node {
                            Some(fallback_node) => branch.push(fallback_node(
                                &input[fallback_position..current_position - 1],
                            )),
                            None => return None,
                        }
                    }
                    branch.push(node);
                    current_position = branch.len() - branch.get_outer_token_length();
                    fallback_position = current_position;
                }
            }
        }
        if fallback_position < input.len() {
            match fallback_node {
                Some(fallback_node) => branch.push(fallback_node(&input[fallback_position..])),
                None => return None,
            }
        }

        Some(branch)
    }
}

pub type MaybeNode<BranchNodes> = Box<dyn Fn(&str) -> Option<BranchNodes>>;
pub type DefinitelyNode<BranchNodes> = Box<dyn Fn(&str) -> BranchNodes>;

pub trait FallbackNode {
    fn fallback_node<BranchNodes>() -> DefinitelyNode<BranchNodes>
    where
        Self: Into<BranchNodes>;
}

pub trait Node {
    fn len(&self) -> usize;
    fn maybe_node<BranchNodes>() -> MaybeNode<BranchNodes>
    where
        Self: Sized + Deserializer + Into<BranchNodes>,
    {
        Box::new(|input| {
            if let Some(token) = Self::deserialize(input) {
                return Some(token.into());
            }
            None
        })
    }
}

pub trait Deserializer {
    fn deserialize(input: &str) -> Option<Self>
    where
        Self: Sized;
}
