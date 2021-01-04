mod _tests;

use std::collections::BTreeSet;
use std::fmt::Display;

use crate::ant_colony::graph::NodeId;

#[derive(Debug)]
pub struct Ant {
    pub current_node: NodeId,
    visited: BTreeSet<NodeId>,
}

impl Ant {
    pub fn new(current_node: NodeId) -> Self {
        let mut visited = BTreeSet::new();

        visited.insert(current_node);

        Ant {
            current_node,
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
}

impl Display for Ant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.visited.iter().collect::<Vec<_>>();
        let path_length = path.len();
        let exceeds_display_length = path_length > 15;
        let ellispis = if exceeds_display_length { "..." } else { "" };

        let vector_to_display = if exceeds_display_length {
            &path[0..15]
        } else {
            &path[..]
        };

        write!(
            f,
            "ant: {:>5} path length; {:?}{}",
            path_length, vector_to_display, ellispis
        )
    }
}
