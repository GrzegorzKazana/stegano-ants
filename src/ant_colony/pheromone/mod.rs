mod _tests;

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;

use crate::ant_colony::graph::EdgeKey;
use crate::common::utils::UniquePair;

pub type PheromoneLevel = f32;

#[derive(Debug, PartialEq)]
pub struct Pheromone {
    values: HashMap<EdgeKey, PheromoneLevel>,
}

impl Pheromone {
    pub fn new() -> Self {
        Pheromone {
            values: HashMap::new(),
        }
    }

    pub fn initialize_pheromone_for_edge(mut self, edge_key: EdgeKey, value: f32) -> Self {
        self.values.insert(edge_key, value);

        self
    }

    pub fn get_pheromone_for_edge(&self, edge_key: EdgeKey) -> PheromoneLevel {
        self.values.get(&edge_key).unwrap_or(&0.0).clone()
    }

    pub fn increase_pheromone_value(mut self, edge_key: EdgeKey, increment: f32) -> Self {
        if let Some(val) = self.values.get_mut(&edge_key) {
            *val += increment;
        }

        self
    }

    pub fn scale_all_pheromone_values(mut self, scaler: f32) -> Self {
        for val in self.values.values_mut() {
            *val *= scaler;
        }

        self
    }

    fn get_values(&self) -> &HashMap<EdgeKey, PheromoneLevel> {
        &self.values
    }

    /// Each pheromone trail is scaled to [0.0, 1.0)
    /// where 1.0 is maximum value
    pub fn get_values_normalized(&self) -> HashMap<EdgeKey, PheromoneLevel> {
        let max: f32 = *self
            .values
            .values()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&1.0);

        self.values
            .iter()
            .map(|(key, val)| (key.clone(), val / max))
            .collect()
    }

    /// Each pheromone trail is scaled in such a way, that sum of all is 1.0
    pub fn get_values_normalized_to_sum(&self) -> HashMap<EdgeKey, PheromoneLevel> {
        let sum: f32 = self.values.values().sum();

        self.values
            .iter()
            .map(|(key, val)| (key.clone(), val / sum))
            .collect()
    }
}

impl Display for Pheromone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let non_zero_edges = self
            .values
            .iter()
            .filter(|(_, value)| **value >= 1e-3)
            .collect::<Vec<_>>();

        let non_empty_edge_ratio = non_zero_edges.len() as f32 / self.values.len() as f32;

        let text = non_zero_edges
            .iter()
            .sorted_by(|(key_a, _), (key_b, _)| {
                Ord::cmp(
                    &UniquePair::decode_key(**key_a),
                    &UniquePair::decode_key(**key_b),
                )
            })
            .collect::<Vec<_>>()
            .chunks(8)
            .fold(String::new(), |text, chunks| {
                chunks
                    .iter()
                    .fold(text + "\n\t| ", |chunk_text, (key, value)| {
                        let (from, to) = UniquePair::decode_key(**key);
                        format!("{}({:^5},{:^5}):{:>6.3} | ", chunk_text, from, to, value)
                    })
            });

        write!(
            f,
            "Pheromone\n\t\
            edges with trail >= 0.001: {} / {} ({:>4.2}%) {}",
            non_zero_edges.len(),
            self.values.len(),
            100.0 * non_empty_edge_ratio,
            text,
        )
    }
}
