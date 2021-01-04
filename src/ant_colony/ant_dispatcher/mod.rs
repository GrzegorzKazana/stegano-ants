mod basic_ant_dispatcher;

use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::graph::{AdjacencyListEntry, Graph, NodeId};
use crate::ant_colony::pheromone::Pheromone;

pub use basic_ant_dispatcher::BasicAntDispatcher;

pub trait AntDispatcher: Display {
    fn place_ants_on_graph(&mut self, num_of_ants: u32, graph: &Graph) -> Vec<Ant> {
        let node_ids = graph.get_node_ids();

        (0..num_of_ants)
            .map(|_| Ant::new(self.select_random_node(&node_ids)))
            .collect()
    }

    fn select_random_node(&mut self, node_ids: &Vec<NodeId>) -> NodeId;

    fn select_next_edge<'a>(
        &mut self,
        ant: &Ant,
        graph: &'a Graph,
        pheromone: &Pheromone,
    ) -> &'a AdjacencyListEntry;
}
