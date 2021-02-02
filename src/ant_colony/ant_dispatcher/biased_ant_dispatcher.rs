use std::fmt::Display;

use crate::ant_colony::graph::AdjacencyListEntry;
use crate::ant_colony::pheromone::Pheromone;

use super::LikelihoodAntDispatcher;

pub struct BiasedAntDispatcher {
    pheromone_bias: f32,
    visibility_bias: f32,
}

impl BiasedAntDispatcher {
    pub fn new(pheromone_bias: f32, visibility_bias: f32) -> Self {
        BiasedAntDispatcher {
            pheromone_bias,
            visibility_bias,
        }
    }
}

impl LikelihoodAntDispatcher for BiasedAntDispatcher {
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

                visibility.powf(self.visibility_bias) * pheromone_level.powf(self.pheromone_bias)
                    + stability_factor!()
            })
            .collect::<Vec<_>>()
    }
}

impl Display for BiasedAntDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ant dispatcher (Biased)\n\t\
            pheromone bias:  {:>5.3}\n\t\
            visibility bias: {:>5.3}",
            self.pheromone_bias, self.visibility_bias
        )
    }
}
