mod summary;

use std::rc::Rc;

use crate::ant_colony::colony::Colony;
use crate::ant_colony::graph::{Graph, Route};
use crate::ant_colony::pheromone_reader::PheromoneReader;
use crate::common::cli_output::CliOutput;
use crate::common::utils::{compare_float, measure, produce_until};

pub use summary::{CycleSummary, EpochSummary};

pub struct ColonyRunner<C, IO>
where
    C: Colony,
    IO: CliOutput,
{
    colony: C,
    graph: Rc<Graph>,
    io: Rc<IO>,
    cycle_history: Vec<CycleSummary>,
    epoch_history: Vec<EpochSummary>,
}

impl<C, IO> ColonyRunner<C, IO>
where
    C: Colony,
    IO: CliOutput,
{
    pub fn new(colony: C, graph: Rc<Graph>, io: Rc<IO>) -> Self {
        ColonyRunner {
            colony,
            graph,
            io,
            cycle_history: Vec::new(),
            epoch_history: Vec::new(),
        }
    }

    pub fn train(self, n_epochs: usize, n_cycles: usize) -> Self {
        (0..n_epochs).fold(self, |runner, _| runner.train_epoch(n_cycles))
    }

    pub fn get_colony(self) -> C {
        self.colony
    }

    pub fn last_cycle_summary(&self) -> Option<CycleSummary> {
        self.cycle_history.last().cloned()
    }

    pub fn last_epoch_summary(&self) -> Option<EpochSummary> {
        self.epoch_history.last().cloned()
    }

    pub fn last_summaries(&self) -> Option<(CycleSummary, EpochSummary)> {
        self.last_cycle_summary().zip(self.last_epoch_summary())
    }

    pub fn train_n_until_no_improvement(self, n_until: usize) -> Self {
        let ColonyRunner {
            colony: init_colony,
            graph,
            io,
            mut cycle_history,
            mut epoch_history,
        } = self;

        let (colony, next_cycle_history) = produce_until(
            (init_colony, Vec::new()),
            |(colony, history), idx| Self::train_cycle(colony, io.as_ref(), history, idx),
            |(_, history), _| Self::had_no_improvement_in_n_last_steps(history, n_until),
        );

        let (shortest_route, shortest_route_cycle_idx) =
            Self::shortest_route_from_cycle_history(&next_cycle_history);

        let epoch_summary = EpochSummary {
            shortest_route,
            shortest_route_cycle_idx,
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

    fn train_epoch(self, n_cycles: usize) -> Self {
        let ColonyRunner {
            colony: init_colony,
            graph,
            io,
            mut cycle_history,
            mut epoch_history,
        } = self;

        let (colony, next_cycle_history) = (0..n_cycles).fold(
            (init_colony, Vec::with_capacity(n_cycles)),
            |(colony, summaries), cycle_idx| {
                ColonyRunner::train_cycle(colony, io.as_ref(), summaries, cycle_idx)
            },
        );

        let (shortest_route, shortest_route_cycle_idx) =
            Self::shortest_route_from_cycle_history(&next_cycle_history);

        let epoch_summary = EpochSummary {
            shortest_route,
            shortest_route_cycle_idx,
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

    fn train_cycle(
        colony: C,
        io: &IO,
        mut summaries: Vec<CycleSummary>,
        cycle_idx: usize,
    ) -> (C, Vec<CycleSummary>) {
        let (new_colony, exec_time_ms) = measure(|| colony.execute_cycle(cycle_idx));

        let pheromone = new_colony.get_pheromone();
        let routes = new_colony.get_routes();
        let shortest_route = routes.get_shortest_route();
        let shortest_dist = shortest_route.clone().map(|r| r.get_distance());
        let shortest_path_length = shortest_route.clone().map(|r| r.get_length());

        let summary = CycleSummary {
            cycle_idx,
            exec_time_ms,
            shortest_dist,
            shortest_path_length,
            avg_dist: routes.get_average_route_distance(),
            ratio_of_incomplete_routes: routes.get_ratio_of_incomplete_routes(),
            n_non_empty_edges: PheromoneReader::count_edges_with_pheromone_above(pheromone, 0.1),
            pheromone_variance: pheromone.calc_variance(),
            shortest_route,
        };

        io.print(&summary);
        summaries.push(summary);

        (new_colony, summaries)
    }

    fn had_no_improvement_in_n_last_steps(
        history: &[CycleSummary],
        n_no_improvement: usize,
    ) -> bool {
        // n_no_improvement + 1, we need additional one to compare against
        let tail_length = n_no_improvement + 1;

        if history.len() < tail_length {
            return false;
        }

        let history_tail = history
            .iter()
            .rev()
            .take(tail_length)
            .rev()
            .collect::<Vec<_>>();

        let should_stop =
            history_tail
                .split_first()
                .map_or(false, |(reference, latest_n_entries)| {
                    let none_improved = latest_n_entries
                        .iter()
                        .all(|latest| latest.shortest_dist >= reference.shortest_dist);

                    none_improved
                });

        should_stop
    }

    fn shortest_route_from_cycle_history(
        cycle_history: &[CycleSummary],
    ) -> (Option<Route>, Option<usize>) {
        let shortest_entry = cycle_history
            .iter()
            .filter_map(|cycle| {
                cycle
                    .shortest_route
                    .as_ref()
                    .map(|route| (route.clone(), cycle.cycle_idx))
            })
            .min_by(|(route_a, _), (route_b, _)| {
                let dist_a = route_a.get_distance();
                let dist_b = route_b.get_distance();

                compare_float(&dist_a, &dist_b)
            });

        (
            shortest_entry.clone().map(|(route, _)| route),
            shortest_entry.map(|(_, cycle_idx)| cycle_idx),
        )
    }
}
