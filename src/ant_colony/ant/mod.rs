mod _tests;

use super::graph::NodeId;

pub struct Ant {
    pub current_node: NodeId,
    visited: Vec<NodeId>,
}

impl Ant {
    pub fn new(current_node: NodeId) -> Self {
        Ant {
            current_node,
            visited: vec![current_node],
        }
    }

    pub fn move_to_node(mut self, next_node: NodeId) -> Self {
        self.visited.push(next_node);

        Ant {
            current_node: next_node,
            ..self
        }
    }

    pub fn has_visited(&self, node_id: &NodeId) -> bool {
        self.visited.contains(node_id)
    }
}
