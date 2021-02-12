mod _tests;
mod _union;
mod average_pheromone_updater;
mod constant_pheromone_updater;
mod cyclical_pheromone_updater;
mod system_pheromone_updater;

use std::fmt::Display;

use crate::ant_colony::graph::{RouteBatch, RouteBatchWithHoles, RouteCollection};
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

pub use _union::Updaters;
pub use average_pheromone_updater::AveragePheromoneUpdater;
pub use constant_pheromone_updater::ConstantPheromoneUpdater;
pub use cyclical_pheromone_updater::CyclicalPheromoneUpdater;
pub use system_pheromone_updater::SystemPheromoneUpdater;

pub trait PheromoneUpdater: Display {
    fn get_initial_value(&self) -> PheromoneLevel;

    fn initialize(&self, init_pheromone: Pheromone, edges: &RouteBatch) -> Pheromone {
        let initial_val = self.get_initial_value();

        edges.iter().fold(init_pheromone, |pheromone, edge| {
            pheromone.initialize_pheromone_for_edge(edge.key, initial_val)
        })
    }

    fn on_after_step(&self, pheromone: Pheromone, taken_edges: &RouteBatchWithHoles) -> Pheromone;

    fn on_after_cycle(&self, pheromone: Pheromone, taken_routes: &RouteCollection) -> Pheromone;
}
