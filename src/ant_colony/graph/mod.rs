mod _tests;
mod adjacency_list_entry;
mod construct;
mod node;
mod route;
mod route_batch;
mod route_collection;

use std::collections::BTreeMap;
use std::fmt::Display;

pub use _tests::{mock_graph_tuple, mock_graph_vector};
pub use adjacency_list_entry::{AdjacencyListEntry, EdgeKey};
pub use node::{Node, NodeId};
pub use route::Route;
pub use route_batch::{RouteBatch, RouteBatchWithHoles};
pub use route_collection::RouteCollection;

#[derive(Debug, PartialEq, Clone)]
pub struct Graph {
    /// using BTreeMap instead of HashMap for stable iteration order
    /// TODO: compare performance against indexmap (https://github.com/bluss/indexmap)
    nodes: BTreeMap<NodeId, Node>,
}

impl Graph {
    #[cfg_attr(feature = "profiler", flame)]
    pub fn get_adjacent_edges(&self, node_id: &NodeId) -> Vec<AdjacencyListEntry> {
        self.nodes
            .get(node_id)
            .map_or_else(|| vec![], |n| n.adjacency_list.to_owned())
    }

    pub fn get_all_edges(&self) -> Vec<AdjacencyListEntry> {
        self.edges_iterator().collect()
    }

    pub fn get_edge(&self, key: EdgeKey) -> Option<AdjacencyListEntry> {
        self.edges_iterator()
            .find(|edge| edge.key == key)
            .map(|edge| edge.to_owned())
    }

    pub fn get_edges(&self, keys: &[EdgeKey]) -> Vec<AdjacencyListEntry> {
        self.edges_iterator()
            .filter(|edge| keys.contains(&edge.key))
            .collect()
    }

    pub fn get_node_ids(&self) -> Vec<NodeId> {
        self.nodes.keys().map(|k| k.clone()).collect()
    }

    pub fn get_amount_of_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_max_cycle_edges(&self) -> usize {
        self.get_amount_of_nodes() - 1
    }

    fn edges_iterator(&self) -> impl Iterator<Item = AdjacencyListEntry> + '_ {
        self.nodes
            .values()
            .flat_map(|node| node.adjacency_list.to_owned())
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node_count = self.get_amount_of_nodes();
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
