use std::fmt::Display;
use std::str::FromStr;

use crate::ant_colony::graph::{AdjacencyListEntry, Graph};
use crate::ant_colony::pheromone::Pheromone;
use crate::ant_colony::{ant::Ant, guiding_config::WithGuidingConfig};

use super::{
    AntDispatcher, BasicAntDispatcher, BiasedAntDispatcher, ColonyAntDispatcher,
    DispatcherStringConfig,
};
use crate::ant_colony::guiding_config::GuidingConfig;

/// using an enum instead of run-time
/// polymorhism to avoid cost of dynamic dispatch
pub enum Dispatchers {
    Basic(BasicAntDispatcher),
    Biased(BiasedAntDispatcher),
    Colony(ColonyAntDispatcher),
}

impl Display for Dispatchers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dispatchers::Basic(dispatcher) => dispatcher.fmt(f),
            Dispatchers::Biased(dispatcher) => dispatcher.fmt(f),
            Dispatchers::Colony(dispatcher) => dispatcher.fmt(f),
        }
    }
}

impl AntDispatcher for Dispatchers {
    fn select_next_edge(
        &self,
        ant: &Ant,
        graph: &Graph,
        pheromone: &Pheromone,
        sample_seed: f32,
        strategy_seed: f32,
    ) -> Option<AdjacencyListEntry> {
        match self {
            Dispatchers::Basic(dispatcher) => {
                dispatcher.select_next_edge(ant, graph, pheromone, sample_seed, strategy_seed)
            }
            Dispatchers::Biased(dispatcher) => {
                dispatcher.select_next_edge(ant, graph, pheromone, sample_seed, strategy_seed)
            }
            Dispatchers::Colony(dispatcher) => {
                dispatcher.select_next_edge(ant, graph, pheromone, sample_seed, strategy_seed)
            }
        }
    }
}

impl FromStr for Dispatchers {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config = DispatcherStringConfig::from_str(s)?;

        Dispatchers::from_string_config(&config, None).ok_or("Failed to parse Dispatcher")
    }
}

impl WithGuidingConfig for Dispatchers {}

impl Dispatchers {
    pub fn from_string_config(
        config: &DispatcherStringConfig,
        maybe_guide: Option<&GuidingConfig>,
    ) -> Option<Dispatchers> {
        match config {
            DispatcherStringConfig::Basic(opts) => BasicAntDispatcher::from_str(opts)
                .ok()
                .or_else(|| maybe_guide.and_then(BasicAntDispatcher::guided))
                .map(Self::Basic),

            DispatcherStringConfig::Biased(opts) => BiasedAntDispatcher::from_str(opts)
                .ok()
                .or_else(|| maybe_guide.and_then(BiasedAntDispatcher::guided))
                .map(Self::Biased),

            DispatcherStringConfig::Colony(opts) => ColonyAntDispatcher::from_str(opts)
                .ok()
                .or_else(|| maybe_guide.and_then(ColonyAntDispatcher::guided))
                .map(Self::Colony),
        }
    }
}
