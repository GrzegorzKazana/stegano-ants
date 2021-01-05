#[cfg(test)]
mod pheromone_reader_tests {
    use itertools::Itertools;

    use crate::ant_colony::pheromone::Pheromone;

    use super::super::PheromoneReader;

    fn get_mock_pheromone() -> Pheromone {
        Pheromone::new()
            .initialize_pheromone_for_edge(0, 1.0)
            .initialize_pheromone_for_edge(1, 0.4)
            .initialize_pheromone_for_edge(2, 0.0)
            .initialize_pheromone_for_edge(3, 0.6)
            .initialize_pheromone_for_edge(4, 0.7)
    }

    fn assert_any_order<T: Eq + std::fmt::Debug + Ord>(a: Vec<T>, b: Vec<T>) {
        assert_eq!(
            a.iter().sorted().collect_vec(),
            b.iter().sorted().collect_vec()
        );
    }

    #[test]
    fn it_returns_top_n_edges() {
        let pheromone = get_mock_pheromone();
        let keys = PheromoneReader::get_top_n_edge_keys(&pheromone, 3);

        assert_any_order(keys, vec![0, 3, 4]);
    }

    #[test]
    fn it_returns_all_edges_if_asked_for_too_much() {
        let pheromone = get_mock_pheromone();
        let keys = PheromoneReader::get_top_n_edge_keys(&pheromone, 10);

        assert_any_order(keys, vec![0, 4, 3, 1, 2]);
    }

    #[test]
    fn it_returns_edges_above_given_level() {
        let pheromone = get_mock_pheromone();
        let keys = PheromoneReader::get_edge_keys_with_pheromone_above(&pheromone, 0.5);

        assert_any_order(keys, vec![0, 3, 4]);
    }

    #[test]
    fn it_returns_no_edges_above_too_high_level() {
        let pheromone = get_mock_pheromone();
        let keys = PheromoneReader::get_edge_keys_with_pheromone_above(&pheromone, 10.0);

        assert_eq!(keys, vec![]);
    }

    #[test]
    fn it_counts_elements_above_level() {
        let pheromone = get_mock_pheromone();
        let num = PheromoneReader::count_edges_with_pheromone_above(&pheromone, 0.5);

        assert_eq!(num, 3);
    }
}
