#[cfg(test)]
mod graph_tests {
    use super::super::_mocks::{mock_coordinates, mock_graph_tuple, mock_graph_vector};
    use crate::ant_colony::graph::Graph;

    #[test]
    fn it_returns_correct_node_ids() {
        let graph = Graph::from_node_vector(mock_graph_vector());
        let expected_ids: Vec<_> = mock_graph_vector().iter().map(|node| node.id).collect();

        let mut result = graph.get_node_ids();
        result.sort();

        assert_eq!(result, expected_ids);
    }

    #[test]
    fn it_returns_correct_adjacent_nodes() {
        let graph = Graph::from_node_vector(mock_graph_vector());
        let expected_edges: Vec<_> = mock_graph_vector()[0]
            .adjacency_list
            .iter()
            .map(|n| n.to)
            .collect();

        let result: Vec<_> = graph.get_adjacent_edges(&0).iter().map(|n| n.to).collect();

        assert_eq!(result, expected_edges);
    }

    #[test]
    fn it_supports_initialization_from_tuples() {
        assert_eq!(
            Graph::from_node_vector(mock_graph_vector()),
            Graph::from_neighbour_tuples(mock_graph_tuple())
        )
    }

    #[test]
    fn it_supports_initialization_from_coords() {
        assert_eq!(
            Graph::from_node_vector(mock_graph_vector()),
            Graph::from_coordinate_csv(mock_coordinates())
        )
    }

    #[test]
    fn it_estimates_cycle_length() {
        let graph = Graph::from_coordinate_csv(mock_coordinates());
        let result = graph.estimate_hamiltonian_cycle();
        let expected = Some(14.0f32);

        assert_eq!(result, expected);
    }
}
