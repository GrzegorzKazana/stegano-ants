mod _union;
mod segments_to_edge;
mod spatial;
mod superpixels;

use std::str::FromStr;

use crate::ant_colony::graph::Graph;
use crate::ant_colony::pheromone::Pheromone;
use crate::images::pixel_map::PixelMap;

pub use _union::Converters;
pub use segments_to_edge::KMeansConverter;
pub use segments_to_edge::WindowToEdgeConverter;
pub use spatial::SpatialEdgeChangeConverter;
pub use spatial::SpatialImageGraphConverter;

pub trait ImageGraphConverter: FromStrAndPixelMap {
    /// image in any form is expected to be passed via constructor
    fn img_to_graph(&self) -> Graph;

    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap;

    fn visualize_conversion(&self) -> Option<PixelMap> {
        None
    }
}

pub trait FromStrAndPixelMap: Sized {
    fn from_str_and_pixel_map(pixel_map: &PixelMap, config: &str) -> Option<Self>;
}

#[derive(Debug)]
pub enum ConverterStringConfig {
    SpatialEdgeChange(String),
    WindowToEdge(String),
    KMeans(String),
}

impl FromStr for ConverterStringConfig {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config_iter = s.split(":");
        let name = config_iter.next().unwrap_or_default();
        let opts = config_iter.next().map(String::from).unwrap_or_default();

        match name {
            "spatial" => Some(Self::SpatialEdgeChange(opts)),
            "window" => Some(Self::WindowToEdge(opts)),
            "kmeans" => Some(Self::KMeans(opts)),
            _ => None,
        }
        .ok_or("Failed to parse image converter type")
    }
}
