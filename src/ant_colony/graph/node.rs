use super::AdjacencyListEntry;

pub type NodeId = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: NodeId,
    pub adjacency_list: Vec<AdjacencyListEntry>,
}
