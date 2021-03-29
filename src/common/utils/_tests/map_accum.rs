#[cfg(test)]
mod common_utils_map_accum_tests {
    use super::super::super::MapAccumExt;

    #[test]
    fn it_yields_same_items_with_identity_mapper() {
        let input = vec![1, 2, 3, 4];
        let result = input
            .iter()
            .cloned()
            .map_accum(0, |acc, curr| (acc, curr))
            .collect::<Vec<_>>();

        let expected = input.clone();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_allows_for_calculating_acc_sum() {
        let input = vec![1, 2, 3, 4];
        let result = input
            .iter()
            .cloned()
            .map_accum(0, |acc, curr| (acc + curr, acc + curr))
            .collect::<Vec<_>>();

        let expected = vec![1, 3, 6, 10];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_as_exact_sized_iterator() {
        let input = vec![1, 2, 3, 4];
        let result = input.iter().map_accum(0, |acc, curr| (acc, curr)).len();
        let expected = input.iter().len();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_as_double_ended_iterator() {
        let input = vec![1, 2, 3, 4];
        let result = input
            .iter()
            .cloned()
            .map_accum(0, |acc, curr| (acc, curr))
            .rev()
            .collect::<Vec<_>>();

        let expected = vec![4, 3, 2, 1];

        assert_eq!(result, expected);
    }
}
