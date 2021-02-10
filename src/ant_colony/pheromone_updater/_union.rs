use itertools::Itertools;
use std::fmt::Display;

use crate::ant_colony::graph::{RouteBatch, RouteBatchWithHoles, RouteCollection};
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

use super::{
    AveragePheromoneUpdater, ConstantPheromoneUpdater, CyclicalPheromoneUpdater, PheromoneUpdater,
    SystemPheromoneUpdater,
};

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
    pub fn from_string(config: &str) -> Option<Updaters> {
        let (name, opts): (&str, &str) = config.splitn(2, ':').collect_tuple()?;

        match name {
            "avg" => {
                let (initial_value_str, evaporation_rate_str, increment_str): (&str, &str, &str) =
                    opts.splitn(3, ',').collect_tuple()?;

                let initial_value = initial_value_str.parse().ok()?;
                let evaporation_rate = evaporation_rate_str.parse().ok()?;
                let increment = increment_str.parse().ok()?;

                let updater =
                    AveragePheromoneUpdater::new(initial_value, evaporation_rate, increment);

                Option::Some(Updaters::Average(updater))
            }

            "const" => {
                let (initial_value_str, evaporation_rate_str, increment_str): (&str, &str, &str) =
                    opts.splitn(2, ',').collect_tuple()?;

                let initial_value = initial_value_str.parse().ok()?;
                let evaporation_rate = evaporation_rate_str.parse().ok()?;
                let increment = increment_str.parse().ok()?;

                let updater =
                    ConstantPheromoneUpdater::new(initial_value, evaporation_rate, increment);

                Option::Some(Updaters::Const(updater))
            }

            "cycle" => {
                let (initial_value_str, evaporation_rate_str, increment_str): (&str, &str, &str) =
                    opts.splitn(3, ',').collect_tuple()?;

                let initial_value = initial_value_str.parse().ok()?;
                let evaporation_rate = evaporation_rate_str.parse().ok()?;
                let increment = increment_str.parse().ok()?;

                let updater =
                    CyclicalPheromoneUpdater::new(initial_value, evaporation_rate, increment);

                Option::Some(Updaters::Cyclical(updater))
            }

            "system" => {
                let (initial_value_str, evaporation_rate_str): (&str, &str) =
                    opts.splitn(2, ',').collect_tuple()?;

                let initial_value = initial_value_str.parse().ok()?;
                let evaporation_rate = evaporation_rate_str.parse().ok()?;

                let updater = SystemPheromoneUpdater::new(initial_value, evaporation_rate);

                Option::Some(Updaters::System(updater))
            }

            _ => Option::None,
        }
    }
}
