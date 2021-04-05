use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::graph::{AdjacencyListEntry, Graph};
use crate::ant_colony::guiding_config::WithGuidingConfig;
use crate::ant_colony::pheromone::Pheromone;
use crate::common::utils::weighted_sample;

use super::AntDispatcher;

/// Trait common for all dispatchers that determine
/// probability of transition to each of edges.
/// Probabilities do not need to sum up to 1 (they are normalized anyway).
///
/// Dispatcher implementing the trait are only required to calculate a vector
/// of probabilities coresponding to given edges.
pub trait LikelihoodAntDispatcher: Display + Send + Sync {
    fn cacluclate_node_likelihoods(
        &self,
        possible_next_edges: &[AdjacencyListEntry],
        pheromone: &Pheromone,
    ) -> Vec<f32>;
}

impl<D: LikelihoodAntDispatcher + WithGuidingConfig> AntDispatcher for D {
    #[cfg_attr(feature = "profiler", flame)]
    fn select_next_edge(
        &self,
        ant: &Ant,
        graph: &Graph,
        pheromone: &Pheromone,
        sample_seed: f32,
        _strategy_seed: f32,
    ) -> Option<AdjacencyListEntry> {
        let possible_next_edges = self.get_possible_next_edges_for_ant(ant, graph);

        let node_likelihood = self.cacluclate_node_likelihoods(&possible_next_edges, pheromone);

        weighted_sample(&possible_next_edges, &node_likelihood, sample_seed)
    }
}
