mod _tests;
mod edge_change_converter;
mod spatial_image_graph_converter;

use crate::ant_colony::graph::Graph;
use crate::images::pixel_map::PixelMap;

pub use spatial_image_graph_converter::SpatialImageGraphConverter;

pub trait ImageGraphConverter {
    fn img_to_graph(pixel_map: &PixelMap) -> Graph;
}
