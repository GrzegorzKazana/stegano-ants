#[cfg(test)]
mod edge_change_converter_tests {
    use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node};
    use crate::images::image::Pixel;
    use crate::images::pixel_map::PixelMap;

    use super::super::super::{EdgeChangeConverter, ImageGraphConverter};

    fn mock_image() -> PixelMap {
        PixelMap::new(
            2,
            3,
            vec![
                Pixel::new(0, 0, 10, 10, 10),
                Pixel::new(1, 0, 20, 20, 20),
                Pixel::new(2, 0, 30, 30, 30),
                Pixel::new(0, 1, 40, 40, 40),
                Pixel::new(1, 1, 50, 50, 50),
                Pixel::new(2, 1, 60, 60, 60),
            ],
        )
    }

    #[test]
    fn it_creates_correct_graph_from_mock() {
        let img = mock_image();

        let result = EdgeChangeConverter::img_to_graph(&img);

        let nodes = vec![
            Node {
                id: 0,
                adjacency_list: vec![
                    AdjacencyListEntry::new(0, 1, 100.0 + 100.0 + 100.0),
                    AdjacencyListEntry::new(0, 3, 900.0 + 900.0 + 900.0),
                ],
            },
            Node {
                id: 1,
                adjacency_list: vec![
                    AdjacencyListEntry::new(1, 2, 100.0 + 100.0 + 100.0),
                    AdjacencyListEntry::new(1, 4, 900.0 + 900.0 + 900.0),
                    AdjacencyListEntry::new(1, 0, 100.0 + 100.0 + 100.0),
                ],
            },
            Node {
                id: 2,
                adjacency_list: vec![
                    AdjacencyListEntry::new(2, 5, 900.0 + 900.0 + 900.0),
                    AdjacencyListEntry::new(2, 1, 100.0 + 100.0 + 100.0),
                ],
            },
            Node {
                id: 3,
                adjacency_list: vec![
                    AdjacencyListEntry::new(3, 0, 900.0 + 900.0 + 900.0),
                    AdjacencyListEntry::new(3, 4, 100.0 + 100.0 + 100.0),
                ],
            },
            Node {
                id: 4,
                adjacency_list: vec![
                    AdjacencyListEntry::new(4, 1, 900.0 + 900.0 + 900.0),
                    AdjacencyListEntry::new(4, 5, 100.0 + 100.0 + 100.0),
                    AdjacencyListEntry::new(4, 3, 100.0 + 100.0 + 100.0),
                ],
            },
            Node {
                id: 5,
                adjacency_list: vec![
                    AdjacencyListEntry::new(5, 2, 900.0 + 900.0 + 900.0),
                    AdjacencyListEntry::new(5, 4, 100.0 + 100.0 + 100.0),
                ],
            },
        ];

        let expected = Graph::from_node_vector(nodes);

        assert_eq!(result, expected);
    }
}
