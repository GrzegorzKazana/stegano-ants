mod _tests;

use std::collections::HashMap;

use super::graph::NodeId;

pub type EdgeKey = u64;
pub type PheromoneLevel = f32;

pub struct Pheromone {
    initial_value: PheromoneLevel,
    values: HashMap<EdgeKey, PheromoneLevel>,
}

impl Pheromone {
    pub fn new(initial_value: PheromoneLevel) -> Self {
        Pheromone {
            initial_value,
            values: HashMap::new(),
        }
    }

    pub fn initialize_pheromone_for_edge(mut self, from: NodeId, to: NodeId) -> Self {
        let key = Pheromone::generate_edge_key(from, to);

        self.values.insert(key, self.initial_value);

        self
    }

    pub fn get_pheromone_for_edge(&self, from: NodeId, to: NodeId) -> PheromoneLevel {
        let key = Pheromone::generate_edge_key(from, to);

        self.values.get(&key).unwrap_or(&self.initial_value).clone()
    }

    pub fn increase_pheromone_value(mut self, from: NodeId, to: NodeId, increment: f32) -> Self {
        let key = Pheromone::generate_edge_key(from, to);

        if let Some(val) = self.values.get_mut(&key) {
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

    /*
     * generates deterministic an unique integer for pairs of integers
     * loosely based on Szudzik's pairing function http://szudzik.com/ElegantPairing.pdf
     * with the difference that input order does not matter
     */
    fn generate_edge_key(from: NodeId, to: NodeId) -> u64 {
        let smaller = std::cmp::min(from, to) as u64;
        let bigger = std::cmp::max(from, to) as u64;

        bigger * bigger + smaller
    }
}
