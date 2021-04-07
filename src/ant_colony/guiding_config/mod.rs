use std::fmt::Display;

use crate::ant_colony::graph::Graph;
use crate::ant_colony::pheromone_updater::UpdaterStringConfig;

/// Common configuration context
/// for pheromone updater and ant dispatcher
///
/// Used for _smart_ estimation of hyperparameters
/// based on some heuristics
pub struct GuidingConfig {
    pub ant_count: usize,
    pub num_of_steps_per_cycle: usize,
    pub pheromone_updater_type: UpdaterStringConfig,
    pub graph_node_count: usize,
    pub graph_edge_count: usize,
    pub graph_min_distance: f32,
    pub graph_max_distance: f32,
    pub graph_avg_distance: f32,
    pub graph_cycle_estimate: Option<f32>,
}

impl GuidingConfig {
    pub fn from_graph(
        ant_count: usize,
        num_of_steps_per_cycle: usize,
        pheromone_updater_type: UpdaterStringConfig,
        graph: &Graph,
    ) -> Self {
        GuidingConfig {
            ant_count,
            num_of_steps_per_cycle,
            pheromone_updater_type,
            graph_node_count: graph.get_amount_of_nodes(),
            graph_edge_count: graph.get_amount_of_edges(),
            graph_min_distance: graph.min_edge_length(),
            graph_max_distance: graph.max_edge_length(),
            graph_avg_distance: graph.avg_edge_length(),
            graph_cycle_estimate: graph.estimate_hamiltonian_cycle(),
        }
    }
}

pub trait WithGuidingConfig: Sized {
    fn guided(_guide: &GuidingConfig) -> Option<Self> {
        Option::None
    }
}

impl Display for GuidingConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Guiding config:\n\t\
            ant count: {}\n\t\
            number of steps per cycle: {}\n\t\
            graph: {} nodes, {} edges, distances: <{}, {}> (avg: {})",
            self.ant_count,
            self.num_of_steps_per_cycle,
            self.graph_node_count,
            self.graph_edge_count,
            self.graph_min_distance,
            self.graph_max_distance,
            self.graph_avg_distance
        )
    }
}
