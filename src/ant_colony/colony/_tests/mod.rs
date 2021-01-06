#[cfg(test)]
mod colony_tests {
    use rand::{prelude::StdRng, SeedableRng};

    use super::super::{Colony, ColonyTrait, Config};
    use crate::ant_colony::ant_dispatcher::BasicAntDispatcher;
    use crate::ant_colony::graph::Graph;
    use crate::ant_colony::pheromone_updater::ConstantPheromoneUpdater;

    fn get_sample_colony<'a>(
        seed: u64,
        ant_count: usize,
        num_of_steps_per_cycle: usize,
        graph: &'a Graph,
    ) -> Colony<'a, ConstantPheromoneUpdater, BasicAntDispatcher, StdRng> {
        let config = Config {
            ant_count,
            num_of_steps_per_cycle,
            pheromone_updater: ConstantPheromoneUpdater::new(1.0, 0.1),
            ant_dispatcher: BasicAntDispatcher,
            rng: StdRng::seed_from_u64(seed),
        };

        Colony::new(config, &graph)
    }

    #[test]
    fn it_is_reproducible() {
        let graph = Graph::random_tsp_graph(&mut StdRng::seed_from_u64(42), 10);

        let colony_a = get_sample_colony(42, 20, 5, &graph).execute_n_cycles(2);
        let colony_b = get_sample_colony(42, 20, 5, &graph).execute_n_cycles(2);

        assert_eq!(colony_a.pheromone, colony_b.pheromone);
        assert_eq!(colony_a.routes.get_routes(), colony_b.routes.get_routes());
    }

    #[test]
    fn it_generates_routes_for_each_ant() {
        let graph = Graph::random_tsp_graph(&mut StdRng::seed_from_u64(42), 10);

        let colony = get_sample_colony(42, 20, 5, &graph).execute_n_cycles(2);

        assert_eq!(colony.routes.get_routes().len(), 20);
        assert!(colony
            .routes
            .get_routes()
            .iter()
            .all(|route| route.get_length() == 5))
    }

    #[test]
    fn it_generates_routes_that_match_ant_visited_set() {
        let graph = Graph::random_tsp_graph(&mut StdRng::seed_from_u64(42), 10);

        let colony = get_sample_colony(42, 20, 5, &graph).execute_n_cycles(2);

        let Colony { ants, routes, .. } = colony;

        assert_eq!(ants.len(), routes.get_routes().len());
        ants.iter()
            .zip(routes.get_routes())
            .for_each(|(ant, route)| {
                let ant_node_ids = ant.get_visited();
                let route_nodes = route.get_nodes();
                // skipping 1, since route also keeps track of initial node
                // whereas, ant only stores it has transitioned to
                let route_node_ids = route_nodes.iter().skip(1).collect::<Vec<_>>();

                assert_eq!(ant_node_ids.len(), route_node_ids.len());
                assert!(route_node_ids
                    .iter()
                    .all(|node_id| ant_node_ids.contains(node_id)));
            })
    }
}