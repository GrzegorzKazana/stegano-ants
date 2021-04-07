use itertools::Itertools;
use std::fmt::Display;
use std::str::FromStr;

use crate::ant_colony::graph::RouteCollection;
use crate::ant_colony::guiding_config::{GuidingConfig, WithGuidingConfig};
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::PheromoneUpdater;

pub struct MaxMinPheromoneUpdater {
    route_estimate: f32,
    evaporation_rate: f32,
    best_route_p: f32,
    target_num_of_steps: usize,
}

impl MaxMinPheromoneUpdater {
    pub fn new(
        route_estimate: f32,
        evaporation_rate: f32,
        best_route_p: f32,
        target_num_of_steps: usize,
    ) -> Self {
        MaxMinPheromoneUpdater {
            route_estimate,
            evaporation_rate,
            best_route_p,
            target_num_of_steps,
        }
    }

    fn pher_max(&self, route_distance: f32) -> f32 {
        1.0 / (self.evaporation_rate * route_distance)
    }

    fn pher_min(&self, pher_max: f32) -> f32 {
        let p_root = self
            .best_route_p
            .powf(1.0 / self.target_num_of_steps as f32);

        let avg = self.target_num_of_steps / 2;

        pher_max * (1.0 - p_root) / ((avg - 1) as f32 * p_root)
    }
}

impl PheromoneUpdater for MaxMinPheromoneUpdater {
    fn get_initial_value(&self) -> PheromoneLevel {
        self.pher_max(self.route_estimate)
    }

    fn on_after_cycle(&self, pheromone: Pheromone, taken_routes: &RouteCollection) -> Pheromone {
        if let Some(route) = taken_routes.get_shortest_route() {
            let decay = 1.0 - self.evaporation_rate;
            let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

            let route_dist = route.get_adjusted_distance(self.target_num_of_steps);
            let increment = 1.0 / route_dist;

            let updated_pheromone =
                route
                    .get_edges()
                    .iter()
                    .fold(decayed_pheromone, |updated_pheromone, edge| {
                        updated_pheromone.increase_pheromone_value(edge.key, increment)
                    });

            let max = self.pher_max(route_dist);
            let min = self.pher_min(max);

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

        let (route_estimate, evaporation_rate, best_route_p, target_num_of_steps): (
            f32,
            f32,
            f32,
            f32,
        ) = opts
            .splitn(4, ',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect_tuple()
            .ok_or(error)?;

        Ok(MaxMinPheromoneUpdater::new(
            route_estimate,
            evaporation_rate,
            best_route_p,
            target_num_of_steps as usize,
        ))
    }
}

impl WithGuidingConfig for MaxMinPheromoneUpdater {
    fn guided(guide: &GuidingConfig) -> Option<Self> {
        guide
            .graph_cycle_estimate
            .map(|dist| MaxMinPheromoneUpdater::new(dist, 0.2, 0.1, guide.num_of_steps_per_cycle))
    }
}

impl Display for MaxMinPheromoneUpdater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MaxMin updater\n\t\
            route estimate:          {:>5}\n\t\
            evaporation rate:   {:>5.3}\n\t\
            target steps:       {:>5.3}",
            self.route_estimate, self.evaporation_rate, self.target_num_of_steps
        )
    }
}
