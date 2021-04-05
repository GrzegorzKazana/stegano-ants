use std::fmt::Display;

use crate::ant_colony::graph::AdjacencyListEntry;
use crate::ant_colony::guiding_config::WithGuidingConfig;
use crate::ant_colony::pheromone::Pheromone;

use super::LikelihoodAntDispatcher;

/// Each edge is attributed probability proportional to
/// pheromone level and inversly to distance.
/// Both factors are raised to given powers to control each importance.
///
/// p_{if}=\frac{\tau_{ij}^a}{d_{ij}^b}
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
                let pheromone_level = pheromone.get_pheromone_for_edge(edge.key);

                edge.visibility.powf(self.visibility_bias)
                    * pheromone_level.powf(self.pheromone_bias)
                    + stability_factor!()
            })
            .collect::<Vec<_>>()
    }
}

impl WithGuidingConfig for BiasedAntDispatcher {}

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
