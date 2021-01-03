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

    proptest! {
        #[test]
        fn it_does_care_for_node_order_when_generating_key(a: NodeId, b: NodeId) {
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
    }
}
