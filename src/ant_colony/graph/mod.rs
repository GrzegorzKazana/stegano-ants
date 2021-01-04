mod _tests;
mod construct;

use std::collections::BTreeMap;
use std::fmt::Display;

pub type NodeId = u32;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub adjacency_list: Vec<AdjacencyListEntry>,
}

#[derive(Debug, PartialEq)]
pub struct AdjacencyListEntry {
    pub from: NodeId,
    pub to: NodeId,
    pub distance: f32,
}

#[derive(Debug, PartialEq)]
pub struct Graph {
    /*
     * using BTreeMap for stable iteration order
     * (which HashMap does not have)
     * TODO: compare performance against indexmap (https://github.com/bluss/indexmap)
     */
    nodes: BTreeMap<NodeId, Node>,
}

impl Graph {
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

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node_count = self.nodes.len();
        let edge_count = self.get_all_edges().len();

        write!(
            f,
            "Graph\n\t\
            nodes: {:>10}\n\t\
            edges: {:>10}",
            node_count, edge_count
        )
    }
}
