use itertools::Itertools;

use std::ops::Range;

pub fn measure_chunks(range: Range<usize>, chunk_size: usize) -> Vec<(usize, usize, usize)> {
    range
        .chunks(chunk_size)
        .into_iter()
        .map(|mut row_chunk_idxs| {
            let first_idx = row_chunk_idxs.next().unwrap_or_default();
            let last_idx = row_chunk_idxs.last().unwrap_or(first_idx);
            let chunk_length = last_idx - first_idx + 1;

            (first_idx, chunk_length, last_idx)
        })
        .collect::<Vec<_>>()
}
