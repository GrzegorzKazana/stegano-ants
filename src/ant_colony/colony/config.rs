use std::fmt::Display;

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

impl<U: PheromoneUpdater, D: AntDispatcher, R: Rng> Display for Config<U, D, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Config:\n\t\
            ant count: {}\n\t\
            number of steps per cycle: {}\n\t\
            dispatcher: {}\n\t\
            updater: {}",
            self.ant_count,
            self.num_of_steps_per_cycle,
            self.ant_dispatcher,
            self.pheromone_updater
        )
    }
}
