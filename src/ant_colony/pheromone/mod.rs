mod _tests;

use std::collections::HashMap;
use std::fmt::Display;

use crate::ant_colony::graph::EdgeKey;
use crate::common::utils::compare_float;

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

    pub fn from_values(values: HashMap<EdgeKey, PheromoneLevel>) -> Self {
        Pheromone { values }
    }

    pub fn initialize_pheromone_for_edge(mut self, edge_key: EdgeKey, value: f32) -> Self {
        self.values.insert(edge_key, value);

        self
    }

    pub fn get_pheromone_for_edge(&self, edge_key: EdgeKey) -> PheromoneLevel {
        let value = self.values.get(&edge_key);

        debug_assert_ne!(
            value,
            Option::None,
            "Failed to find pheromone value for edge"
        );

        value.unwrap_or(&0.0).clone()
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

    pub fn get_values(&self) -> &HashMap<EdgeKey, PheromoneLevel> {
        &self.values
    }

    /// Each pheromone trail is scaled to [0.0, 1.0)
    /// where 1.0 is maximum value
    pub fn get_values_normalized(&self) -> HashMap<EdgeKey, PheromoneLevel> {
        let max: f32 = *self.values.values().max_by(compare_float).unwrap_or(&1.0);

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

    pub fn normalize(&self) -> Self {
        Pheromone {
            values: self.get_values_normalized(),
        }
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

        write!(
            f,
            "Pheromone\n\t\
            edges with trail >= 0.001: {} / {} ({:>4.2}%)",
            non_zero_edges.len(),
            self.values.len(),
            100.0 * non_empty_edge_ratio,
        )
    }
}
