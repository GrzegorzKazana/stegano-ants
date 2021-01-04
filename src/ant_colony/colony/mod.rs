use std::fmt::Display;

use super::ant::{Ant, AntDispatcher};
use super::graph::Graph;
use super::pheromone::{Pheromone, PheromoneUpdater};

pub struct Config<U: PheromoneUpdater, D: AntDispatcher> {
    pub ant_count: u32,
    pub num_of_steps_per_cycle: u32,
    pub pheromone_updater: U,
    pub ant_dispatcher: D,
}

pub struct Colony<U: PheromoneUpdater, D: AntDispatcher> {
    ants: Vec<Ant>,
    graph: Graph,
    pheromone: Pheromone,
    config: Config<U, D>,
}

impl<U: PheromoneUpdater, D: AntDispatcher> Colony<U, D> {
    pub fn new(config: Config<U, D>, graph: Graph) -> Self {
        Colony {
            graph,
            config,
            ants: Vec::new(),
            pheromone: Pheromone::new(),
        }
    }

    pub fn initialize_ants(self) -> Self {
        let Colony {
            graph, mut config, ..
        } = self;

        let ants = config
            .ant_dispatcher
            .place_ants_on_graph(config.ant_count, &graph);

        Colony {
            ants,
            graph,
            config,
            ..self
        }
    }

    pub fn initialize_pheromone(self) -> Self {
        let edges = self.graph.get_all_edges();
        let pheromone = self
            .config
            .pheromone_updater
            .initialize(self.pheromone, edges);

        Colony { pheromone, ..self }
    }

    pub fn execute_n_cycles(self, n_cycles: u32) -> Self {
        let initialized_colony = self.initialize_pheromone();
        let cycles = 0..n_cycles;

        cycles.fold(initialized_colony, Colony::execute_cycle)
    }

    pub fn execute_cycle(self, _cycle: u32) -> Self {
        let initialized_colony = self.initialize_ants();
        let steps = 0..initialized_colony.config.num_of_steps_per_cycle;

        let Colony {
            ants,
            config,
            graph,
            pheromone,
        } = steps.fold(initialized_colony, Colony::execute_step_for_all_ants);

        let new_pheromone = config
            .pheromone_updater
            .on_after_cycle(pheromone, &ants, &graph);

        Colony {
            ants,
            config,
            graph,
            pheromone: new_pheromone,
        }
    }

    pub fn execute_step_for_all_ants(self, _step: u32) -> Self {
        let Colony {
            ants,
            graph,
            pheromone,
            mut config,
        } = self;

        let (new_ants, taken_edges) = ants
            .into_iter()
            .map(|ant| {
                let next_edge = config
                    .ant_dispatcher
                    .select_next_edge(&ant, &graph, &pheromone);

                (ant.move_to_node(next_edge.to), next_edge)
            })
            .unzip();

        let new_pheromone = config
            .pheromone_updater
            .on_after_step(pheromone, taken_edges);

        Colony {
            ants: new_ants,
            graph,
            pheromone: new_pheromone,
            config,
        }
    }
}

impl<U: PheromoneUpdater, D: AntDispatcher> Display for Colony<U, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "----------\n\
            Ant colony\n\t\
            {} ants\n\
            {}\n\
            {}\n\
            {}\n\
            ----------\n",
            self.config.ant_count, self.config.pheromone_updater, self.graph, self.pheromone
        )
    }
}
