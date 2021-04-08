use crate::common::utils::UniquePair;

use super::NodeId;

pub type EdgeKey = u64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AdjacencyListEntry {
    pub key: EdgeKey,
    pub from: NodeId,
    pub to: NodeId,
    pub distance: f32,
    pub visibility: f32,
}

impl AdjacencyListEntry {
    pub fn new(from: NodeId, to: NodeId, distance: f32) -> Self {
        AdjacencyListEntry {
            key: Self::get_key(from, to),
            from,
            to,
            distance,
            visibility: 1.0 / (distance + stability_factor!()),
        }
    }

    pub fn get_key(from: NodeId, to: NodeId) -> EdgeKey {
        UniquePair::generate_key(from, to)
    }
}
