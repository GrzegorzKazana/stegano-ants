use crate::ant_colony::graph::{AdjacencyListEntry, Node, NodeId};

pub fn mock_graph_vector() -> Vec<Node> {
    vec![
        Node {
            id: 0,
            adjacency_list: vec![
                AdjacencyListEntry::new(0, 1, 1.0),
                AdjacencyListEntry::new(0, 2, 2.0),
                AdjacencyListEntry::new(0, 3, 10.0),
            ],
        },
        Node {
            id: 1,
            adjacency_list: vec![
                AdjacencyListEntry::new(1, 0, 1.0),
                AdjacencyListEntry::new(1, 2, 2.0),
                AdjacencyListEntry::new(1, 3, 5.0),
            ],
        },
        Node {
            id: 2,
            adjacency_list: vec![
                AdjacencyListEntry::new(2, 0, 2.0),
                AdjacencyListEntry::new(2, 1, 2.0),
                AdjacencyListEntry::new(2, 3, 6.0),
            ],
        },
        Node {
            id: 3,
            adjacency_list: vec![
                AdjacencyListEntry::new(3, 0, 10.0),
                AdjacencyListEntry::new(3, 1, 5.0),
                AdjacencyListEntry::new(3, 2, 6.0),
            ],
        },
    ]
}

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
