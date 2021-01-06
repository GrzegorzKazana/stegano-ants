use rand::{distributions::Uniform, Rng};
use rayon::prelude::*;
use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::ant_dispatcher::AntDispatcher;
use crate::ant_colony::graph::{Graph, RouteBatch, RouteCollection};
use crate::ant_colony::pheromone::Pheromone;
use crate::ant_colony::pheromone_updater::PheromoneUpdater;

use super::{Colony, Config, ConfigurableColony};

pub struct StepwiseParallelColony<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> {
    ants: Vec<Ant>,
    graph: &'a Graph,
    pheromone: Pheromone,
    routes: RouteCollection<'a>,
    config: Config<U, D, R>,
}

impl<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> Colony
    for StepwiseParallelColony<'a, U, D, R>
{
    fn execute_n_cycles(self, n_cycles: usize) -> Self {
        (0..n_cycles).fold(self, StepwiseParallelColony::execute_cycle)
    }

    #[cfg_attr(feature = "profiler", flame)]
    fn execute_cycle(self, _cycle: usize) -> Self {
        let steps = 0..self.config.num_of_steps_per_cycle;
        let init_colony = self.initialize_ants().initialize_routes();

        let colony = steps.fold(
            init_colony,
            StepwiseParallelColony::execute_step_for_all_ants,
        );

        let pheromone = colony
            .config
            .pheromone_updater
            .on_after_cycle(colony.pheromone, &colony.routes);

        StepwiseParallelColony {
            pheromone,
            ..colony
        }
    }

    fn get_pheromone(&self) -> &Pheromone {
        &self.pheromone
    }

    fn get_routes(&self) -> &'a RouteCollection {
        &self.routes
    }

    fn get_ants(&self) -> &Vec<Ant> {
        &self.ants
    }
}

impl<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> ConfigurableColony<'a, U, D, R>
    for StepwiseParallelColony<'a, U, D, R>
{
    fn new(config: Config<U, D, R>, graph: &'a Graph) -> Self {
        StepwiseParallelColony {
            graph,
            config,
            routes: RouteCollection::default(),
            ants: Vec::new(),
            pheromone: Pheromone::new(),
        }
        .initialize_pheromone()
    }
}

impl<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> StepwiseParallelColony<'a, U, D, R> {
    #[cfg_attr(feature = "profiler", flame)]
    fn execute_step_for_all_ants(self, _step: usize) -> Self {
        let StepwiseParallelColony {
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

        StepwiseParallelColony {
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

        StepwiseParallelColony { routes, ..self }
    }

    fn initialize_ants(self) -> Self {
        let mut config = self.config;

        let ants = config.ant_dispatcher.place_ants_on_graph(
            config.ant_count,
            &self.graph,
            &mut config.rng,
        );

        StepwiseParallelColony {
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

        StepwiseParallelColony { pheromone, ..self }
    }
}

impl<'a, U: PheromoneUpdater, D: AntDispatcher, R: Rng> Display
    for StepwiseParallelColony<'a, U, D, R>
{
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
