#[cfg(test)]
mod colony_tests {
    use rand::{prelude::StdRng, SeedableRng};
    use std::rc::Rc;

    use super::super::{Colony, Config, ConfigurableColony, StepwiseParallelColony};
    use crate::ant_colony::ant_dispatcher::BasicAntDispatcher;
    use crate::ant_colony::graph::Graph;
    use crate::ant_colony::pheromone_updater::ConstantPheromoneUpdater;

    fn get_sample_colony(
        seed: u64,
        ant_count: usize,
        num_of_steps_per_cycle: usize,
        graph: Rc<Graph>,
    ) -> StepwiseParallelColony<ConstantPheromoneUpdater, BasicAntDispatcher, StdRng> {
        let config = Config {
            ant_count,
            num_of_steps_per_cycle,
            pheromone_updater: ConstantPheromoneUpdater::new(1.0, 0.1, 0.1),
            ant_dispatcher: BasicAntDispatcher,
            rng: StdRng::seed_from_u64(seed),
        };

        StepwiseParallelColony::new(config, graph)
    }

    #[test]
    fn it_is_reproducible() {
        let graph = Graph::random_tsp_graph(&mut StdRng::seed_from_u64(42), 10);
        let graph = Rc::new(graph);

        let colony_a = get_sample_colony(42, 20, 5, Rc::clone(&graph)).execute_n_cycles(2);
        let colony_b = get_sample_colony(42, 20, 5, Rc::clone(&graph)).execute_n_cycles(2);

        assert_eq!(colony_a.get_pheromone(), colony_b.get_pheromone());
        assert_eq!(
            colony_a.get_routes().get_routes(),
            colony_b.get_routes().get_routes()
        );
    }

    #[test]
    fn it_generates_routes_for_each_ant() {
        let graph = Graph::random_tsp_graph(&mut StdRng::seed_from_u64(42), 10);
        let graph = Rc::new(graph);

        let colony = get_sample_colony(42, 20, 5, Rc::clone(&graph)).execute_n_cycles(2);

        assert_eq!(colony.get_routes().get_routes().len(), 20);
        assert!(colony
            .get_routes()
            .get_routes()
            .iter()
            .all(|route| route.get_length() == 5))
    }

    #[test]
    fn it_generates_routes_that_match_ant_visited_set() {
        let graph = Graph::random_tsp_graph(&mut StdRng::seed_from_u64(42), 10);
        let graph = Rc::new(graph);

        let colony = get_sample_colony(42, 20, 5, Rc::clone(&graph)).execute_n_cycles(2);

        let ants = colony.get_ants();
        let routes = colony.get_routes();

        assert_eq!(ants.len(), routes.get_routes().len());
        ants.iter()
            .zip(routes.get_routes())
            .for_each(|(ant, route)| {
                let ant_node_ids = ant.get_visited();
                let route_node_ids = route.get_nodes();

                assert_eq!(ant_node_ids.len(), route_node_ids.len());
                assert!(route_node_ids
                    .iter()
                    .all(|node_id| ant_node_ids.contains(node_id)));
            })
    }
}
