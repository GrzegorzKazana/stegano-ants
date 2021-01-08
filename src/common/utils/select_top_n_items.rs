use itertools::Itertools;

pub fn select_top_n_items<T: Copy, P: PartialOrd, F: Fn(&T) -> &P>(
    data: &[T],
    n: usize,
    picker: F,
) -> Vec<T> {
    let data_copy = data.iter().collect_vec();

    let partially_sorted_data = (0..n).fold(data_copy, |mut acc, idx| {
        let maybe_max_idx = acc
            .iter()
            .skip(idx)
            .position_max_by(|a, b| picker(a).partial_cmp(picker(b)).unwrap());

        match maybe_max_idx {
            Option::Some(max_idx) => {
                acc.swap(idx, idx + max_idx);
                acc
            }
            Option::None => acc,
        }
    });

    partially_sorted_data
        .iter()
        .take(n)
        .map(|item| **item)
        .collect_vec()
}
