use rand::Rng;

mod _tests;
mod config;
mod stepwise_parallel_colony;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::ant_dispatcher::AntDispatcher;
use crate::ant_colony::graph::{Graph, RouteCollection};
use crate::ant_colony::pheromone::Pheromone;
use crate::ant_colony::pheromone_updater::PheromoneUpdater;

pub use config::Config;
pub use stepwise_parallel_colony::StepwiseParallelColony;

pub trait Colony {
    fn execute_n_cycles(self, n_cycles: usize) -> Self;

    fn execute_cycle(self, cycle: usize) -> Self;

    fn get_pheromone(&self) -> &Pheromone;

    fn get_routes(&self) -> &RouteCollection;

    fn get_ants(&self) -> &[Ant];
}

pub trait ConfigurableColony<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> {
    fn new(config: Config<U, D, R>, graph: &'a Graph) -> Self;
}
