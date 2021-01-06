mod _tests;

use rand::{distributions::Uniform, Rng};
use rayon::prelude::*;
use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::ant_dispatcher::AntDispatcher;
use crate::ant_colony::graph::{Graph, RouteBatch, RouteCollection};
use crate::ant_colony::pheromone::Pheromone;
use crate::ant_colony::pheromone_updater::PheromoneUpdater;

pub struct Config<U: PheromoneUpdater, D: AntDispatcher, R: Rng> {
    pub ant_count: usize,
    pub num_of_steps_per_cycle: usize,
    pub pheromone_updater: U,
    pub ant_dispatcher: D,
    pub rng: R,
}

pub trait ColonyTrait {
    fn execute_n_cycles(self, n_cycles: usize) -> Self;

    fn execute_cycle(self, _cycle: usize) -> Self;

    fn get_progress(&self) -> (&Pheromone, &RouteCollection);
}

pub struct Colony<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> {
    ants: Vec<Ant>,
    graph: &'a Graph,
    pheromone: Pheromone,
    routes: RouteCollection<'a>,
    config: Config<U, D, R>,
}

impl<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> ColonyTrait for Colony<'a, U, D, R> {
    fn execute_n_cycles(self, n_cycles: usize) -> Self {
        (0..n_cycles).fold(self, Colony::execute_cycle)
    }

    #[cfg_attr(feature = "profiler", flame)]
    fn execute_cycle(self, _cycle: usize) -> Self {
        let steps = 0..self.config.num_of_steps_per_cycle;
        let init_colony = self.initialize_ants().initialize_routes();

        let colony = steps.fold(init_colony, Colony::execute_step_for_all_ants);

        let pheromone = colony
            .config
            .pheromone_updater
            .on_after_cycle(colony.pheromone, &colony.routes);

        Colony {
            pheromone,
            ..colony
        }
    }

    fn get_progress(&self) -> (&Pheromone, &RouteCollection) {
        (&self.pheromone, &self.routes)
    }
}

impl<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> Colony<'a, U, D, R> {
    pub fn new(config: Config<U, D, R>, graph: &'a Graph) -> Self {
        Colony {
            graph,
            config,
            routes: RouteCollection::default(),
            ants: Vec::new(),
            pheromone: Pheromone::new(),
        }
        .initialize_pheromone()
    }

    #[cfg_attr(feature = "profiler", flame)]
    fn execute_step_for_all_ants(self, _step: usize) -> Self {
        let Colony {
            ants: init_ants,
            graph,
            pheromone: init_pheromone,
            routes: init_routes,
            config,
            ..
        } = self;

        let Config {
            pheromone_updater,
            ant_dispatcher,
            mut rng,
            ..
        } = config;

        let seeds = (&mut rng).sample_iter(Uniform::new::<f32, f32>(0.0, 1.0));

        let (ants, taken_edges): (Vec<Ant>, RouteBatch) = init_ants
            .into_iter()
            .zip(seeds)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|(ant, seed)| {
                let next_edge =
                    ant_dispatcher.select_next_edge(&ant, &graph, &init_pheromone, seed);

                (ant.move_to_node(next_edge.to), next_edge)
            })
            .unzip();

        let pheromone = pheromone_updater.on_after_step(init_pheromone, &taken_edges);

        let routes = init_routes.add_steps(&taken_edges);

        Colony {
            ants,
            routes,
            pheromone,
            config: Config {
                ant_dispatcher,
                rng,
                pheromone_updater,
                ..config
            },
            ..self
        }
    }

    fn initialize_routes(self) -> Self {
        let routes =
            RouteCollection::new(self.config.ant_count, self.config.num_of_steps_per_cycle);

        Colony { routes, ..self }
    }

    fn initialize_ants(self) -> Self {
        let mut config = self.config;

        let ants = config.ant_dispatcher.place_ants_on_graph(
            config.ant_count,
            &self.graph,
            &mut config.rng,
        );

        Colony {
            ants,
            config,
            ..self
        }
    }

    fn initialize_pheromone(self) -> Self {
        let edges = self.graph.get_all_edges();

        let pheromone = self
            .config
            .pheromone_updater
            .initialize(self.pheromone, &edges);

        Colony { pheromone, ..self }
    }
}

impl<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> Display for Colony<'a, U, D, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "----------\n\
            Ant colony\n\t\
            {} ants\n\
            {}\n\
            {}\n\
            {}\n\
            {}\n\
            ----------\n",
            self.config.ant_count,
            self.config.pheromone_updater,
            self.config.ant_dispatcher,
            self.graph,
            self.pheromone
        )
    }
}
