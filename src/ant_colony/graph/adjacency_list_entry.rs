use crate::common::utils::UniquePair;

use super::NodeId;

pub type EdgeKey = u64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AdjacencyListEntry {
    pub key: EdgeKey,
    pub from: NodeId,
    pub to: NodeId,
    pub distance: f32,
}

impl AdjacencyListEntry {
    pub fn new(from: NodeId, to: NodeId, distance: f32) -> Self {
        AdjacencyListEntry {
            key: UniquePair::generate_key(from, to),
            from,
            to,
            distance,
        }
    }
}
