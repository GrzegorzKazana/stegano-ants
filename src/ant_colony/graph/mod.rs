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

use crate::common::utils::compare_float;

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
            .map_or_else(Vec::new, |n| n.adjacency_list.to_owned())
    }

    pub fn get_all_edges(&self) -> Vec<AdjacencyListEntry> {
        self.edges_iter().collect()
    }

    pub fn get_edge(&self, key: EdgeKey) -> Option<AdjacencyListEntry> {
        self.edges_iter()
            .find(|edge| edge.key == key)
            .map(|edge| edge.to_owned())
    }

    pub fn get_edges(&self, keys: &[EdgeKey]) -> Vec<AdjacencyListEntry> {
        self.edges_iter()
            .filter(|edge| keys.contains(&edge.key))
            .collect()
    }

    pub fn get_node_ids(&self) -> Vec<NodeId> {
        self.nodes.keys().map(|k| k.clone()).collect()
    }

    pub fn get_amount_of_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_amount_of_edges(&self) -> usize {
        self.nodes
            .values()
            .map(|node| node.adjacency_list.len())
            .sum()
    }

    pub fn get_max_cycle_edges(&self) -> usize {
        self.get_amount_of_nodes() - 1
    }

    pub fn min_edge_length(&self) -> f32 {
        self.edges_lengths_iter()
            .min_by(compare_float)
            .unwrap_or_default()
    }

    pub fn max_edge_length(&self) -> f32 {
        self.edges_lengths_iter()
            .max_by(compare_float)
            .unwrap_or_default()
    }

    pub fn avg_edge_length(&self) -> f32 {
        let edges_total_length: f32 = self.edges_lengths_iter().sum();
        let edges_count = self.edges_lengths_iter().count() as f32;

        iif!(edges_count > 0.0, edges_total_length / edges_count, 0.0)
    }

    fn edges_iter(&self) -> impl Iterator<Item = AdjacencyListEntry> + '_ {
        self.nodes
            .values()
            .flat_map(|node| node.adjacency_list.to_owned())
    }

    fn edges_lengths_iter(&self) -> impl Iterator<Item = f32> + '_ {
        self.nodes
            .values()
            .flat_map(|node| node.adjacency_list.iter().map(|edge| edge.distance))
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node_count = self.get_amount_of_nodes();
        let edge_count = self.get_amount_of_edges();

        write!(
            f,
            "Graph\n\t\
            nodes: {:>10}\n\t\
            edges: {:>10}",
            node_count, edge_count
        )
    }
}
