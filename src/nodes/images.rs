use std::fmt::{Display, Formatter};

use serde::Serialize;

use super::Image;

/// Image Gallery node is a node that contains multiple Image nodes
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Images {
    pub nodes: Vec<Image>,
}

impl Images {
    pub fn new(nodes: Vec<Image>) -> Self {
        Self { nodes }
    }
}

impl Default for Images {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Display for Images {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for n in self.nodes.iter() {
            f.write_str(n.to_string().as_str())?;
        }
        Ok(())
    }
}
