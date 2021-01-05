use super::AdjacencyListEntry;

pub type NodeId = u32;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub adjacency_list: Vec<AdjacencyListEntry>,
}
