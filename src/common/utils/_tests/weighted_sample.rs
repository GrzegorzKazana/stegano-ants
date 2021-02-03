#[cfg(test)]
mod common_utils_weighted_sample_tests {
    use super::super::super::weighted_sample;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn it_will_always_pick_item_if_other_have_zero_weight(random_seed in 0.0f32..1.0, non_zero_idx in 0usize..5, weight_seed: f32) {
            let data = ['a', 'b', 'c', 'd', 'e'].iter().collect::<Vec<_>>();

            let weight = weight_seed.abs() + stability_factor!();
            let mut weights= [0.0, 0.0, 0.0, 0.0, 0.0];
            weights[non_zero_idx] = weight;

            let expected = data[non_zero_idx];
            let result = weighted_sample(&data, &weights, random_seed).unwrap();

            assert_eq!(expected, result);
        }
    }
}
