use std::fmt::Display;

use crate::ant_colony::graph::{RouteBatch, RouteCollection};
use crate::ant_colony::pheromone::Pheromone;

use super::PheromoneUpdater;

pub struct ConstantPheromoneUpdater {
    initial_value: f32,
    evaporation_rate: f32,
}

impl ConstantPheromoneUpdater {
    pub fn new(initial_value: f32, evaporation_rate: f32) -> Self {
        ConstantPheromoneUpdater {
            initial_value,
            evaporation_rate,
        }
    }
}

impl Display for ConstantPheromoneUpdater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pheromone updater (Basic)\n\t\
            initial_value: {:>5}\n\t\
            evaporation:   {:>5.3}",
            self.initial_value, self.evaporation_rate
        )
    }
}

impl PheromoneUpdater for ConstantPheromoneUpdater {
    fn initialize(&self, init_pheromone: Pheromone, edges: &RouteBatch) -> Pheromone {
        edges.iter().fold(init_pheromone, |pheromone, edge| {
            pheromone.initialize_pheromone_for_edge(edge.key, self.initial_value)
        })
    }

    fn on_after_step(&self, pheromone: Pheromone, taken_edges: &RouteBatch) -> Pheromone {
        let decay = 1.0 - self.evaporation_rate;
        let increment = self.evaporation_rate * self.initial_value;

        let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

        taken_edges
            .iter()
            .fold(decayed_pheromone, |updated_pheromone, taken_edge| {
                updated_pheromone.increase_pheromone_value(taken_edge.key, increment)
            })
    }

    fn on_after_cycle(&self, pheromone: Pheromone, _taken_edges: &RouteCollection) -> Pheromone {
        pheromone
    }
}
