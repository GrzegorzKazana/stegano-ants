use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

use crate::ant_colony::graph::{RouteBatchWithHoles, RouteCollection};
use crate::ant_colony::guiding_config::{GuidingConfig, WithGuidingConfig};
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::PheromoneUpdater;

/// After all ants execute a single step, decays the pheromone level
/// and increases pheromone value of taken edges by a contant value.
///
/// After whole cycle is finished, increases pheromone levels of edges
/// that belong to the shortest route.
pub struct ColonyPheromoneUpdater {
    // advised to be (n / tour length), n - number of cities
    initial_and_step_increment: f32,
    step_evaporation_rate: f32,
    cycle_evaporation_rate: f32,
    target_num_of_steps: usize,
}

impl ColonyPheromoneUpdater {
    pub fn new(
        initial_and_step_increment: f32,
        step_evaporation_rate: f32,
        cycle_evaporation_rate: f32,
        target_num_of_steps: usize,
    ) -> Self {
        ColonyPheromoneUpdater {
            initial_and_step_increment,
            step_evaporation_rate,
            cycle_evaporation_rate,
            target_num_of_steps,
        }
    }
}

impl PheromoneUpdater for ColonyPheromoneUpdater {
    fn get_initial_value(&self) -> PheromoneLevel {
        self.initial_and_step_increment
    }

    fn on_after_step(&self, pheromone: Pheromone, taken_edges: &RouteBatchWithHoles) -> Pheromone {
        let decay = 1.0 - self.step_evaporation_rate;
        let increment = self.step_evaporation_rate * self.initial_and_step_increment;

        let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

        taken_edges.iter().filter_map(|a| a.as_ref()).fold(
            decayed_pheromone,
            |updated_pheromone, taken_edge| {
                updated_pheromone.increase_pheromone_value(taken_edge.key, increment)
            },
        )
    }

    fn on_after_cycle(&self, pheromone: Pheromone, taken_routes: &RouteCollection) -> Pheromone {
        let decay = 1.0 - self.cycle_evaporation_rate;

        let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

        match taken_routes.get_shortest_route() {
            Option::None => decayed_pheromone,
            Option::Some(route) => {
                let route_dist = route.get_adjusted_distance(self.target_num_of_steps);

                route.get_edges().iter().fold(
                    decayed_pheromone,
                    |edge_updated_route, taken_edge| {
                        let increment = self.cycle_evaporation_rate / route_dist;

                        edge_updated_route.increase_pheromone_value(taken_edge.key, increment)
                    },
                )
            }
        }
    }
}

impl FromStr for ColonyPheromoneUpdater {
    type Err = &'static str;

    fn from_str(opts: &str) -> Result<Self, Self::Err> {
        let error = "Failed to parse opts of ColonyPheromoneUpdater";

        let (
            initial_and_step_increment,
            step_evaporation_rate,
            cycle_evaporation_rate,
            target_len,
        ): (f32, f32, f32, f32) = opts
            .splitn(4, ',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect_tuple()
            .ok_or(error)?;

        Ok(ColonyPheromoneUpdater::new(
            initial_and_step_increment,
            step_evaporation_rate,
            cycle_evaporation_rate,
            target_len as usize,
        ))
    }
}

impl WithGuidingConfig for ColonyPheromoneUpdater {
    fn guided(guide: &GuidingConfig) -> Option<Self> {
        Some(ColonyPheromoneUpdater::new(
            (guide.num_of_steps_per_cycle as f32).powi(3) / guide.graph_avg_distance,
            0.1,
            0.1,
            guide.num_of_steps_per_cycle,
        ))
    }
}

impl Display for ColonyPheromoneUpdater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pheromone updater (System)\n\t\
            initial value and step increment: {:>5}\n\t\
            local evaporation:   {:>5.3}\n\t\
            global evaporation:   {:>5.3}",
            self.initial_and_step_increment,
            self.step_evaporation_rate,
            self.cycle_evaporation_rate,
        )
    }
}
