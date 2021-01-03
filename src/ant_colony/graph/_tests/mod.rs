// #[cfg(test)]
// mod graph_tests {
//     use crate::ant_colony::graph::{Edge, Graph, Node};

//     #[test]
//     fn it_correctly_creates_nodes() {
//         let id = String::from("testid");
//         let node = Node::new(id.clone());

//         assert_eq!(id, node.id);
//     }

//     #[test]
//     fn it_generates_edge_key_in_correct_order() {
//         let node_a = Node::new(String::from("id_a"));
//         let node_b = Node::new(String::from("id_b"));
//         let edge = Edge::new(&node_a, &node_b, 5.0);

//         assert_eq!(edge.get_key(), "id_a-id_b");
//     }

//     #[test]
//     fn it_generates_edge_key_regardless_to_order() {
//         let node_a = Node::new(String::from("id_a"));
//         let node_b = Node::new(String::from("id_b"));
//         let edge_a = Edge::new(&node_a, &node_b, 5.0);
//         let edge_b = Edge::new(&node_b, &node_a, 5.0);

//         assert_eq!(edge_a.get_key(), edge_b.get_key());
//     }

//     #[test]
//     fn it_returns_adjacent_edges() {
//         let node_a = Node::new(String::from("id_a"));
//         let node_b = Node::new(String::from("id_b"));
//         let node_c = Node::new(String::from("id_c"));
//         let edge_a = Edge::new(&node_a, &node_b, 5.0);
//         let edge_b = Edge::new(&node_a, &node_c, 5.0);

//         let edges = vec![edge_a, edge_b];
//         let nodes = vec![node_a, node_b, node_c];

//         // edges;

//         // let graph = Graph::new(edges, nodes);
//         // let graph = Graph::new(vec![edge_a, edge_b], vec![node_a, node_b, node_c]);
//     }
// }
