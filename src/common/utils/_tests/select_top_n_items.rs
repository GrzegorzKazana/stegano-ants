#[cfg(test)]
mod common_utils_select_top_n_items {
    use super::super::super::select_top_n_items;

    #[test]
    fn it_return_top_n_elements_if_available() {
        let result = select_top_n_items(&vec![2.0, 1.0, 5.0, 3.0], 3, |a| a);
        let expected: Vec<f32> = vec![5.0, 3.0, 2.0];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_allows_for_picking_from_items() {
        let result = select_top_n_items(
            &vec![('a', 2.0), ('b', 1.0), ('c', 5.0), ('d', 3.0)],
            3,
            |(_, num)| num,
        );
        let expected: Vec<(char, f32)> = vec![('c', 5.0), ('d', 3.0), ('a', 2.0)];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_returns_less_items_if_provided_less_of_them() {
        let result = select_top_n_items(&vec![2.0, 1.0, 5.0, 3.0], 5, |a| a);
        let expected: Vec<f32> = vec![5.0, 3.0, 2.0, 1.0];

        assert_eq!(result, expected);
    }
}
