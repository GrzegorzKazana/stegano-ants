#[cfg(test)]
mod pheromone_tests {
    use crate::ant_colony::graph::NodeId;
    use crate::ant_colony::pheromone::Pheromone;
    use proptest::prelude::*;

    #[test]
    fn it_does_calulate_edge_key() {
        let result = Pheromone::generate_edge_key(2, 1);

        assert_eq!(result, 5);
    }

    #[test]
    fn it_does_decode_edge_key() {
        let (from, to) = (2, 1);
        let key = Pheromone::generate_edge_key(from, to);
        let result = Pheromone::decode_edge_key(key);

        assert!((from, to) == result || (to, from) == result);
    }

    #[test]
    fn it_allows_for_initializing_pheromone_trace() {
        let from = 2;
        let to = 1;
        let initial_value = 1.0;

        let pheromone = Pheromone::new().initialize_pheromone_for_edge(from, to, initial_value);

        assert_eq!(pheromone.get_pheromone_for_edge(from, to), initial_value);
    }

    #[test]
    fn it_allows_for_increasing_pheromone_trace_of_specific_edge() {
        let edge_a = (2, 1);
        let edge_b = (3, 1);
        let initial_value = 1.0;
        let increment = 0.5;

        let pheromone = Pheromone::new()
            .initialize_pheromone_for_edge(edge_a.0, edge_a.1, initial_value)
            .initialize_pheromone_for_edge(edge_b.0, edge_b.1, initial_value)
            .increase_pheromone_value(edge_a.0, edge_a.1, increment);

        assert_eq!(
            pheromone.get_pheromone_for_edge(edge_a.0, edge_a.1),
            initial_value + increment
        );
        assert_eq!(
            pheromone.get_pheromone_for_edge(edge_b.0, edge_b.1),
            initial_value
        );
    }

    #[test]
    fn it_allows_for_scaling_pheromone_level_for_all_edges() {
        let edge_a = (2, 1);
        let edge_b = (3, 1);
        let initial_value = 1.0;
        let scaler = 0.5;

        let pheromone = Pheromone::new()
            .initialize_pheromone_for_edge(edge_a.0, edge_a.1, initial_value)
            .initialize_pheromone_for_edge(edge_b.0, edge_b.1, initial_value)
            .scale_all_pheromone_values(scaler);

        assert_eq!(
            pheromone.get_pheromone_for_edge(edge_a.0, edge_a.1),
            initial_value * scaler
        );
        assert_eq!(
            pheromone.get_pheromone_for_edge(edge_b.0, edge_b.1),
            initial_value * scaler
        );
    }

    proptest! {
        #[test]
        fn it_does_not_care_for_node_order_when_generating_key(a: NodeId, b: NodeId) {
            let key_a = Pheromone::generate_edge_key(a, b);
            let key_b = Pheromone::generate_edge_key(b, a);

            assert_eq!(key_a, key_b);
        }

        #[test]
        fn it_does_not_have_key_collisions(a: NodeId, b: NodeId, c: NodeId, d: NodeId) {
            let key_a = Pheromone::generate_edge_key(a, b);
            let key_b = Pheromone::generate_edge_key(c, d);

            let keys_are_same = key_a == key_b;
            let inputs_are_same = (a == c && b == d) || (a == d && b == c);

            assert!(inputs_are_same || !keys_are_same);
        }

        #[test]
        fn it_correctly_decodes_generated_key(a: NodeId, b: NodeId) {
            let key = Pheromone::generate_edge_key(a, b);
            let result = Pheromone::decode_edge_key(key);

            assert!((a, b) == result || (b, a) == result);
        }

        #[test]
        fn it_decodes_generated_key_in_smaller_first_order(a: NodeId, b: NodeId) {
            let key = Pheromone::generate_edge_key(a, b);
            let result = Pheromone::decode_edge_key(key);
            let expected = (std::cmp::min(a, b), std::cmp::max(a, b));

            assert!(expected == result);
        }
    }
}
