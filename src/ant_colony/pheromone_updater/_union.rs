use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

use crate::ant_colony::graph::{RouteBatchWithHoles, RouteCollection};
use crate::ant_colony::guided_configuration::WithGuidingConfig;
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::{
    AveragePheromoneUpdater, ConstantPheromoneUpdater, CyclicalPheromoneUpdater, PheromoneUpdater,
    SystemPheromoneUpdater,
};
use crate::ant_colony::guided_configuration::GuidingConfig;

/// using an enum instead of run-time
/// polymorhism to avoid cost of dynamic dispatch
pub enum Updaters {
    Average(AveragePheromoneUpdater),
    Const(ConstantPheromoneUpdater),
    Cyclical(CyclicalPheromoneUpdater),
    System(SystemPheromoneUpdater),
}

impl PheromoneUpdater for Updaters {
    fn get_initial_value(&self) -> PheromoneLevel {
        match self {
            Updaters::Average(updater) => updater.get_initial_value(),
            Updaters::Const(updater) => updater.get_initial_value(),
            Updaters::Cyclical(updater) => updater.get_initial_value(),
            Updaters::System(updater) => updater.get_initial_value(),
        }
    }

    fn on_after_step(&self, pheromone: Pheromone, taken_edges: &RouteBatchWithHoles) -> Pheromone {
        match self {
            Updaters::Average(updater) => updater.on_after_step(pheromone, taken_edges),
            Updaters::Const(updater) => updater.on_after_step(pheromone, taken_edges),
            Updaters::Cyclical(updater) => updater.on_after_step(pheromone, taken_edges),
            Updaters::System(updater) => updater.on_after_step(pheromone, taken_edges),
        }
    }

    fn on_after_cycle(&self, pheromone: Pheromone, taken_routes: &RouteCollection) -> Pheromone {
        match self {
            Updaters::Average(updater) => updater.on_after_cycle(pheromone, taken_routes),
            Updaters::Const(updater) => updater.on_after_cycle(pheromone, taken_routes),
            Updaters::Cyclical(updater) => updater.on_after_cycle(pheromone, taken_routes),
            Updaters::System(updater) => updater.on_after_cycle(pheromone, taken_routes),
        }
    }
}

impl WithGuidingConfig for Updaters {}

impl Display for Updaters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Updaters::Average(updater) => updater.fmt(f),
            Updaters::Const(updater) => updater.fmt(f),
            Updaters::Cyclical(updater) => updater.fmt(f),
            Updaters::System(updater) => updater.fmt(f),
        }
    }
}

impl Updaters {
    pub fn from_string(config: &str, maybe_guide: Option<&GuidingConfig>) -> Option<Updaters> {
        let mut config_iter = config.split(":");
        let name = config_iter.next().unwrap_or_default();
        let opts = config_iter.next().unwrap_or_default();

        Self::from_string_with_opts(name, opts)
            .or_else(|| maybe_guide.and_then(|guide| Self::from_string_with_guide(name, guide)))
    }

    fn from_string_with_opts(name: &str, opts: &str) -> Option<Self> {
        match name {
            "avg" => {
                let (initial_value_str, evaporation_rate_str, increment_str): (&str, &str, &str) =
                    opts.splitn(3, ',').collect_tuple()?;

                let initial_value = initial_value_str.parse().ok()?;
                let evaporation_rate = evaporation_rate_str.parse().ok()?;
                let increment = increment_str.parse().ok()?;

                let updater =
                    AveragePheromoneUpdater::new(initial_value, evaporation_rate, increment);

                Option::Some(Self::Average(updater))
            }

            "const" => {
                let (initial_value_str, evaporation_rate_str, increment_str): (&str, &str, &str) =
                    opts.splitn(3, ',').collect_tuple()?;

                let initial_value = initial_value_str.parse().ok()?;
                let evaporation_rate = evaporation_rate_str.parse().ok()?;
                let increment = increment_str.parse().ok()?;

                let updater =
                    ConstantPheromoneUpdater::new(initial_value, evaporation_rate, increment);

                Option::Some(Self::Const(updater))
            }

            "cycle" => {
                let (initial_value_str, evaporation_rate_str, increment_str): (&str, &str, &str) =
                    opts.splitn(3, ',').collect_tuple()?;

                let initial_value = initial_value_str.parse().ok()?;
                let evaporation_rate = evaporation_rate_str.parse().ok()?;
                let increment = increment_str.parse().ok()?;

                let updater =
                    CyclicalPheromoneUpdater::new(initial_value, evaporation_rate, increment);

                Option::Some(Self::Cyclical(updater))
            }

            "system" => {
                let (initial_value_str, evaporation_rate_str): (&str, &str) =
                    opts.splitn(2, ',').collect_tuple()?;

                let initial_value = initial_value_str.parse().ok()?;
                let evaporation_rate = evaporation_rate_str.parse().ok()?;

                let updater = SystemPheromoneUpdater::new(initial_value, evaporation_rate);

                Option::Some(Self::System(updater))
            }

            _ => Option::None,
        }
    }

    fn from_string_with_guide(name: &str, guide: &GuidingConfig) -> Option<Self> {
        match name {
            "avg" => AveragePheromoneUpdater::guided(guide).map(Self::Average),
            "const" => ConstantPheromoneUpdater::guided(guide).map(Self::Const),
            "cycle" => CyclicalPheromoneUpdater::guided(guide).map(Self::Cyclical),
            "system" => SystemPheromoneUpdater::guided(guide).map(Self::System),
            _ => Option::None,
        }
    }
}

impl FromStr for Updaters {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s, Option::None).ok_or("Failed to parse Updater")
    }
}
