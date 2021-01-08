#[cfg(test)]
mod ant_dispatcher_tests {
    use crate::ant_colony::graph::AdjacencyListEntry;
    use crate::ant_colony::pheromone::Pheromone;

    use super::super::{BasicAntDispatcher, BiasedAntDispatcher, LikelihoodAntDispatcher};

    fn get_pheromone() -> Pheromone {
        Pheromone::from_values(map!(
            0 => 0.5,
            1 => 1.0,
            2 => 0.25
        ))
    }

    fn get_possible_edges() -> Vec<AdjacencyListEntry> {
        vec![
            AdjacencyListEntry {
                key: 0,
                from: 0,
                to: 0,
                distance: 1.0,
            },
            AdjacencyListEntry {
                key: 1,
                from: 0,
                to: 0,
                distance: 2.0,
            },
            AdjacencyListEntry {
                key: 2,
                from: 0,
                to: 0,
                distance: 5.0,
            },
        ]
    }

    fn test_likelihood_dispatcher<D: LikelihoodAntDispatcher>(dispatcher: D) -> Vec<f32> {
        let edges = get_possible_edges();
        let pheromone = get_pheromone();

        dispatcher.cacluclate_node_likelihoods(&vec![&edges[0], &edges[1], &edges[2]], &pheromone)
    }

    #[test]
    fn basic_calcs_correctly_likelihoods() {
        let dispatcher = BasicAntDispatcher;
        let result = test_likelihood_dispatcher(dispatcher);
        let expected = vec![
            0.5,  // 0.5 / 1.0
            0.5,  // 1.0 / 2.0
            0.05, // 0.25 / 5.0
        ];

        assert_vec_delta!(result, expected, 1e-4);
    }

    #[test]
    fn biased_calcs_correctly_likelihoods() {
        let dispatcher = BiasedAntDispatcher::new(2.0, 0.5);
        let result = test_likelihood_dispatcher(dispatcher);
        let expected: Vec<f32> = vec![
            0.5f32.powf(2.0),                     // 0.5^2.0 / 1.0^0.5
            1.0 / 2.0f32.powf(0.5),               // 1.0^2.0 / 2.0^0.5
            0.25f32.powf(2.0) / 5.0f32.powf(0.5), // 0.25^2.0 / 5.0^0.5
        ];

        assert_vec_delta!(result, expected, 1e-4);
    }
}
