use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node, NodeId};
use crate::ant_colony::pheromone::Pheromone;
use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;

/// Trait shared by all ImageGraphConverters that create graphs
/// that have 1:1 mapping between pixel and node.
/// Structs implementing need to provide list of pixels which will be adjacent for given pixel.
pub trait SpatialImageGraphConverter {
    fn get_pixel_neighbours(pixel_map: &PixelMap, pixel: &Pixel) -> Vec<Pixel>;

    fn calc_distance_between_pixels(pixel_a: &Pixel, pixel_b: &Pixel) -> f32;

    fn pixel_to_id(pixel_map: &PixelMap, pixel: &Pixel) -> NodeId {
        (pixel.y * pixel_map.width + pixel.x) as NodeId
    }

    fn get_pixel_adjacency_list(
        pixel_map: &PixelMap,
        pixel: &Pixel,
        node_id: NodeId,
    ) -> Vec<AdjacencyListEntry> {
        Self::get_pixel_neighbours(pixel_map, pixel)
            .iter()
            .map(|neighbour_pixel| {
                let neighbour_id = Self::pixel_to_id(pixel_map, neighbour_pixel);
                let distance = Self::calc_distance_between_pixels(pixel, neighbour_pixel);

                AdjacencyListEntry::new(node_id, neighbour_id, distance)
            })
            .collect()
    }

    fn construct_graph(pixel_map: &PixelMap) -> Graph {
        let nodes = pixel_map
            .pixels()
            .iter()
            .map(|pixel| {
                let node_id = Self::pixel_to_id(pixel_map, pixel);

                let adjacency_list = Self::get_pixel_adjacency_list(pixel_map, pixel, node_id);

                Node {
                    id: node_id,
                    adjacency_list,
                }
            })
            .collect();

        Graph::from_node_vector(nodes)
    }

    fn calculate_pixel_intensity_from_pheromone(
        pheromone: &Pheromone,
        adjacent_edges: &[AdjacencyListEntry],
    ) -> u8 {
        let num_of_edges = adjacent_edges.len();

        let intensity_sum: f32 = adjacent_edges
            .into_iter()
            .map(|edge| pheromone.get_pheromone_for_edge(edge.key))
            .sum();

        (255.0 * intensity_sum / num_of_edges as f32) as u8
    }

    fn construct_pheromone_visualization(
        pixel_map: &PixelMap,
        graph: &Graph,
        pheromone: &Pheromone,
    ) -> PixelMap {
        let pheromone_norm = pheromone.normalize();

        pixel_map.map(|pixel| {
            let node_id = Self::pixel_to_id(pixel_map, pixel);
            let edges_adjacent_to_pixel = graph.get_adjacent_edges(&node_id);
            let intensity_level = Self::calculate_pixel_intensity_from_pheromone(
                &pheromone_norm,
                &edges_adjacent_to_pixel,
            );

            Pixel::grey(pixel.x, pixel.y, intensity_level)
        })
    }
}
