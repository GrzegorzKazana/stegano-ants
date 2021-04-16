mod _tests;
mod _union;
mod basic_ant_dispatcher;
mod biased_ant_dispatcher;
mod colony_ant_dispatcher;
mod likelihood_dispatcher;

use rand::{seq::IteratorRandom, Rng};
use std::fmt::Display;
use std::str::FromStr;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::graph::{AdjacencyListEntry, Graph};
use crate::ant_colony::guiding_config::WithGuidingConfig;
use crate::ant_colony::pheromone::Pheromone;

pub use _union::Dispatchers;
pub use basic_ant_dispatcher::BasicAntDispatcher;
pub use biased_ant_dispatcher::BiasedAntDispatcher;
pub use colony_ant_dispatcher::ColonyAntDispatcher;
pub use likelihood_dispatcher::LikelihoodAntDispatcher;

pub trait AntDispatcher: WithGuidingConfig + Display + Send + Sync + Sized + FromStr {
    fn place_ants_on_graph<R: Rng>(
        &self,
        num_of_ants: usize,
        graph: &Graph,
        rng: &mut R,
    ) -> Vec<Ant> {
        let node_ids = graph.get_node_ids();

        (0..num_of_ants)
            .filter_map(|_| node_ids.iter().choose(rng).cloned())
            .map(Ant::new)
            .collect()
    }

    fn select_next_edge(
        &self,
        ant: &Ant,
        graph: &Graph,
        pheromone: &Pheromone,
        sample_seed: f32,
        strategy_seed: f32,
    ) -> Option<AdjacencyListEntry>;

    #[cfg_attr(feature = "profiler", flame)]
    fn get_possible_next_edges_for_ant(&self, ant: &Ant, graph: &Graph) -> Vec<AdjacencyListEntry> {
        let adjacent_edges = graph.get_adjacent_edges(&ant.current_node);

        let possible_next_edges = adjacent_edges
            .iter()
            .filter(|edge| !ant.has_visited(&edge.to))
            .map(|edge| edge.to_owned())
            .collect::<Vec<_>>();

        if possible_next_edges.len() > 0 {
            return possible_next_edges;
        }

        // in case of tsp-like tasks, we want to close the cycle
        // this means, that if there is no nodes that have not been visited
        // it might be because ant has travelled all possible nodes,
        // here, we allow the ant to go back to the initial node
        let edge_leading_to_inital_node = adjacent_edges
            .into_iter()
            .find(|edge| edge.to == ant.inital_node);

        edge_leading_to_inital_node.map_or(possible_next_edges, |edge| vec![edge])
    }
}

#[derive(Debug, Clone)]
pub enum DispatcherStringConfig {
    Basic(String),
    Biased(String),
    Colony(String),
}

impl FromStr for DispatcherStringConfig {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config_iter = s.split(":");
        let name = config_iter.next().unwrap_or_default();
        let opts = config_iter.next().map(String::from).unwrap_or_default();

        match name {
            "basic" => Some(Self::Basic(opts)),
            "biased" => Some(Self::Biased(opts)),
            "colony" => Some(Self::Colony(opts)),
            _ => None,
        }
        .ok_or("Failed to parse ant dispatcher type")
    }
}

impl ToString for DispatcherStringConfig {
    fn to_string(&self) -> String {
        match self {
            Self::Basic(opts) => format!("basic:{}", opts),
            Self::Biased(opts) => format!("biased:{}", opts),
            Self::Colony(opts) => format!("colony:{}", opts),
        }
    }
}
