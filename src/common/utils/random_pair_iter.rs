use rand::distributions::Distribution;
use rand::Rng;

pub fn random_pair_iter<'a, T, R: Rng, D: 'a + Distribution<T> + Copy>(
    rng: &'a mut R,
    distribution: D,
) -> impl Iterator<Item = (T, T)> + 'a {
    std::iter::from_fn(move || {
        let seed_a = rng.sample(distribution);
        let seed_b = rng.sample(distribution);

        Option::Some((seed_a, seed_b))
    })
}
