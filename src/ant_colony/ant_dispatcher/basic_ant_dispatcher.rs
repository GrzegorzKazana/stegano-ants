use std::fmt::Display;

use crate::ant_colony::graph::AdjacencyListEntry;
use crate::ant_colony::pheromone::Pheromone;

use super::LikelihoodAntDispatcher;

pub struct BasicAntDispatcher;

impl LikelihoodAntDispatcher for BasicAntDispatcher {
    fn cacluclate_node_likelihoods(
        &self,
        possible_next_edges: &[AdjacencyListEntry],
        pheromone: &Pheromone,
    ) -> Vec<f32> {
        possible_next_edges
            .iter()
            .map(|edge| {
                let visibility = 1.0 / edge.distance;
                let pheromone_level = pheromone.get_pheromone_for_edge(edge.key);

                visibility * pheromone_level + 1e-5
            })
            .collect::<Vec<_>>()
    }
}

impl Display for BasicAntDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ant dispatcher (Basic)",)
    }
}
