use rand::{distributions::Uniform, Rng};
use std::collections::BTreeMap;

use super::{AdjacencyListEntry, Graph, Node, NodeId};

impl Graph {
    #[allow(dead_code)]
    pub fn from_node_vector(nodes_vec: Vec<Node>) -> Self {
        let nodes = nodes_vec
            .into_iter()
            .fold(BTreeMap::new(), |mut nodes, node| {
                nodes.insert(node.id, node);
                nodes
            });

        Graph { nodes }
    }

    #[allow(dead_code)]
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
            .fold(BTreeMap::new(), |mut nodes, node| {
                nodes.insert(node.id, node);
                nodes
            });

        Graph { nodes }
    }

    #[allow(dead_code)]
    pub fn random_tsp_graph<R: Rng>(rng: &mut R, nodes: u32) -> Self {
        let distances = rng.sample_iter(Uniform::from(0.1..9.9));

        let tuples = (0..nodes - 1)
            .flat_map(|from| (from + 1..nodes).map(move |to| (from, to)))
            .zip(distances)
            .map(|((from, to), distance)| (from, to, distance))
            .collect();

        Graph::from_neighbour_tuples(tuples)
    }

    fn parse_adjacency_list_from_tuple(
        node_id: NodeId,
        tuple_vec: &Vec<(NodeId, NodeId, f32)>,
    ) -> Vec<AdjacencyListEntry> {
        tuple_vec
            .iter()
            .filter_map(|(from, to, distance)| {
                if node_id == *from {
                    Option::Some(AdjacencyListEntry::new(
                        from.clone(),
                        to.clone(),
                        distance.clone(),
                    ))
                } else if node_id == *to {
                    Option::Some(AdjacencyListEntry::new(
                        to.clone(),
                        from.clone(),
                        distance.clone(),
                    ))
                } else {
                    Option::None
                }
            })
            .collect()
    }
}
