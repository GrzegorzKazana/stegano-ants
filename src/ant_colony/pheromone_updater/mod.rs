mod constant_pheromone_updater;

use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::graph::{AdjacencyListEntry, Graph};
use crate::ant_colony::pheromone::Pheromone;

pub use constant_pheromone_updater::ConstantPheromoneUpdater;

pub trait PheromoneUpdater: Display {
    fn initialize(&self, pheromone: Pheromone, edges: Vec<&AdjacencyListEntry>) -> Pheromone;

    fn on_after_step(
        &self,
        pheromone: Pheromone,
        taken_edges: Vec<&AdjacencyListEntry>,
    ) -> Pheromone;

    fn on_after_cycle(&self, pheromone: Pheromone, ants: &Vec<Ant>, graph: &Graph) -> Pheromone;
}
