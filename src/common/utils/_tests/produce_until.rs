#[cfg(test)]
mod common_utils_produce_until_tests {
    use super::super::super::produce_until;

    #[test]
    fn it_yields_initial_if_pred_is_true() {
        let result = produce_until(1, |a, _| a + 1, |_, _| true);

        assert_eq!(result, 1);
    }

    #[test]
    fn it_yields_first_value_that_fulfills_predicate() {
        let result = produce_until(1, |a, _| a + 1, |a, _| *a > 4);

        assert_eq!(result, 5);
    }
}
