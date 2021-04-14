#[cfg(test)]
mod vec_2d_tests {
    use super::super::super::Vec2d;

    fn mock() -> Vec2d<usize> {
        Vec2d::new(
            vec![
                0, 1, 2, 3, //
                4, 5, 6, 7, //
                8, 9, 10, 11,
            ],
            4,
            3,
        )
    }

    #[test]
    fn it_allows_for_indexing_by_delta() {
        let data = mock();

        assert_eq!(data.index_by_delta((1, 1), (-1, 1)), Some(&8));
        assert_eq!(data.index_by_delta((1, 1), (2, -1)), Some(&3));
    }

    #[test]
    fn it_allows_for_indexing_8_neighbours() {
        let data = mock();
        let mut result = data.index_neighbours_8((1, 1)).cloned().collect::<Vec<_>>();
        let mut expected = vec![0usize, 1, 2, 4, 6, 8, 9, 10];

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_allows_for_indexing_by_range() {
        let data = mock();
        let result = data.iter_block((1, 1), 1).cloned().collect::<Vec<_>>();
        let expected = vec![0usize, 1, 2, 4, 5, 6, 8, 9, 10];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_allows_for_indexing_by_range_overflow() {
        let data = mock();
        let result = data.iter_block((1, 1), 2).cloned().collect::<Vec<_>>();
        let expected = vec![0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        assert_eq!(result, expected);
    }
}
