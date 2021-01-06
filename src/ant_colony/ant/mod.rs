mod _tests;

use std::collections::HashSet;
use std::fmt::Display;

use crate::ant_colony::graph::NodeId;

#[derive(Debug)]
pub struct Ant {
    pub inital_node: NodeId,
    pub current_node: NodeId,
    visited: HashSet<NodeId>,
}

impl Ant {
    pub fn new(inital_node: NodeId) -> Self {
        let mut visited = HashSet::new();

        visited.insert(inital_node);

        Ant {
            inital_node,
            current_node: inital_node,
            visited,
        }
    }

    pub fn move_to_node(mut self, next_node: NodeId) -> Self {
        self.visited.insert(next_node);

        Ant {
            current_node: next_node,
            ..self
        }
    }

    pub fn has_visited(&self, node_id: &NodeId) -> bool {
        self.visited.contains(node_id)
    }

    pub fn get_visited(&self) -> HashSet<NodeId> {
        self.visited.clone()
    }
}

impl Display for Ant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.visited.iter().collect::<Vec<_>>();
        let path_length = path.len();
        let exceeds_display_length = path_length > 15;
        let ellispis = iif!(exceeds_display_length, "...", "");

        let vector_to_display = iif!(exceeds_display_length, &path[0..15], &path[..]);

        write!(
            f,
            "ant: {:>5} path length; {:?}{}",
            path_length, vector_to_display, ellispis
        )
    }
}
