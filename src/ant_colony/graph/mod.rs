mod _tests;

use std::collections::HashMap;

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

    pub fn from_tuples(tuple_vec: Vec<(NodeId, NodeId, f32)>) -> Self {
        let mut node_ids: Vec<_> = tuple_vec
            .iter()
            .flat_map(|(from, to, _distance)| vec![from.clone(), to.clone()])
            .collect();

        node_ids.sort();
        node_ids.dedup();

        let nodes = node_ids
            .into_iter()
            .map(|node_id| Node {
                id: node_id,
                adjacency_list: Graph::parse_adjacency_list_from_tuple(node_id, &tuple_vec),
            })
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

    fn parse_adjacency_list_from_tuple(
        node_id: NodeId,
        tuple_vec: &Vec<(NodeId, NodeId, f32)>,
    ) -> Vec<AdjacencyListEntry> {
        tuple_vec
            .iter()
            .filter_map(|(from, to, distance)| {
                if node_id == *from {
                    Option::Some(AdjacencyListEntry {
                        from: from.clone(),
                        to: to.clone(),
                        distance: distance.clone(),
                    })
                } else if node_id == *to {
                    Option::Some(AdjacencyListEntry {
                        from: to.clone(),
                        to: from.clone(),
                        distance: distance.clone(),
                    })
                } else {
                    Option::None
                }
            })
            .collect()
    }
}
