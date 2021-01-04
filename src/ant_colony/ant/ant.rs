use std::fmt::Display;

use crate::ant_colony::graph::NodeId;

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

impl Display for Ant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_length = self.visited.len();
        let exceeds_display_length = path_length > 15;
        let ellispis = if exceeds_display_length { "..." } else { "" };

        let vector_to_display = if exceeds_display_length {
            &self.visited[0..15]
        } else {
            &self.visited[..]
        };

        write!(
            f,
            "ant: {:>5} path length; {:?}{}",
            path_length, vector_to_display, ellispis
        )
    }
}
