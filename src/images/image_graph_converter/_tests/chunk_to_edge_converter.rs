#[cfg(test)]
mod chunk_to_edge_converter_tests {
    use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node};
    use crate::ant_colony::pheromone::Pheromone;
    use crate::common::utils::UniquePair;
    use crate::images::image::Pixel;
    use crate::images::pixel_map::PixelMap;

    use super::super::super::{ChunkToEdgeConverter, ImageGraphConverter};

    fn mock_image() -> PixelMap {
        PixelMap::new(
            6,
            5,
            vec![
                // using blue channel as primitive form of id
                Pixel::new(0, 0, 5, 5, 5),
                Pixel::new(1, 0, 10, 10, 10),
                Pixel::new(2, 0, 15, 15, 15),
                Pixel::new(3, 0, 20, 20, 20),
                Pixel::new(4, 0, 25, 25, 25),
                //
                Pixel::new(0, 1, 30, 30, 30),
                Pixel::new(1, 1, 35, 35, 35),
                Pixel::new(2, 1, 40, 40, 40),
                Pixel::new(3, 1, 45, 45, 45),
                Pixel::new(4, 1, 50, 50, 50),
                //
                Pixel::new(0, 2, 55, 55, 55),
                Pixel::new(1, 2, 60, 60, 60),
                Pixel::new(2, 2, 65, 65, 65),
                Pixel::new(3, 2, 70, 70, 70),
                Pixel::new(4, 2, 75, 75, 75),
                //
                Pixel::new(0, 3, 80, 80, 80),
                Pixel::new(1, 3, 85, 85, 85),
                Pixel::new(2, 3, 90, 90, 90),
                Pixel::new(3, 3, 95, 95, 95),
                Pixel::new(4, 3, 100, 100, 100),
                //
                Pixel::new(0, 4, 105, 105, 105),
                Pixel::new(1, 4, 110, 110, 110),
                Pixel::new(2, 4, 115, 115, 115),
                Pixel::new(3, 4, 120, 120, 120),
                Pixel::new(4, 4, 125, 125, 125),
                //
                Pixel::new(0, 5, 130, 130, 130),
                Pixel::new(1, 5, 135, 135, 135),
                Pixel::new(2, 5, 140, 140, 140),
                Pixel::new(3, 5, 145, 145, 145),
                Pixel::new(4, 5, 150, 150, 150),
            ],
        )
    }

    fn mock_pheromone() -> Pheromone {
        // make sure max value is 1.0, so we do not have to bother with normalization
        Pheromone::from_values(map!(
            UniquePair::generate_key(0, 1) => 1.0,
            UniquePair::generate_key(0, 2) => 0.9,
            UniquePair::generate_key(0, 3) => 1.0,
            UniquePair::generate_key(0, 4) => 0.8,
            UniquePair::generate_key(0, 5) => 1.0,
            //
            UniquePair::generate_key(1, 2) => 0.7,
            UniquePair::generate_key(1, 3) => 1.0,
            UniquePair::generate_key(1, 4) => 0.6,
            UniquePair::generate_key(1, 5) => 1.0,
            //
            UniquePair::generate_key(2, 3) => 0.5,
            UniquePair::generate_key(2, 4) => 1.0,
            UniquePair::generate_key(2, 5) => 0.4,
            //
            UniquePair::generate_key(3, 4) => 1.0,
            UniquePair::generate_key(3, 5) => 0.3,
            //
            UniquePair::generate_key(4, 5) => 1.0
        ))
    }

    #[test]
    fn it_should_create_correct_graph() {
        let image = mock_image();
        let result = ChunkToEdgeConverter::new(&image, 5, 3, 6).img_to_graph();

        let dist = 1.0 / (156.25 + stability_factor!());
        let expected = Graph::from_node_vector(vec![
            Node {
                id: 0,
                adjacency_list: vec![
                    AdjacencyListEntry::new(0, 1, dist),
                    AdjacencyListEntry::new(0, 2, dist),
                    AdjacencyListEntry::new(0, 3, dist),
                    AdjacencyListEntry::new(0, 4, dist),
                    AdjacencyListEntry::new(0, 5, dist),
                ],
            },
            Node {
                id: 1,
                adjacency_list: vec![
                    AdjacencyListEntry::new(1, 0, dist),
                    AdjacencyListEntry::new(1, 2, dist),
                    AdjacencyListEntry::new(1, 3, dist),
                    AdjacencyListEntry::new(1, 4, dist),
                    AdjacencyListEntry::new(1, 5, dist),
                ],
            },
            Node {
                id: 2,
                adjacency_list: vec![
                    AdjacencyListEntry::new(2, 0, dist),
                    AdjacencyListEntry::new(2, 1, dist),
                    AdjacencyListEntry::new(2, 3, dist),
                    AdjacencyListEntry::new(2, 4, dist),
                    AdjacencyListEntry::new(2, 5, dist),
                ],
            },
            Node {
                id: 3,
                adjacency_list: vec![
                    AdjacencyListEntry::new(3, 0, dist),
                    AdjacencyListEntry::new(3, 1, dist),
                    AdjacencyListEntry::new(3, 2, dist),
                    AdjacencyListEntry::new(3, 4, dist),
                    AdjacencyListEntry::new(3, 5, dist),
                ],
            },
            Node {
                id: 4,
                adjacency_list: vec![
                    AdjacencyListEntry::new(4, 0, dist),
                    AdjacencyListEntry::new(4, 1, dist),
                    AdjacencyListEntry::new(4, 2, dist),
                    AdjacencyListEntry::new(4, 3, dist),
                    AdjacencyListEntry::new(4, 5, dist),
                ],
            },
            Node {
                id: 5,
                adjacency_list: vec![
                    AdjacencyListEntry::new(5, 0, dist),
                    AdjacencyListEntry::new(5, 1, dist),
                    AdjacencyListEntry::new(5, 2, dist),
                    AdjacencyListEntry::new(5, 3, dist),
                    AdjacencyListEntry::new(5, 4, dist),
                ],
            },
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_correctly_visualize_pheromone() {
        let image = mock_image();
        let pheromone = mock_pheromone();
        let result = ChunkToEdgeConverter::new(&image, 5, 3, 6).visualize_pheromone(&pheromone);
        let expected = PixelMap::new(
            6,
            5,
            vec![
                Pixel::new(0, 0, 255, 255, 255),
                Pixel::new(1, 0, 229, 229, 229),
                Pixel::new(2, 0, 255, 255, 255),
                Pixel::new(3, 0, 204, 204, 204),
                Pixel::new(4, 0, 255, 255, 255),
                //
                Pixel::new(0, 1, 255, 255, 255),
                Pixel::new(1, 1, 229, 229, 229),
                Pixel::new(2, 1, 255, 255, 255),
                Pixel::new(3, 1, 204, 204, 204),
                Pixel::new(4, 1, 255, 255, 255),
                //
                Pixel::new(0, 2, 178, 178, 178),
                Pixel::new(1, 2, 255, 255, 255),
                Pixel::new(2, 2, 153, 153, 153),
                Pixel::new(3, 2, 255, 255, 255),
                Pixel::new(4, 2, 127, 127, 127),
                //
                Pixel::new(0, 3, 178, 178, 178),
                Pixel::new(1, 3, 255, 255, 255),
                Pixel::new(2, 3, 153, 153, 153),
                Pixel::new(3, 3, 255, 255, 255),
                Pixel::new(4, 3, 127, 127, 127),
                //
                Pixel::new(0, 4, 255, 255, 255),
                Pixel::new(1, 4, 102, 102, 102),
                Pixel::new(2, 4, 255, 255, 255),
                Pixel::new(3, 4, 76, 76, 76),
                Pixel::new(4, 4, 255, 255, 255),
                //
                Pixel::new(0, 5, 255, 255, 255),
                Pixel::new(1, 5, 102, 102, 102),
                Pixel::new(2, 5, 255, 255, 255),
                Pixel::new(3, 5, 76, 76, 76),
                Pixel::new(4, 5, 255, 255, 255),
            ],
        );

        assert_eq!(result, expected);
    }
}
