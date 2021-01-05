use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::graph::{AdjacencyListEntry, Graph};
use crate::ant_colony::pheromone::Pheromone;
use crate::common::utils::weighted_sample;

use super::AntDispatcher;

pub struct BasicAntDispatcher;

impl Display for BasicAntDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ant dispatcher (Basic)",)
    }
}

impl AntDispatcher for BasicAntDispatcher {
    fn select_next_edge<'a>(
        &self,
        ant: &Ant,
        graph: &'a Graph,
        pheromone: &Pheromone,
        random_seed: f32,
    ) -> &'a AdjacencyListEntry {
        let possible_next_edges: Vec<&AdjacencyListEntry> = graph
            .get_adjacent_edges(&ant.current_node)
            .iter()
            .filter(|edge| !ant.has_visited(&edge.to))
            .collect();

        let node_likelihood = possible_next_edges
            .iter()
            .map(|edge| {
                let visibility = 1.0 / edge.distance;
                let pheromone_level = pheromone.get_pheromone_for_edge(edge.key);

                visibility * pheromone_level + 1e-5
            })
            .collect::<Vec<_>>();

        weighted_sample(&possible_next_edges, &node_likelihood, random_seed)
    }
}
