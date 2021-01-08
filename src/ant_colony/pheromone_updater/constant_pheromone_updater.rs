use std::fmt::Display;

use crate::ant_colony::graph::{RouteBatch, RouteCollection};
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::PheromoneUpdater;

pub struct ConstantPheromoneUpdater {
    initial_value: f32,
    evaporation_rate: f32,
    increment: f32,
}

impl ConstantPheromoneUpdater {
    pub fn new(initial_value: f32, evaporation_rate: f32, increment: f32) -> Self {
        ConstantPheromoneUpdater {
            initial_value,
            evaporation_rate,
            increment,
        }
    }
}

impl PheromoneUpdater for ConstantPheromoneUpdater {
    fn get_initial_value(&self) -> PheromoneLevel {
        self.initial_value
    }

    fn on_after_step(&self, pheromone: Pheromone, taken_edges: &RouteBatch) -> Pheromone {
        let decay = 1.0 - self.evaporation_rate;

        let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

        taken_edges
            .iter()
            .fold(decayed_pheromone, |updated_pheromone, taken_edge| {
                updated_pheromone.increase_pheromone_value(taken_edge.key, self.increment)
            })
    }

    fn on_after_cycle(&self, pheromone: Pheromone, _taken_routes: &RouteCollection) -> Pheromone {
        pheromone
    }
}

impl Display for ConstantPheromoneUpdater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pheromone updater (Constant)\n\t\
            initial_value: {:>5}\n\t\
            evaporation:   {:>5.3}\n\t\
            increment:     {:>5.3}",
            self.initial_value, self.evaporation_rate, self.increment
        )
    }
}
