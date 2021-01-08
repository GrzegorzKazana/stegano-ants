mod _tests;

use crate::ant_colony::graph::{AdjacencyListEntry, EdgeKey, Graph};
use crate::ant_colony::pheromone::Pheromone;

use crate::common::utils::select_top_n_items;

pub struct PheromoneReader;

impl PheromoneReader {
    pub fn count_edges_with_pheromone_above<'a>(pheromone: &Pheromone, level: f32) -> usize {
        pheromone
            .get_values_normalized()
            .iter()
            .filter(|(_, value)| **value > level)
            .count()
    }

    pub fn get_edges_with_pheromone_above<'a>(
        pheromone: &Pheromone,
        graph: &'a Graph,
        level: f32,
    ) -> Vec<&'a AdjacencyListEntry> {
        let keys = PheromoneReader::get_edge_keys_with_pheromone_above(pheromone, level);

        graph.get_edges(&keys)
    }

    pub fn get_top_n_edges<'a>(
        pheromone: &Pheromone,
        graph: &'a Graph,
        n: usize,
    ) -> Vec<&'a AdjacencyListEntry> {
        let keys = PheromoneReader::get_top_n_edge_keys(pheromone, n);

        graph.get_edges(&keys)
    }

    fn get_edge_keys_with_pheromone_above<'a>(pheromone: &Pheromone, level: f32) -> Vec<EdgeKey> {
        pheromone
            .get_values_normalized()
            .iter()
            .filter_map(|(key, value)| iif!(*value > level, Option::Some(*key), Option::None))
            .collect::<Vec<_>>()
    }

    fn get_top_n_edge_keys<'a>(pheromone: &Pheromone, n: usize) -> Vec<EdgeKey> {
        let pheromone_vals = pheromone.get_values_normalized();
        let pheromone_norm = pheromone_vals.iter().collect::<Vec<_>>();

        select_top_n_items(&pheromone_norm, n, |(_, value)| value)
            .iter()
            .map(|(key, _)| **key)
            .collect()
    }
}
