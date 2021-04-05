use std::{fmt::Display, str::FromStr};

use crate::ant_colony::graph::{RouteBatchWithHoles, RouteCollection};
use crate::ant_colony::guiding_config::WithGuidingConfig;
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::{
    AveragePheromoneUpdater, ColonyPheromoneUpdater, ConstantPheromoneUpdater,
    CyclicalPheromoneUpdater, PheromoneUpdater, UpdaterStringConfig,
};
use crate::ant_colony::guiding_config::GuidingConfig;

/// using an enum instead of run-time
/// polymorhism to avoid cost of dynamic dispatch
pub enum Updaters {
    Average(AveragePheromoneUpdater),
    Const(ConstantPheromoneUpdater),
    Cyclical(CyclicalPheromoneUpdater),
    Colony(ColonyPheromoneUpdater),
}

impl PheromoneUpdater for Updaters {
    fn get_initial_value(&self) -> PheromoneLevel {
        match self {
            Updaters::Average(updater) => updater.get_initial_value(),
            Updaters::Const(updater) => updater.get_initial_value(),
            Updaters::Cyclical(updater) => updater.get_initial_value(),
            Updaters::Colony(updater) => updater.get_initial_value(),
        }
    }

    fn on_after_step(&self, pheromone: Pheromone, taken_edges: &RouteBatchWithHoles) -> Pheromone {
        match self {
            Updaters::Average(updater) => updater.on_after_step(pheromone, taken_edges),
            Updaters::Const(updater) => updater.on_after_step(pheromone, taken_edges),
            Updaters::Cyclical(updater) => updater.on_after_step(pheromone, taken_edges),
            Updaters::Colony(updater) => updater.on_after_step(pheromone, taken_edges),
        }
    }

    fn on_after_cycle(&self, pheromone: Pheromone, taken_routes: &RouteCollection) -> Pheromone {
        match self {
            Updaters::Average(updater) => updater.on_after_cycle(pheromone, taken_routes),
            Updaters::Const(updater) => updater.on_after_cycle(pheromone, taken_routes),
            Updaters::Cyclical(updater) => updater.on_after_cycle(pheromone, taken_routes),
            Updaters::Colony(updater) => updater.on_after_cycle(pheromone, taken_routes),
        }
    }
}

impl FromStr for Updaters {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config = UpdaterStringConfig::from_str(s)?;

        Self::from_string_config(&config, None).ok_or("Failed to parse PheromoneUpdaters")
    }
}

impl WithGuidingConfig for Updaters {}

impl Display for Updaters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Updaters::Average(updater) => updater.fmt(f),
            Updaters::Const(updater) => updater.fmt(f),
            Updaters::Cyclical(updater) => updater.fmt(f),
            Updaters::Colony(updater) => updater.fmt(f),
        }
    }
}

impl Updaters {
    pub fn from_string_config(
        config: &UpdaterStringConfig,
        maybe_guide: Option<&GuidingConfig>,
    ) -> Option<Updaters> {
        match config {
            UpdaterStringConfig::Const(opts) => ConstantPheromoneUpdater::from_str(opts)
                .ok()
                .or_else(|| maybe_guide.and_then(ConstantPheromoneUpdater::guided))
                .map(Self::Const),

            UpdaterStringConfig::Average(opts) => AveragePheromoneUpdater::from_str(opts)
                .ok()
                .or_else(|| maybe_guide.and_then(AveragePheromoneUpdater::guided))
                .map(Self::Average),

            UpdaterStringConfig::Cyclical(opts) => CyclicalPheromoneUpdater::from_str(opts)
                .ok()
                .or_else(|| maybe_guide.and_then(CyclicalPheromoneUpdater::guided))
                .map(Self::Cyclical),

            UpdaterStringConfig::Colony(opts) => ColonyPheromoneUpdater::from_str(opts)
                .ok()
                .or_else(|| maybe_guide.and_then(ColonyPheromoneUpdater::guided))
                .map(Self::Colony),
        }
    }
}
