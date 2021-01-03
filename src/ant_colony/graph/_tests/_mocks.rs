use crate::ant_colony::graph::{AdjacencyListEntry, Node, NodeId};

#[allow(dead_code)]
pub fn mock_graph_vector() -> Vec<Node> {
    vec![
        Node {
            id: 0,
            adjacency_list: vec![
                AdjacencyListEntry {
                    from: 0,
                    to: 1,
                    distance: 1.0,
                },
                AdjacencyListEntry {
                    from: 0,
                    to: 2,
                    distance: 2.0,
                },
                AdjacencyListEntry {
                    from: 0,
                    to: 3,
                    distance: 10.0,
                },
            ],
        },
        Node {
            id: 1,
            adjacency_list: vec![
                AdjacencyListEntry {
                    from: 1,
                    to: 0,
                    distance: 1.0,
                },
                AdjacencyListEntry {
                    from: 1,
                    to: 2,
                    distance: 2.0,
                },
                AdjacencyListEntry {
                    from: 1,
                    to: 3,
                    distance: 5.0,
                },
            ],
        },
        Node {
            id: 2,
            adjacency_list: vec![
                AdjacencyListEntry {
                    from: 2,
                    to: 0,
                    distance: 2.0,
                },
                AdjacencyListEntry {
                    from: 2,
                    to: 1,
                    distance: 2.0,
                },
                AdjacencyListEntry {
                    from: 2,
                    to: 3,
                    distance: 6.0,
                },
            ],
        },
        Node {
            id: 3,
            adjacency_list: vec![
                AdjacencyListEntry {
                    from: 3,
                    to: 0,
                    distance: 10.0,
                },
                AdjacencyListEntry {
                    from: 3,
                    to: 1,
                    distance: 5.0,
                },
                AdjacencyListEntry {
                    from: 3,
                    to: 2,
                    distance: 6.0,
                },
            ],
        },
    ]
}

#[allow(dead_code)]
pub fn mock_graph_tuple() -> Vec<(NodeId, NodeId, f32)> {
    vec![
        (0, 1, 1.0),
        (0, 2, 2.0),
        (0, 3, 10.0),
        (1, 2, 2.0),
        (1, 3, 5.0),
        (2, 3, 6.0),
    ]
}
