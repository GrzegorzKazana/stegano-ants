mod _tests;

use super::graph::NodeId;

pub struct Pheromone {}

impl Pheromone {
    pub fn new() -> Self {
        Pheromone {}
    }

    pub fn get_pheromone_for_edge(from: NodeId, to: NodeId) -> f32 {
        0.0
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
