mod _tests;
mod _union;
mod average_pheromone_updater;
mod colony_pheromone_updater;
mod constant_pheromone_updater;
mod cyclical_pheromone_updater;
mod maxmin_pheromone_updater;

use std::fmt::Display;
use std::str::FromStr;

use crate::ant_colony::graph::{RouteBatch, RouteBatchWithHoles, RouteCollection};
use crate::ant_colony::guiding_config::WithGuidingConfig;
use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};

pub use _union::Updaters;
pub use average_pheromone_updater::AveragePheromoneUpdater;
pub use colony_pheromone_updater::ColonyPheromoneUpdater;
pub use constant_pheromone_updater::ConstantPheromoneUpdater;
pub use cyclical_pheromone_updater::CyclicalPheromoneUpdater;
pub use maxmin_pheromone_updater::MaxMinPheromoneUpdater;

pub trait PheromoneUpdater: WithGuidingConfig + Display + Sized + FromStr {
    fn get_initial_value(&self) -> PheromoneLevel;

    fn initialize(&self, init_pheromone: Pheromone, edges: &RouteBatch) -> Pheromone {
        let initial_val = self.get_initial_value();

        edges.iter().fold(init_pheromone, |pheromone, edge| {
            pheromone.initialize_pheromone_for_edge(edge.key, initial_val)
        })
    }

    fn on_after_step(&self, pheromone: Pheromone, _taken_edges: &RouteBatchWithHoles) -> Pheromone {
        pheromone
    }

    fn on_after_cycle(&self, pheromone: Pheromone, _taken_routes: &RouteCollection) -> Pheromone {
        pheromone
    }
}

#[derive(Debug, Clone)]
pub enum UpdaterStringConfig {
    Const(String),
    Average(String),
    Cyclical(String),
    Colony(String),
    MaxMin(String),
}

impl FromStr for UpdaterStringConfig {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config_iter = s.split(":");
        let name = config_iter.next().unwrap_or_default();
        let opts = config_iter.next().map(String::from).unwrap_or_default();

        match name {
            "const" => Some(Self::Const(opts)),
            "avg" => Some(Self::Average(opts)),
            "cycle" => Some(Self::Cyclical(opts)),
            "colony" => Some(Self::Colony(opts)),
            "maxmin" => Some(Self::MaxMin(opts)),
            _ => None,
        }
        .ok_or("Failed to parse pheromone updater type")
    }
}

impl ToString for UpdaterStringConfig {
    fn to_string(&self) -> String {
        match self {
            Self::Const(opts) => format!("const:{}", opts),
            Self::Average(opts) => format!("avg:{}", opts),
            Self::Cyclical(opts) => format!("cycle:{}", opts),
            Self::Colony(opts) => format!("colony:{}", opts),
            Self::MaxMin(opts) => format!("maxmin:{}", opts),
        }
    }
}
