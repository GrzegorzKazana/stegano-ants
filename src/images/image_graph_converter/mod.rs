mod _tests;
mod edge_change_converter;
mod spatial_image_graph_converter;

use crate::ant_colony::graph::Graph;
use crate::ant_colony::pheromone::Pheromone;
use crate::images::pixel_map::PixelMap;

pub use edge_change_converter::EdgeChangeConverter;
pub use spatial_image_graph_converter::SpatialImageGraphConverter;

pub trait ImageGraphConverter {
    fn initialize(pixel_map: &PixelMap) -> Self;

    fn img_to_graph(&self) -> Graph;

    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap;
}
