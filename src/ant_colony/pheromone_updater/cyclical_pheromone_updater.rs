use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

use crate::ant_colony::graph::RouteCollection;
use crate::ant_colony::guiding_config::WithGuidingConfig;
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
    target_num_of_steps: usize,
}

impl CyclicalPheromoneUpdater {
    pub fn new(
        initial_value: f32,
        evaporation_rate: f32,
        increment: f32,
        target_num_of_steps: usize,
    ) -> Self {
        CyclicalPheromoneUpdater {
            initial_value,
            evaporation_rate,
            increment,
            target_num_of_steps,
        }
    }
}

impl PheromoneUpdater for CyclicalPheromoneUpdater {
    fn get_initial_value(&self) -> PheromoneLevel {
        self.initial_value
    }

    fn on_after_cycle(&self, pheromone: Pheromone, taken_routes: &RouteCollection) -> Pheromone {
        let decay = 1.0 - self.evaporation_rate;
        let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

        taken_routes.get_routes().iter().fold(
            decayed_pheromone,
            |route_updated_pheromone, taken_route| {
                let route_dist = taken_route.get_adjusted_distance(self.target_num_of_steps);

                taken_route.get_edges().iter().fold(
                    route_updated_pheromone,
                    |edge_updated_route, taken_edge| {
                        let increment = self.increment / route_dist;

                        edge_updated_route.increase_pheromone_value(taken_edge.key, increment)
                    },
                )
            },
        )
    }
}

impl FromStr for CyclicalPheromoneUpdater {
    type Err = &'static str;

    fn from_str(opts: &str) -> Result<Self, Self::Err> {
        let error = "Failed to parse opts of CyclicalPheromoneUpdater";

        let (initial_value, evaporation_rate, increment, target_len): (f32, f32, f32, f32) = opts
            .splitn(4, ',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect_tuple()
            .ok_or(error)?;

        Ok(CyclicalPheromoneUpdater::new(
            initial_value,
            evaporation_rate,
            increment,
            target_len as usize,
        ))
    }
}

impl WithGuidingConfig for CyclicalPheromoneUpdater {
    fn guided(guide: &crate::ant_colony::guiding_config::GuidingConfig) -> Option<Self> {
        Some(CyclicalPheromoneUpdater::new(
            1.0,
            0.3,
            1.0,
            guide.num_of_steps_per_cycle,
        ))
    }
}

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
