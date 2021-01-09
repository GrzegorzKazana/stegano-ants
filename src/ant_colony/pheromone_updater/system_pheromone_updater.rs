use std::fmt::Display;

use crate::ant_colony::graph::{RouteBatchWithHoles, RouteCollection};
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::PheromoneUpdater;

pub struct SystemPheromoneUpdater {
    initial_value: f32,
    evaporation_rate: f32,
}

impl SystemPheromoneUpdater {
    pub fn new(initial_value: f32, evaporation_rate: f32) -> Self {
        SystemPheromoneUpdater {
            initial_value,
            evaporation_rate,
        }
    }
}

impl PheromoneUpdater for SystemPheromoneUpdater {
    fn get_initial_value(&self) -> PheromoneLevel {
        self.initial_value
    }

    fn on_after_step(&self, pheromone: Pheromone, taken_edges: &RouteBatchWithHoles) -> Pheromone {
        let decay = 1.0 - self.evaporation_rate;
        let increment = self.evaporation_rate * self.initial_value;

        let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

        taken_edges.iter().filter_map(|a| a.as_ref()).fold(
            decayed_pheromone,
            |updated_pheromone, taken_edge| {
                updated_pheromone.increase_pheromone_value(taken_edge.key, increment)
            },
        )
    }

    fn on_after_cycle(&self, pheromone: Pheromone, taken_routes: &RouteCollection) -> Pheromone {
        let decay = 1.0 - self.evaporation_rate;

        let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

        match taken_routes.get_shortest_route() {
            Option::None => decayed_pheromone,
            Option::Some(route) => {
                let route_len = route.get_distance();

                route.get_edges().iter().fold(
                    decayed_pheromone,
                    |edge_updated_route, taken_edge| {
                        let increment = self.evaporation_rate * self.initial_value / route_len;

                        edge_updated_route.increase_pheromone_value(taken_edge.key, increment)
                    },
                )
            }
        }
    }
}

impl Display for SystemPheromoneUpdater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pheromone updater (System)\n\t\
            initial_value: {:>5}\n\t\
            evaporation:   {:>5.3}",
            self.initial_value, self.evaporation_rate
        )
    }
}
