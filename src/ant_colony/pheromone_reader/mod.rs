mod _tests;

use itertools::Itertools;

use crate::ant_colony::graph::{AdjacencyListEntry, EdgeKey, Graph};
use crate::ant_colony::pheromone::Pheromone;

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
        let pheromone_norm = pheromone.get_values_normalized();
        let mut pheromone_copy = pheromone_norm.iter().collect::<Vec<_>>();

        (0..n).fold(Vec::with_capacity(n), |mut acc, _| {
            let maybe_max_idx = pheromone_copy
                .iter()
                .position_max_by(|(_, value_a), (_, value_b)| value_a.partial_cmp(value_b).unwrap())
                .filter(|idx| *pheromone_copy[*idx].1 > std::f32::NEG_INFINITY);

            match maybe_max_idx {
                Option::Some(max_idx) => {
                    acc.push(pheromone_copy[max_idx].0.clone());
                    pheromone_copy[max_idx].1 = &std::f32::NEG_INFINITY;
                    acc
                }
                Option::None => acc,
            }
        })
    }
}
