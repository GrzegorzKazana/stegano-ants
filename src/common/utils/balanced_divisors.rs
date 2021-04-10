pub fn balanced_divisors(n: usize) -> (usize, usize) {
    let max = (n as f32).sqrt() as usize;

    (1..=max)
        .rev()
        .filter(|div| n % div == 0)
        .map(|div| (div, n / div))
        .next()
        .unwrap_or((1, n))
}
