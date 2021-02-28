#[cfg(test)]
mod edge_change_converter_tests {
    use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node};
    use crate::ant_colony::pheromone::Pheromone;
    use crate::common::utils::UniquePair;
    use crate::images::image::Pixel;
    use crate::images::pixel_map::PixelMap;

    use super::super::super::{EdgeChangeConverter, ImageGraphConverter};

    const MAX_PIXEL_DISTANCE: f32 = 255.0 * 255.0 * 3.0;

    fn mock_image() -> PixelMap {
        PixelMap::new(
            2,
            3,
            vec![
                Pixel::new(0, 0, 10, 10, 10), // 0 (id)
                Pixel::new(1, 0, 20, 20, 20), // 1
                Pixel::new(2, 0, 30, 30, 30), // 2
                Pixel::new(0, 1, 40, 40, 40), // 3
                Pixel::new(1, 1, 50, 50, 50), // 4
                Pixel::new(2, 1, 60, 60, 60), // 5
            ],
        )
    }

    fn mock_pheromone() -> Pheromone {
        // make sure max value is 1.0, so we do not have to bother with normalization
        Pheromone::from_values(map!(
            UniquePair::generate_key(0, 1) => 0.1,
            UniquePair::generate_key(0, 3) => 0.2,
            UniquePair::generate_key(1, 2) => 0.4,
            UniquePair::generate_key(1, 4) => 0.5,
            UniquePair::generate_key(2, 5) => 0.6,
            UniquePair::generate_key(3, 4) => 0.8,
            UniquePair::generate_key(4, 5) => 1.0
        ))
    }

    #[test]
    fn it_creates_correct_graph_from_mock() {
        let img = mock_image();

        let result = EdgeChangeConverter::initialize(&img).img_to_graph();

        let nodes = vec![
            Node {
                id: 0,
                adjacency_list: vec![
                    AdjacencyListEntry::new(
                        0,
                        1,
                        (100.0 + 100.0 + 100.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                    AdjacencyListEntry::new(
                        0,
                        3,
                        (900.0 + 900.0 + 900.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                ],
            },
            Node {
                id: 1,
                adjacency_list: vec![
                    AdjacencyListEntry::new(
                        1,
                        2,
                        (100.0 + 100.0 + 100.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                    AdjacencyListEntry::new(
                        1,
                        4,
                        (900.0 + 900.0 + 900.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                    AdjacencyListEntry::new(
                        1,
                        0,
                        (100.0 + 100.0 + 100.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                ],
            },
            Node {
                id: 2,
                adjacency_list: vec![
                    AdjacencyListEntry::new(
                        2,
                        5,
                        (900.0 + 900.0 + 900.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                    AdjacencyListEntry::new(
                        2,
                        1,
                        (100.0 + 100.0 + 100.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                ],
            },
            Node {
                id: 3,
                adjacency_list: vec![
                    AdjacencyListEntry::new(
                        3,
                        0,
                        (900.0 + 900.0 + 900.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                    AdjacencyListEntry::new(
                        3,
                        4,
                        (100.0 + 100.0 + 100.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                ],
            },
            Node {
                id: 4,
                adjacency_list: vec![
                    AdjacencyListEntry::new(
                        4,
                        1,
                        (900.0 + 900.0 + 900.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                    AdjacencyListEntry::new(
                        4,
                        5,
                        (100.0 + 100.0 + 100.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                    AdjacencyListEntry::new(
                        4,
                        3,
                        (100.0 + 100.0 + 100.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                ],
            },
            Node {
                id: 5,
                adjacency_list: vec![
                    AdjacencyListEntry::new(
                        5,
                        2,
                        (900.0 + 900.0 + 900.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                    AdjacencyListEntry::new(
                        5,
                        4,
                        (100.0 + 100.0 + 100.0) / MAX_PIXEL_DISTANCE + stability_factor!(),
                    ),
                ],
            },
        ];

        let expected = Graph::from_node_vector(nodes);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_creates_correct_visualization_for_pheromone() {
        let img = mock_image();
        let pheromone = mock_pheromone();

        let result = EdgeChangeConverter::initialize(&img).visualize_pheromone(&pheromone);

        let expected = PixelMap::new(
            2,
            3,
            vec![
                Pixel::grey(0, 0, (255.0 * 0.15) as u8), // (0.1 + 0.2) / 2
                Pixel::grey(1, 0, (255.0 * 1.0 / 3.0) as u8), // (0.1 + 0.4 + 0.5) / 3
                Pixel::grey(2, 0, (255.0 * 0.5) as u8),  // (0.4 + 0.6) / 2
                Pixel::grey(0, 1, (255.0 * 0.5) as u8),  // (0.2 + 0.8) / 2
                Pixel::grey(1, 1, (255.0 * 2.3 / 3.0) as u8), // (0.5 + 0.8 + 1.0) / 3
                Pixel::grey(2, 1, (255.0 * 0.8) as u8),  // (0.6 + 1.0) / 2
            ],
        );

        assert_eq!(result, expected);
    }
}
