// mod _tests;

use std::collections::HashMap;

pub type NodeId = u32;

pub struct Node {
    pub id: NodeId,
    pub adjacency_list: Vec<AdjacencyListEntry>,
}

pub struct AdjacencyListEntry {
    pub from: NodeId,
    pub to: NodeId,
    pub distance: f32,
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

    pub fn get_adjacent_edges(&self, node_id: &NodeId) -> &[AdjacencyListEntry] {
        self.nodes.get(node_id).map_or(&[], |n| &n.adjacency_list)
    }

    pub fn get_all_edges(&self) -> Vec<&AdjacencyListEntry> {
        self.nodes
            .values()
            .flat_map(|node| &node.adjacency_list)
            .collect()
    }

    pub fn get_node_ids(&self) -> Vec<NodeId> {
        self.nodes.keys().map(|k| k.clone()).collect()
    }
}
