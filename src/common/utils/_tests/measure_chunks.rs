#[cfg(test)]
mod common_utils_measure_chunks_tests {
    use super::super::super::measure_chunks;

    #[test]
    fn it_corrctly_returns_chunks_info_for_equal_sized_chunks() {
        let result = measure_chunks(9, 3);
        let expected = vec![(0, 3, 2), (3, 3, 5), (6, 3, 8)];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_corrctly_returns_chunks_info_for_unequal_sized_chunks() {
        let result = measure_chunks(10, 3);
        let expected = vec![(0, 3, 2), (3, 3, 5), (6, 4, 9)];

        assert_eq!(result, expected);
    }
}
