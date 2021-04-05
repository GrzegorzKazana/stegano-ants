use itertools::Itertools;
use std::fmt::Display;
use std::str::FromStr;

use crate::ant_colony::graph::AdjacencyListEntry;
use crate::ant_colony::guiding_config::{GuidingConfig, WithGuidingConfig};
use crate::ant_colony::pheromone::Pheromone;
use crate::ant_colony::pheromone_updater::UpdaterStringConfig;

use super::LikelihoodAntDispatcher;

/// Each edge is attributed probability proportional to
/// pheromone level and inversly to distance.
/// Both factors are raised to given powers to control each importance.
///
/// p_{if}=\frac{\tau_{ij}^a}{d_{ij}^b}
pub struct BiasedAntDispatcher {
    pheromone_bias: f32,
    visibility_bias: f32,
}

impl BiasedAntDispatcher {
    pub fn new(pheromone_bias: f32, visibility_bias: f32) -> Self {
        BiasedAntDispatcher {
            pheromone_bias,
            visibility_bias,
        }
    }
}

impl LikelihoodAntDispatcher for BiasedAntDispatcher {
    fn cacluclate_node_likelihoods(
        &self,
        possible_next_edges: &[AdjacencyListEntry],
        pheromone: &Pheromone,
    ) -> Vec<f32> {
        possible_next_edges
            .iter()
            .map(|edge| {
                let pheromone_level = pheromone.get_pheromone_for_edge(edge.key);

                edge.visibility.powf(self.visibility_bias)
                    * pheromone_level.powf(self.pheromone_bias)
            })
            .collect::<Vec<_>>()
    }
}

impl FromStr for BiasedAntDispatcher {
    type Err = &'static str;

    fn from_str(opts: &str) -> Result<Self, Self::Err> {
        let error = "Failed to parse opts of BiasedAntDispatcher";

        let (pheromone_bias, visibility_bias): (f32, f32) = opts
            .splitn(2, ',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect_tuple()
            .ok_or(error)?;

        Ok(BiasedAntDispatcher::new(pheromone_bias, visibility_bias))
    }
}

impl WithGuidingConfig for BiasedAntDispatcher {
    fn guided(guide: &GuidingConfig) -> Option<Self> {
        // values based on experiments and `Dorigo1991AntSA`
        let (pheromone_bias, visibility_bias) = match guide.pheromone_updater_type {
            UpdaterStringConfig::Const(_) => (2.0, 2.0),
            UpdaterStringConfig::Average(_) => (1.0, 1.0),
            UpdaterStringConfig::Cyclical(_) => (1.0, 2.5),
            _ => (1.0, 1.0),
        };

        Some(BiasedAntDispatcher::new(pheromone_bias, visibility_bias))
    }
}

impl Display for BiasedAntDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ant dispatcher (Biased)\n\t\
            pheromone bias:  {:>5.3}\n\t\
            visibility bias: {:>5.3}",
            self.pheromone_bias, self.visibility_bias
        )
    }
}
