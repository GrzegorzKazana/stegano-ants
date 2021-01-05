mod basic_ant_dispatcher;

use rand::{seq::IteratorRandom, Rng};
use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::graph::{AdjacencyListEntry, Graph};
use crate::ant_colony::pheromone::Pheromone;

pub use basic_ant_dispatcher::BasicAntDispatcher;

pub trait AntDispatcher: Display + Send + Sync {
    fn place_ants_on_graph<R: Rng>(
        &self,
        num_of_ants: usize,
        graph: &Graph,
        rng: &mut R,
    ) -> Vec<Ant> {
        let node_ids = graph.get_node_ids();

        (0..num_of_ants)
            .map(|_| node_ids.iter().choose(rng).unwrap().clone())
            .map(Ant::new)
            .collect()
    }

    fn select_next_edge<'a>(
        &self,
        ant: &Ant,
        graph: &'a Graph,
        pheromone: &Pheromone,
        random_seed: f32,
    ) -> &'a AdjacencyListEntry;
}
