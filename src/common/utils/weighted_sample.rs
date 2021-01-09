/// Picks random sample from given array
/// in order to be pure, accepts random_seed argument
/// which is expected to in range [0.0, 1.0)
/// returns None for empty data array or random_seed above or equal to 1.0
///
/// Based on LinearScan algorithm from https://blog.bruce-hill.com/a-faster-weighted-random-choice
/// could utilize faster Hopscotch Selection, but it requires more costly preparation
/// which for ant colony problems is not suitable (weights are dynamic and not reusable)
#[cfg_attr(feature = "profiler", flame)]
pub fn weighted_sample<T: Copy>(data: &[T], weights: &[f32], random_seed: f32) -> Option<T> {
    let weight_sum: f32 = weights.iter().sum();
    let guess = weight_sum * random_seed;

    let cumulative_weigts = weights.iter().scan(0.0, |state, weight| {
        *state += weight;
        Option::Some(*state)
    });

    data.iter()
        .zip(cumulative_weigts)
        .find_map(|(item, weight_sum)| iif!(guess <= weight_sum, Option::Some(*item), Option::None))
}
