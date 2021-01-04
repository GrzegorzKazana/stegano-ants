#[cfg(test)]
mod pheromone_tests {
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
}
