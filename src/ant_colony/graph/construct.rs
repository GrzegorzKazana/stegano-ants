use super::{AdjacencyListEntry, Graph, Node, NodeId};
use std::collections::HashMap;

impl Graph {
    #[allow(dead_code)]
    pub fn from_node_vector(nodes_vec: Vec<Node>) -> Self {
        let nodes = nodes_vec
            .into_iter()
            .fold(HashMap::new(), |mut nodes, node| {
                nodes.insert(node.id, node);
                nodes
            });

        Graph { nodes }
    }

    pub fn from_neighbour_tuples(tuple_vec: Vec<(NodeId, NodeId, f32)>) -> Self {
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