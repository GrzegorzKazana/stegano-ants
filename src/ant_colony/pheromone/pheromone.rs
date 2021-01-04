use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;

use crate::ant_colony::graph::NodeId;

pub type EdgeKey = u64;
pub type PheromoneLevel = f32;

pub struct Pheromone {
    values: HashMap<EdgeKey, PheromoneLevel>,
}

impl Pheromone {
    pub fn new() -> Self {
        Pheromone {
            values: HashMap::new(),
        }
    }

    pub fn initialize_pheromone_for_edge(mut self, from: NodeId, to: NodeId, value: f32) -> Self {
        let key = Pheromone::generate_edge_key(from, to);

        self.values.insert(key, value);

        self
    }

    pub fn get_pheromone_for_edge(&self, from: NodeId, to: NodeId) -> PheromoneLevel {
        let key = Pheromone::generate_edge_key(from, to);

        self.values.get(&key).unwrap_or(&0.0).clone()
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
    pub fn generate_edge_key(from: NodeId, to: NodeId) -> u64 {
        let smaller = std::cmp::min(from, to) as u64;
        let bigger = std::cmp::max(from, to) as u64;

        bigger * bigger + smaller
    }

    /*
     * reverse function to generate_edge_key
     * result order is not respected smaller id first
     * do not use for performance ciritical sections
     */
    pub fn decode_edge_key(key: u64) -> (NodeId, NodeId) {
        let floor = (key as f64).sqrt().floor() as u64;

        ((key - floor * floor) as NodeId, floor as NodeId)
    }
}

impl Display for Pheromone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self
            .values
            .iter()
            .sorted_by(|(key_a, _), (key_b, _)| {
                Ord::cmp(
                    &Pheromone::decode_edge_key(**key_a),
                    &Pheromone::decode_edge_key(**key_b),
                )
            })
            .collect::<Vec<_>>()
            .chunks(8)
            .fold(String::new(), |text, chunks| {
                chunks
                    .iter()
                    .fold(text + "\n\t| ", |chunk_text, (key, value)| {
                        let (from, to) = Pheromone::decode_edge_key(**key);
                        format!("{}({:^5},{:^5}):{:>6.3} | ", chunk_text, from, to, value)
                    })
            });

        write!(f, "Pheromone {}", text)
    }
}
