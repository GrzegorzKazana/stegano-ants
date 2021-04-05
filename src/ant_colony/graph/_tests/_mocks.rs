use crate::ant_colony::graph::{AdjacencyListEntry, Node, NodeId};

pub fn mock_graph_vector() -> Vec<Node> {
    vec![
        Node {
            id: 0,
            adjacency_list: vec![
                AdjacencyListEntry::new(0, 1, 3.0),
                AdjacencyListEntry::new(0, 2, 4.0),
                AdjacencyListEntry::new(0, 3, 5.0),
            ],
        },
        Node {
            id: 1,
            adjacency_list: vec![
                AdjacencyListEntry::new(1, 0, 3.0),
                AdjacencyListEntry::new(1, 2, 5.0),
                AdjacencyListEntry::new(1, 3, 4.0),
            ],
        },
        Node {
            id: 2,
            adjacency_list: vec![
                AdjacencyListEntry::new(2, 0, 4.0),
                AdjacencyListEntry::new(2, 1, 5.0),
                AdjacencyListEntry::new(2, 3, 3.0),
            ],
        },
        Node {
            id: 3,
            adjacency_list: vec![
                AdjacencyListEntry::new(3, 0, 5.0),
                AdjacencyListEntry::new(3, 1, 4.0),
                AdjacencyListEntry::new(3, 2, 3.0),
            ],
        },
    ]
}

pub fn mock_graph_tuple() -> Vec<(NodeId, NodeId, f32)> {
    vec![
        (0, 1, 3.0),
        (0, 2, 4.0),
        (0, 3, 5.0),
        (1, 2, 5.0),
        (1, 3, 4.0),
        (2, 3, 3.0),
    ]
}

pub fn mock_coordinates() -> &'static str {
    "0,0\n3,0\n0,4\n3,4\n"
}
