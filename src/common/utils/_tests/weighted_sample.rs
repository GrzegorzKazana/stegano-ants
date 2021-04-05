#[cfg(test)]
mod common_utils_weighted_sample_tests {
    use super::super::super::weighted_sample;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn it_will_always_pick_item_if_other_have_zero_weight(
        random_seed in 0.0f32..1.0,
        non_zero_idx in 0usize..10,
        data: [char; 10],
        weight_seed: f32,
      ) {
          let weight = weight_seed.abs() + stability_factor!();
          let mut weights = data.iter().map(|_| 0.0).collect::<Vec<_>>();
          weights[non_zero_idx] = weight;

          let expected = data[non_zero_idx];
          let result = weighted_sample(&data, &weights, random_seed).unwrap();

          assert_eq!(expected, result);
      }
    }

    proptest! {
        #[test]
        fn it_should_pick_same_item_with_scaled_props(
          random_seed in 0.0f32..1.0,
          data: [char; 10],
          weights: [f32; 10],
          scale in 0.0f32..10.0,
        ) {
            let max_safe_weight = (f32::MAX / scale - 1.0) / 10.0;
            let weights = weights.iter().map(|w| w.abs().min(max_safe_weight)).collect::<Vec<_>>();
            let weights_doubled = weights.iter().cloned().map(|w| w * scale).collect::<Vec<_>>();

            println!("{:?}", weights);

            assert_eq!(
                weighted_sample(&data, &weights, random_seed).unwrap(),
                weighted_sample(&data, &weights_doubled, random_seed).unwrap()
            )
        }
    }
}
