use std::fmt::Display;

use crate::ant_colony::graph::Route;

pub struct CycleSummary {
    pub cycle_idx: usize,
    pub exec_time_ms: u128,
    pub shortest_dist: Option<f32>,
    pub avg_dist: f32,
    pub n_non_empty_edges: usize,
}

impl Display for CycleSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cycle #{:<3} {:>5}ms  |  non empty edges: {:>10}  |  avg path length: {:>10.3}  |  shortest length: {:>10.3}",
            self.cycle_idx,
            self.exec_time_ms,
            self.n_non_empty_edges,
            self.avg_dist,
            self.shortest_dist.unwrap_or(0.0)
        )
    }
}

pub struct EpochSummary {
    pub epoch_idx: usize,
    pub exec_time_ms: u128,
    pub shortest_route: Option<Route>,
}

impl Display for EpochSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_appendix = match &self.shortest_route {
            Option::Some(route) => format!(
                "\n\tshortest path ({}): {:?}",
                route.get_distance(),
                route.get_nodes()
            ),
            Option::None => String::from(""),
        };

        write!(
            f,
            "Epoch #{:<3} {:>5}ms{}",
            self.epoch_idx, self.exec_time_ms, path_appendix
        )
    }
}
