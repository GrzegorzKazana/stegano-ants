use itertools::Itertools;
use std::fmt::Display;
use std::str::FromStr;

use crate::ant_colony::graph::{AdjacencyListEntry, Graph};
use crate::ant_colony::pheromone::Pheromone;
use crate::ant_colony::{ant::Ant, guided_configuration::WithGuidingConfig};

use super::{AntDispatcher, BasicAntDispatcher, BiasedAntDispatcher, SystemAntDispatcher};
use crate::ant_colony::guided_configuration::GuidingConfig;

/// using an enum instead of run-time
/// polymorhism to avoid cost of dynamic dispatch
pub enum Dispatchers {
    Basic(BasicAntDispatcher),
    Biased(BiasedAntDispatcher),
    System(SystemAntDispatcher),
}

impl Display for Dispatchers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dispatchers::Basic(dispatcher) => dispatcher.fmt(f),
            Dispatchers::Biased(dispatcher) => dispatcher.fmt(f),
            Dispatchers::System(dispatcher) => dispatcher.fmt(f),
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
            Dispatchers::System(dispatcher) => {
                dispatcher.select_next_edge(ant, graph, pheromone, sample_seed, strategy_seed)
            }
        }
    }
}

impl WithGuidingConfig for Dispatchers {}

impl Dispatchers {
    pub fn from_string(config: &str, maybe_guide: Option<&GuidingConfig>) -> Option<Dispatchers> {
        let mut config_iter = config.split(":");
        let name = config_iter.next().unwrap_or_default();
        let opts = config_iter.next().unwrap_or_default();

        Self::from_string_with_opts(name, opts)
            .or_else(|| maybe_guide.and_then(|guide| Self::from_string_with_guide(name, guide)))
    }

    fn from_string_with_opts(name: &str, opts: &str) -> Option<Self> {
        match name {
            "basic" => Option::Some(Self::Basic(BasicAntDispatcher)),

            "biased" => {
                let (pheromone_bias_str, visibility_bias_str): (&str, &str) =
                    opts.splitn(2, ',').collect_tuple()?;

                let pheromone_bias = pheromone_bias_str.parse().ok()?;
                let visibility_bias = visibility_bias_str.parse().ok()?;
                let dispatcher = BiasedAntDispatcher::new(pheromone_bias, visibility_bias);

                Option::Some(Self::Biased(dispatcher))
            }

            "system" => {
                let (exploitation_rate_str, visibility_bias_str): (&str, &str) =
                    opts.splitn(2, ',').collect_tuple()?;

                let exploitation_rate = exploitation_rate_str.parse().ok()?;
                let visibility_bias = visibility_bias_str.parse().ok()?;
                let dispatcher = SystemAntDispatcher::new(exploitation_rate, visibility_bias);

                Option::Some(Self::System(dispatcher))
            }

            _ => Option::None,
        }
    }

    fn from_string_with_guide(name: &str, guide: &GuidingConfig) -> Option<Self> {
        match name {
            "basic" => BasicAntDispatcher::guided(guide).map(Self::Basic),
            "biased" => BiasedAntDispatcher::guided(guide).map(Self::Biased),
            "system" => SystemAntDispatcher::guided(guide).map(Self::System),
            _ => Option::None,
        }
    }
}

impl FromStr for Dispatchers {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Dispatchers::from_string(s, Option::None).ok_or("Failed to parse Dispatcher")
    }
}
