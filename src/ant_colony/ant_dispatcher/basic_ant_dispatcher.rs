use std::fmt::Display;

use crate::ant_colony::graph::AdjacencyListEntry;
use crate::ant_colony::guided_configuration::WithGuidingConfig;
use crate::ant_colony::pheromone::Pheromone;

use super::LikelihoodAntDispatcher;

pub struct BasicAntDispatcher;

/// Each edge is attributed probability simply proportional to
/// pheromone level and inversly to distance
///
/// p_{if}=\frac{\tau_{ij}}{d_{ij}}
impl LikelihoodAntDispatcher for BasicAntDispatcher {
    fn cacluclate_node_likelihoods(
        &self,
        possible_next_edges: &[AdjacencyListEntry],
        pheromone: &Pheromone,
    ) -> Vec<f32> {
        possible_next_edges
            .iter()
            .map(|edge| {
                let pheromone_level = pheromone.get_pheromone_for_edge(edge.key);

                edge.visibility * pheromone_level + stability_factor!()
            })
            .collect::<Vec<_>>()
    }
}

impl WithGuidingConfig for BasicAntDispatcher {}

impl Display for BasicAntDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ant dispatcher (Basic)",)
    }
}
