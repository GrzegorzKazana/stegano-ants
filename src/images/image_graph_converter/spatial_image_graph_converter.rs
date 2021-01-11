use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node, NodeId};
use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;

use super::ImageGraphConverter;

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
}

impl<C: SpatialImageGraphConverter> ImageGraphConverter for C {
    fn img_to_graph(pixel_map: &PixelMap) -> Graph {
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
}
