use rand::rngs::ThreadRng;
use rand::Rng;

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::ant::Ant;
use super::graph::{AdjacencyListEntry, Graph};
use super::pheromone::{Pheromone, PheromoneUpdater};

pub struct Config<U: PheromoneUpdater> {
    pub ant_count: u32,
    pub num_of_steps_per_cycle: u32,
    pub random: ThreadRng,
    pub pheromone_updater: U,
}

pub struct Colony<U: PheromoneUpdater> {
    ants: Vec<Ant>,
    graph: Graph,
    pheromone: Pheromone,
    config: Config<U>,
}

impl<U: PheromoneUpdater> Colony<U> {
    pub fn new(config: Config<U>, graph: Graph) -> Self {
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

        let node_ids = graph.get_node_ids();
        let ants = (0..config.ant_count)
            .map(|_| {
                let idx = config.random.gen_range(0..node_ids.len());
                let initial_node = node_ids[idx];

                Ant::new(initial_node)
            })
            .collect();

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
                Colony::<U>::execute_step_for_single_ant(
                    ant,
                    &graph,
                    &pheromone,
                    &mut config.random,
                )
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

    fn execute_step_for_single_ant<'a>(
        ant: Ant,
        graph: &'a Graph,
        pheromone: &Pheromone,
        random: &mut ThreadRng,
    ) -> (Ant, &'a AdjacencyListEntry) {
        let possible_next_edges: Vec<&AdjacencyListEntry> = graph
            .get_adjacent_edges(&ant.current_node)
            .iter()
            .filter(|edge| !ant.has_visited(&edge.to))
            .collect();

        // node_likelihood is not normalized - does not sum up to one
        let node_likelihood = possible_next_edges.iter().map(|edge| {
            let visibility = 1.0 / edge.distance;
            let pheromone_level = pheromone.get_pheromone_for_edge(edge.from, edge.to);

            visibility * pheromone_level
        });

        let dist = WeightedIndex::new(node_likelihood).unwrap();

        let next_edge = possible_next_edges[dist.sample(random)];
        let next_node = next_edge.to;

        (ant.move_to_node(next_node), next_edge)
    }
}
