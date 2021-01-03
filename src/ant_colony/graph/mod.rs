// mod _tests;

use std::collections::HashMap;

pub type NodeId = u32;

pub struct Node {
    id: NodeId,
    adjacency_list: Vec<AdjacencyListEntry>,
}

pub struct AdjacencyListEntry {
    adjacent: NodeId,
    distance: f32,
}

pub struct Graph {
    nodes: HashMap<NodeId, Node>,
}

impl Graph {
    pub fn from_vector(nodes_vec: Vec<Node>) -> Self {
        let nodes = nodes_vec
            .into_iter()
            .fold(HashMap::new(), |mut nodes, node| {
                nodes.insert(node.id, node);
                nodes
            });

        Graph { nodes }
    }

    pub fn get_adjacent_edges(&self, node: &Node) -> &[AdjacencyListEntry] {
        self.nodes.get(&node.id).map_or(&[], |n| &n.adjacency_list)
    }
}
