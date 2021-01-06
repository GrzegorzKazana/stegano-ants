mod io;

use std::fmt::Display;

use crate::ant_colony::colony::ColonyTrait;
use crate::ant_colony::graph::Graph;
use crate::ant_colony::pheromone_reader::PheromoneReader;
use crate::common::utils::measure;

pub use io::{CliOutput, CommandLine, DummyOutput};

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
            "Cycle #{:<5} ({:>6}ms)\n\t\
            non empty edges: {:>10}\n\t\
            avg path length: {:>10.3}\n\t\
            shortest length: {:>10}\n",
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
}

impl Display for EpochSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Epoch #{:<5} ({:>6})ms\n",
            self.epoch_idx, self.exec_time_ms,
        )
    }
}

pub struct ColonyRunner<'a, C: ColonyTrait, IO: CliOutput> {
    colony: C,
    graph: &'a Graph,
    io: IO,
    cycle_history: Vec<CycleSummary>,
    epoch_history: Vec<EpochSummary>,
}

impl<'a, C: ColonyTrait, IO: CliOutput> ColonyRunner<'a, C, IO> {
    pub fn new(colony: C, graph: &'a Graph, io: IO) -> Self {
        ColonyRunner {
            colony,
            graph,
            io,
            cycle_history: Vec::new(),
            epoch_history: Vec::new(),
        }
    }

    pub fn train(self, n_cycles: usize) -> Self {
        let ColonyRunner {
            colony: init_colony,
            graph,
            io,
            mut cycle_history,
            mut epoch_history,
        } = self;

        let n_prev_cycles = cycle_history.len();

        let (colony, next_cycle_history) = (0..n_cycles).fold(
            (init_colony, Vec::with_capacity(n_cycles)),
            |(colony, mut summaries), cycle_idx| {
                let (new_colony, exec_time_ms) = measure(|| colony.execute_cycle(cycle_idx));

                let (pheromone, routes) = new_colony.get_progress();

                let summary = CycleSummary {
                    cycle_idx: n_prev_cycles + cycle_idx,
                    exec_time_ms,
                    shortest_dist: routes.get_shortest_route_distance(),
                    avg_dist: routes.get_average_route_distance(),
                    n_non_empty_edges: PheromoneReader::count_edges_with_pheromone_above(
                        pheromone, 0.01,
                    ),
                };

                io.print(&summary);
                summaries.push(summary);

                (new_colony, summaries)
            },
        );

        let epoch_summary = EpochSummary {
            epoch_idx: epoch_history.len() + 1,
            exec_time_ms: next_cycle_history
                .iter()
                .fold(0, |acc, cycle| acc + cycle.exec_time_ms),
        };

        io.print(&epoch_summary);

        cycle_history.extend(next_cycle_history);
        epoch_history.push(epoch_summary);

        ColonyRunner {
            colony,
            graph,
            io,
            cycle_history,
            epoch_history,
        }
    }
}
