mod constant_pheromone_updater;

use std::fmt::Display;

use crate::ant_colony::graph::{RouteBatch, RouteCollection};
use crate::ant_colony::pheromone::Pheromone;

pub use constant_pheromone_updater::ConstantPheromoneUpdater;

pub trait PheromoneUpdater: Display {
    fn initialize(&self, pheromone: Pheromone, edges: &RouteBatch) -> Pheromone;

    fn on_after_step(&self, pheromone: Pheromone, taken_edges: &RouteBatch) -> Pheromone;

    fn on_after_cycle(&self, pheromone: Pheromone, taken_edges: &RouteCollection) -> Pheromone;
}
