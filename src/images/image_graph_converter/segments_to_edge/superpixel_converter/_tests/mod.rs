#[cfg(test)]
mod superpixel_converter_tests {
    use super::super::SuperPixelConverter;

    #[test]
    fn it_should_adjust_labels_if_too_much() {
        let labels = vec![6, 0, 1, 2, 5, 3, 4, 5, 6];
        let result = SuperPixelConverter::fix_unexact_superpixel_count(5, 7, labels);
        let expected = vec![4, 0, 1, 2, 3, 3, 4, 3, 4];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_adjust_labels_if_not_enough() {
        let labels = vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4];
        let result = SuperPixelConverter::fix_unexact_superpixel_count(7, 5, labels);
        let expected = vec![5, 0, 6, 1, 2, 2, 3, 3, 4, 4];

        assert_eq!(result, expected);
    }
}
