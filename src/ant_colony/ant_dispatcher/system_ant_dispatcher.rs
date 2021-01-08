use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::graph::{AdjacencyListEntry, Graph};
use crate::ant_colony::pheromone::Pheromone;
use crate::common::utils::weighted_sample;

use super::AntDispatcher;

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

    fn try_expoit_best_edge<'a>(
        &self,
        possible_next_edges: &[&'a AdjacencyListEntry],
        pheromone: &Pheromone,
        strategy_seed: f32,
    ) -> Option<&'a AdjacencyListEntry> {
        if strategy_seed < self.exploitation_rate {
            return Option::None;
        }

        possible_next_edges
            .iter()
            .max_by(|edge_a, edge_b| {
                let pheromone_a = pheromone.get_pheromone_for_edge(edge_a.key);
                let pheromone_b = pheromone.get_pheromone_for_edge(edge_b.key);
                let visibility_a = 1.0 / edge_a.distance;
                let visibility_b = 1.0 / edge_b.distance;
                let value_a = pheromone_a * visibility_a;
                let value_b = pheromone_b * visibility_b;

                value_a.partial_cmp(&value_b).unwrap()
            })
            .map(|edge| *edge)
    }

    fn get_explored_edge<'a>(
        &self,
        possible_next_edges: &[&'a AdjacencyListEntry],
        pheromone: &Pheromone,
        sample_seed: f32,
    ) -> &'a AdjacencyListEntry {
        let node_likelihood = possible_next_edges
            .into_iter()
            .map(|edge| {
                let visibility = 1.0 / edge.distance;
                let pheromone_level = pheromone.get_pheromone_for_edge(edge.key);

                visibility.powf(self.visibility_bias) * pheromone_level + 1e-5
            })
            .collect::<Vec<_>>();

        weighted_sample(&possible_next_edges, &node_likelihood, sample_seed)
    }
}

impl AntDispatcher for SystemAntDispatcher {
    fn select_next_edge<'a>(
        &self,
        ant: &Ant,
        graph: &'a Graph,
        pheromone: &Pheromone,
        sample_seed: f32,
        strategy_seed: f32,
    ) -> &'a AdjacencyListEntry {
        let possible_next_edges: Vec<&AdjacencyListEntry> =
            self.get_possible_next_edges_for_ant(ant, graph);

        self.try_expoit_best_edge(&possible_next_edges, pheromone, strategy_seed)
            .unwrap_or_else(|| self.get_explored_edge(&possible_next_edges, pheromone, sample_seed))
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