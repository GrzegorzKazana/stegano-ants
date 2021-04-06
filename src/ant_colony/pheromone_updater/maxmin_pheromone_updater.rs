use itertools::Itertools;
use std::fmt::Display;
use std::str::FromStr;

use crate::ant_colony::graph::RouteCollection;
use crate::ant_colony::guiding_config::{GuidingConfig, WithGuidingConfig};
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::PheromoneUpdater;

pub struct MaxMinPheromoneUpdater {
    max_value: f32,
    evaporation_rate: f32,
    best_route_p: f32,
    target_num_of_steps: usize,
}

impl MaxMinPheromoneUpdater {
    pub fn new(
        max_value: f32,
        evaporation_rate: f32,
        best_route_p: f32,
        target_num_of_steps: usize,
    ) -> Self {
        MaxMinPheromoneUpdater {
            max_value,
            evaporation_rate,
            best_route_p,
            target_num_of_steps,
        }
    }
}

impl PheromoneUpdater for MaxMinPheromoneUpdater {
    fn get_initial_value(&self) -> PheromoneLevel {
        self.max_value
    }

    fn on_after_cycle(&self, pheromone: Pheromone, taken_routes: &RouteCollection) -> Pheromone {
        if let Some(route) = taken_routes.get_shortest_route() {
            let decay = 1.0 - self.evaporation_rate;
            let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

            let route_dist = route.get_distance();
            let route_len = route.get_length();
            let adjusted_route_dist =
                route_dist / route_len as f32 * self.target_num_of_steps as f32;
            let increment = 1.0 / adjusted_route_dist;

            let updated_pheromone =
                route
                    .get_edges()
                    .iter()
                    .fold(decayed_pheromone, |updated_pheromone, edge| {
                        updated_pheromone.increase_pheromone_value(edge.key, increment)
                    });

            let max = 1.0 / (self.evaporation_rate * adjusted_route_dist);

            let p_root = self
                .best_route_p
                .powf(1.0 / self.target_num_of_steps as f32);
            let avg = self.target_num_of_steps / 2;
            let min = max * (1.0 - p_root) / ((avg - 1) as f32 * p_root);

            updated_pheromone.clamp(min, max)
        } else {
            pheromone
        }
    }
}

impl FromStr for MaxMinPheromoneUpdater {
    type Err = &'static str;

    fn from_str(opts: &str) -> Result<Self, Self::Err> {
        let error = "Failed to parse opts of MaxMinPheromoneUpdater";

        let (max_value, evaporation_rate, best_route_p, target_num_of_steps): (f32, f32, f32, f32) =
            opts.splitn(4, ',')
                .map(str::parse)
                .filter_map(Result::ok)
                .collect_tuple()
                .ok_or(error)?;

        Ok(MaxMinPheromoneUpdater::new(
            max_value,
            evaporation_rate,
            best_route_p,
            target_num_of_steps as usize,
        ))
    }
}

impl WithGuidingConfig for MaxMinPheromoneUpdater {}

impl Display for MaxMinPheromoneUpdater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MaxMin updater\n\t\
            max value:          {:>5}\n\t\
            evaporation rate:   {:>5.3}\n\t\
            target steps:       {:>5.3}",
            self.max_value, self.evaporation_rate, self.target_num_of_steps
        )
    }
}
