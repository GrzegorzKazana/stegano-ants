use rand::Rng;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::ant_colony::graph::{AdjacencyListEntry, Graph, NodeId};
use crate::ant_colony::pheromone::Pheromone;

use super::ant::Ant;

pub trait AntDispatcher {
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

pub struct BasicAntDispatcher<R: Rng> {
    random: R,
}

impl<R: Rng> BasicAntDispatcher<R> {
    pub fn new(random: R) -> Self {
        BasicAntDispatcher { random }
    }
}

impl<R: Rng> AntDispatcher for BasicAntDispatcher<R> {
    fn select_random_node(&mut self, node_ids: &Vec<NodeId>) -> NodeId {
        let idx = self.random.gen_range(0..node_ids.len());

        node_ids[idx]
    }

    fn select_next_edge<'a>(
        &mut self,
        ant: &Ant,
        graph: &'a Graph,
        pheromone: &Pheromone,
    ) -> &'a AdjacencyListEntry {
        let possible_next_edges: Vec<&AdjacencyListEntry> = graph
            .get_adjacent_edges(&ant.current_node)
            .iter()
            .filter(|edge| !ant.has_visited(&edge.to))
            .collect();

        // node_likelihood is not normalized - does not sum up to one
        // this is fine, since WeightedIndex takes care of it
        let node_likelihood = possible_next_edges.iter().map(|edge| {
            let visibility = 1.0 / edge.distance;
            let pheromone_level = pheromone.get_pheromone_for_edge(edge.key);

            visibility * pheromone_level + 1e-5
        });

        let dist = WeightedIndex::new(node_likelihood).unwrap();

        possible_next_edges[dist.sample(&mut self.random)]
    }
}
