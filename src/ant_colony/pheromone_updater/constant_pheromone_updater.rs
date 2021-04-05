use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

use crate::ant_colony::graph::RouteBatchWithHoles;
use crate::ant_colony::guiding_config::{GuidingConfig, WithGuidingConfig};
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::PheromoneUpdater;

/// After all ants execute a single step, decays the pheromone level
/// and increases pheromone value of taken edges by a constant value.
///
/// If edge was taken by multiple ants it will be increased multiple times.
///
/// Does not perform updates after whole cycle.
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

    fn on_after_step(&self, pheromone: Pheromone, taken_edges: &RouteBatchWithHoles) -> Pheromone {
        let decay = 1.0 - self.evaporation_rate;

        let decayed_pheromone = pheromone.scale_all_pheromone_values(decay);

        taken_edges.iter().filter_map(|a| a.as_ref()).fold(
            decayed_pheromone,
            |updated_pheromone, taken_edge| {
                updated_pheromone.increase_pheromone_value(taken_edge.key, self.increment)
            },
        )
    }
}

impl FromStr for ConstantPheromoneUpdater {
    type Err = &'static str;

    fn from_str(opts: &str) -> Result<Self, Self::Err> {
        let error = "Failed to parse opts of ConstantPheromoneUpdater";

        let (initial_value, evaporation_rate, increment): (f32, f32, f32) = opts
            .splitn(3, ',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect_tuple()
            .ok_or(error)?;

        Ok(ConstantPheromoneUpdater::new(
            initial_value,
            evaporation_rate,
            increment,
        ))
    }
}

impl WithGuidingConfig for ConstantPheromoneUpdater {
    fn guided(_guide: &GuidingConfig) -> Option<Self> {
        Some(ConstantPheromoneUpdater::new(1.0, 0.001, 1.0))
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
