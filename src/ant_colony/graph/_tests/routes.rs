#[cfg(test)]
mod graph_routes_tests {
    use super::super::super::{AdjacencyListEntry, Route};

    fn get_mock_route() -> Route {
        Route::default()
            .add_step(AdjacencyListEntry {
                key: 0,
                from: 0,
                to: 1,
                distance: 1.0,
                visibility: 1.0,
            })
            .add_step(AdjacencyListEntry {
                key: 1,
                from: 1,
                to: 2,
                distance: 2.0,
                visibility: 0.5,
            })
            .add_step(AdjacencyListEntry {
                key: 2,
                from: 2,
                to: 3,
                distance: 3.0,
                visibility: 0.3333,
            })
    }

    #[test]
    fn it_allows_for_adding_edges() {
        let route = get_mock_route();

        assert_eq!(route.get_length(), 3);
        assert_eq!(route.get_distance(), 6.0);
    }

    #[test]
    fn it_returns_correct_node_ids() {
        let route = get_mock_route();

        assert_eq!(route.get_nodes(), vec![0, 1, 2, 3]);
    }
}
