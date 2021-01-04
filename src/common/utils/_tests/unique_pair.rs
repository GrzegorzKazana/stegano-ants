#[cfg(test)]
mod common_utils_tests {
    use super::super::super::UniquePair;
    use proptest::prelude::*;

    #[test]
    fn it_does_calulate_edge_key() {
        let result = UniquePair::generate_key(2, 1);

        assert_eq!(result, 5);
    }

    #[test]
    fn it_does_decode_edge_key() {
        let (from, to) = (2, 1);
        let key = UniquePair::generate_key(from, to);
        let result = UniquePair::decode_key(key);

        assert!((from, to) == result || (to, from) == result);
    }

    proptest! {
        #[test]
        fn it_does_not_care_for_node_order_when_generating_key(a: u32, b: u32) {
            let key_a = UniquePair::generate_key(a, b);
            let key_b = UniquePair::generate_key(b, a);

            assert_eq!(key_a, key_b);
        }

        #[test]
        fn it_does_not_have_key_collisions(a: u32, b: u32, c: u32, d: u32) {
            let key_a = UniquePair::generate_key(a, b);
            let key_b = UniquePair::generate_key(c, d);

            let keys_are_same = key_a == key_b;
            let inputs_are_same = (a == c && b == d) || (a == d && b == c);

            assert!(inputs_are_same || !keys_are_same);
        }

        #[test]
        fn it_correctly_decodes_generated_key(a: u32, b: u32) {
            let key = UniquePair::generate_key(a, b);
            let result = UniquePair::decode_key(key);

            assert!((a, b) == result || (b, a) == result);
        }

        #[test]
        fn it_decodes_generated_key_in_smaller_first_order(a: u32, b: u32) {
            let key = UniquePair::generate_key(a, b);
            let result = UniquePair::decode_key(key);
            let expected = (std::cmp::min(a, b), std::cmp::max(a, b));

            assert!(expected == result);
        }
    }
}
