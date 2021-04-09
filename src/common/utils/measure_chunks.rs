pub type MeasuredChunk = (usize, usize, usize);

pub fn measure_chunks(max_value: usize, chunk_size: usize) -> Vec<MeasuredChunk> {
    (0..max_value)
        .take_while(|n| n + chunk_size <= max_value)
        .filter(|n| n % chunk_size == 0)
        .map(|from| {
            let to = iif!(
                from + 2 * chunk_size > max_value,
                max_value - 1,
                from + chunk_size - 1
            );
            let len = to - from + 1;

            (from, len, to)
        })
        .collect()
}
