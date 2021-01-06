use rand::Rng;

use crate::ant_colony::ant_dispatcher::AntDispatcher;
use crate::ant_colony::pheromone_updater::PheromoneUpdater;

pub struct Config<U: PheromoneUpdater, D: AntDispatcher, R: Rng> {
    pub ant_count: usize,
    pub num_of_steps_per_cycle: usize,
    pub pheromone_updater: U,
    pub ant_dispatcher: D,
    pub rng: R,
}
