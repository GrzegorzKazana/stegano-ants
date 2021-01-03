use super::graph::NodeId;

pub struct Ant {
    pub currentNode: NodeId,
    visited: Vec<NodeId>,
}

impl Ant {
    pub fn new(currentNode: NodeId) -> Self {
        Ant {
            currentNode,
            visited: vec![currentNode],
        }
    }

    pub fn move_to_node(mut self, next_node: NodeId) -> Self {
        self.visited.push(next_node);
        self
    }

    pub fn has_visited(&self, node_id: &NodeId) -> bool {
        self.visited.contains(node_id)
    }
}
