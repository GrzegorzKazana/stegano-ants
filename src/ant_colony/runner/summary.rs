use std::fmt::Display;

use crate::ant_colony::graph::Route;

#[derive(Clone)]
pub struct CycleSummary {
    pub cycle_idx: usize,
    pub exec_time_ms: u128,
    pub shortest_dist: Option<f32>,
    pub shortest_path_length: Option<usize>,
    pub avg_dist: f32,
    pub n_non_empty_edges: usize,
    pub ratio_of_incomplete_routes: f32,
    pub pheromone_variance: f32,
    pub shortest_route: Option<Route>,
}

impl Display for CycleSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cycle #{:<3} {:>5}ms  |  non empty edges: {:>10}  |  avg path length: {:>10.3}  ({:>4.1}%)  |  shortest length: {:>10.3}  ({:>3})  |  pheromone variance: {:>4.5}",
            self.cycle_idx,
            self.exec_time_ms,
            self.n_non_empty_edges,
            self.avg_dist,
            self.ratio_of_incomplete_routes,
            self.shortest_dist.unwrap_or(0.0),
            self.shortest_path_length.unwrap_or(0),
            self.pheromone_variance
        )
    }
}

#[derive(Clone)]

pub struct EpochSummary {
    pub epoch_idx: usize,
    pub exec_time_ms: u128,
    pub shortest_route: Option<Route>,
    pub shortest_route_cycle_idx: Option<usize>,
}

impl Display for EpochSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_appendix = match self
            .shortest_route
            .as_ref()
            .zip(self.shortest_route_cycle_idx)
        {
            Option::Some((route, cycle_idx)) => format!(
                "\n\tshortest path #{} ({}): {:?}",
                cycle_idx,
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
