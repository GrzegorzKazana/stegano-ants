#[cfg(test)]
mod pheromone_tests {
    use std::collections::HashMap;

    use crate::ant_colony::pheromone::Pheromone;

    #[test]
    fn it_allows_for_initializing_pheromone_trace() {
        let edge_key = 2;
        let initial_value = 1.0;

        let pheromone = Pheromone::new().initialize_pheromone_for_edge(edge_key, initial_value);

        assert_eq!(pheromone.get_pheromone_for_edge(edge_key), initial_value);
    }

    #[test]
    fn it_allows_for_increasing_pheromone_trace_of_specific_edge() {
        let edge_a = 1;
        let edge_b = 2;
        let initial_value = 1.0;
        let increment = 0.5;

        let pheromone = Pheromone::new()
            .initialize_pheromone_for_edge(edge_a, initial_value)
            .initialize_pheromone_for_edge(edge_b, initial_value)
            .increase_pheromone_value(edge_a, increment);

        assert_eq!(
            pheromone.get_pheromone_for_edge(edge_a),
            initial_value + increment
        );
        assert_eq!(pheromone.get_pheromone_for_edge(edge_b), initial_value);
    }

    #[test]
    fn it_allows_for_scaling_pheromone_level_for_all_edges() {
        let edge_a = 1;
        let edge_b = 2;
        let initial_value = 1.0;
        let scaler = 0.5;

        let pheromone = Pheromone::new()
            .initialize_pheromone_for_edge(edge_a, initial_value)
            .initialize_pheromone_for_edge(edge_b, initial_value)
            .scale_all_pheromone_values(scaler);

        assert_eq!(
            pheromone.get_pheromone_for_edge(edge_a),
            initial_value * scaler
        );
        assert_eq!(
            pheromone.get_pheromone_for_edge(edge_b),
            initial_value * scaler
        );
    }

    #[test]
    fn it_allows_for_normalization_to_max_of_1() {
        let pheromone = Pheromone::new()
            .initialize_pheromone_for_edge(0, 0.5)
            .initialize_pheromone_for_edge(1, 0.2)
            .initialize_pheromone_for_edge(2, 1.5);

        let expected: HashMap<_, _> = vec![(0, 5.0 / 15.0), (1, 2.0 / 15.0), (2, 15.0 / 15.0)]
            .into_iter()
            .collect();

        let result = pheromone.get_values_normalized();

        assert_delta!(result.get(&0).unwrap(), expected.get(&0).unwrap());
        assert_delta!(result.get(&1).unwrap(), expected.get(&1).unwrap());
        assert_delta!(result.get(&2).unwrap(), expected.get(&2).unwrap());
    }

    #[test]
    fn it_allows_for_normalization_to_sum() {
        let pheromone = Pheromone::new()
            .initialize_pheromone_for_edge(0, 0.5)
            .initialize_pheromone_for_edge(1, 0.2)
            .initialize_pheromone_for_edge(2, 1.5);

        let result = pheromone.get_values_normalized_to_sum();
        let result_sum: f32 = result.values().sum();

        assert_delta!(result_sum, 1.0);
    }
}
