mod io;
mod summary;

use crate::ant_colony::colony::Colony;
use crate::ant_colony::graph::Graph;
use crate::ant_colony::pheromone_reader::PheromoneReader;
use crate::common::utils::measure;

pub use io::{CliOutput, CommandLine, DummyOutput};
pub use summary::{CycleSummary, EpochSummary};

pub struct ColonyRunner<'a, C: Colony, IO: CliOutput> {
    colony: C,
    graph: &'a Graph,
    io: IO,
    cycle_history: Vec<CycleSummary>,
    epoch_history: Vec<EpochSummary>,
}

impl<'a, C: Colony, IO: CliOutput> ColonyRunner<'a, C, IO> {
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

                let pheromone = new_colony.get_pheromone();
                let routes = new_colony.get_routes();

                let summary = CycleSummary {
                    cycle_idx: n_prev_cycles + cycle_idx,
                    exec_time_ms,
                    shortest_dist: routes.get_shortest_route_distance(),
                    avg_dist: routes.get_average_route_distance(),
                    n_non_empty_edges: PheromoneReader::count_edges_with_pheromone_above(
                        pheromone, 0.1,
                    ),
                };

                io.print_cycle_summary(&summary);
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

        io.print_epoch_summary(&epoch_summary);

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
