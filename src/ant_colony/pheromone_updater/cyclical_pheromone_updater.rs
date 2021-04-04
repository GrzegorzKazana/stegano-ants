use std::fmt::Display;

use crate::ant_colony::graph::{RouteBatchWithHoles, RouteCollection};
use crate::ant_colony::guided_configuration::WithGuidingConfig;
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::PheromoneUpdater;

/// After all ants finish whole cycle, decays the pheromone and updates pheromone levels
/// of specific edges by value inversly proportional to length of route they belong to.
///
/// If edge was taken by multiple ants it will be increased multiple times.
///
/// Does not perform updates after each step.
pub struct CyclicalPheromoneUpdater {
    initial_value: f32,
    evaporation_rate: f32,
    increment: f32,
}

impl CyclicalPheromoneUpdater {
    pub fn new(initial_value: f32, evaporation_rate: f32, increment: f32) -> Self {
        CyclicalPheromoneUpdater {
            initial_value,
            evaporation_rate,
            increment,
        }
    }
}

impl PheromoneUpdater for CyclicalPheromoneUpdater {
    fn get_initial_value(&self) -> PheromoneLevel {
        self.initial_value
    }

    fn on_after_step(&self, pheromone: Pheromone, _taken_edges: &RouteBatchWithHoles) -> Pheromone {
        pheromone
    }

    fn on_after_cycle(&self, pheromone: Pheromone, taken_routes: &RouteCollection) -> Pheromone {
        let decay = 1.0 - self.evaporation_rate;

        let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

        taken_routes.get_routes().iter().fold(
            decayed_pheromone,
            |route_updated_pheromone, taken_route| {
                let route_len = taken_route.get_distance();

                taken_route.get_edges().iter().fold(
                    route_updated_pheromone,
                    |edge_updated_route, taken_edge| {
                        let increment = self.increment / route_len;

                        edge_updated_route.increase_pheromone_value(taken_edge.key, increment)
                    },
                )
            },
        )
    }
}

impl WithGuidingConfig for CyclicalPheromoneUpdater {}

impl Display for CyclicalPheromoneUpdater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pheromone updater (Cyclical)\n\t\
            initial_value: {:>5}\n\t\
            evaporation:   {:>5.3}\n\t\
            increment:     {:>5.3}",
            self.initial_value, self.evaporation_rate, self.increment
        )
    }
}
