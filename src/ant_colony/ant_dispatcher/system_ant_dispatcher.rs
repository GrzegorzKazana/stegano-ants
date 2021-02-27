use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::graph::{AdjacencyListEntry, Graph};
use crate::ant_colony::pheromone::Pheromone;
use crate::common::utils::{compare_float, weighted_sample};

use super::AntDispatcher;

/// Picks edge in one of two modes which are selected with given probability (exploitation_rate).
///
/// Exploration:
/// each edge is attributed probability proportional to pheromone trail and inversly proportional to distance raised to bias power
///         p_{if}=\frac{\tau_{ij}}{d_{ij}^b}
///
/// Exploitation:
/// selects edge with maximum score which is evaluated in the same way as in exploration mode.
pub struct SystemAntDispatcher {
    exploitation_rate: f32,
    visibility_bias: f32,
}

impl SystemAntDispatcher {
    pub fn new(exploitation_rate: f32, visibility_bias: f32) -> Self {
        SystemAntDispatcher {
            exploitation_rate,
            visibility_bias,
        }
    }

    fn evalutate_edge(&self, pheromone: &Pheromone, edge: &AdjacencyListEntry) -> f32 {
        let visibility = 1.0 / edge.distance;
        let pheromone_level = pheromone.get_pheromone_for_edge(edge.key);

        visibility.powf(self.visibility_bias) * pheromone_level + stability_factor!()
    }

    fn try_expoit_best_edge(
        &self,
        possible_next_edges: &[AdjacencyListEntry],
        pheromone: &Pheromone,
        strategy_seed: f32,
    ) -> Option<AdjacencyListEntry> {
        if strategy_seed < self.exploitation_rate {
            return Option::None;
        }

        possible_next_edges
            .iter()
            .max_by(|edge_a, edge_b| {
                let value_a = self.evalutate_edge(pheromone, edge_a);
                let value_b = self.evalutate_edge(pheromone, edge_b);

                compare_float(&value_a, &value_b)
            })
            .map(|edge| edge.to_owned())
    }

    fn get_explored_edge(
        &self,
        possible_next_edges: &[AdjacencyListEntry],
        pheromone: &Pheromone,
        sample_seed: f32,
    ) -> Option<AdjacencyListEntry> {
        let node_likelihood = possible_next_edges
            .into_iter()
            .map(|edge| self.evalutate_edge(pheromone, edge))
            .collect::<Vec<_>>();

        weighted_sample(&possible_next_edges, &node_likelihood, sample_seed)
    }
}

impl AntDispatcher for SystemAntDispatcher {
    fn select_next_edge(
        &self,
        ant: &Ant,
        graph: &Graph,
        pheromone: &Pheromone,
        sample_seed: f32,
        strategy_seed: f32,
    ) -> Option<AdjacencyListEntry> {
        let possible_next_edges = self.get_possible_next_edges_for_ant(ant, graph);

        self.try_expoit_best_edge(&possible_next_edges, pheromone, strategy_seed)
            .or_else(|| self.get_explored_edge(&possible_next_edges, pheromone, sample_seed))
    }
}

impl Display for SystemAntDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ant dispatcher (System)\n\t\
            exploitation rate: {:>5.3}\n\t\
            visibility bias:   {:>5.3}",
            self.exploitation_rate, self.visibility_bias
        )
    }
}
