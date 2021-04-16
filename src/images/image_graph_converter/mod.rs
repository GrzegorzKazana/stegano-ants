mod _union;
mod segments_to_edge;
mod spatial;

use std::str::FromStr;

use crate::ant_colony::graph::Graph;
use crate::ant_colony::pheromone::Pheromone;
use crate::images::pixel_map::PixelMap;

pub use _union::Converters;
pub use segments_to_edge::KMeansConverter;
pub use segments_to_edge::SuperPixelConverter;
pub use segments_to_edge::WindowToEdgeConverter;
pub use spatial::SpatialEdgeChangeConverter;
pub use spatial::SpatialImageGraphConverter;

pub trait ImageGraphConverter: FromStrAndPixelMap {
    /// image in any form is expected to be passed via constructor
    ///
    /// graph distances are by default meant to be inversly proportional to region complexity
    /// (to encourage ants to explore complex regions)
    fn img_to_graph(&self) -> Graph;

    /// higher pheromone value translates to brighter pixels
    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap;

    /// higher complexity (lower graph distances) translates to bighter pixels
    fn visualize_conversion(&self) -> Option<PixelMap> {
        None
    }
}

pub trait FromStrAndPixelMap: Sized {
    fn from_str_and_pixel_map(pixel_map: &PixelMap, config: &str) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub enum ConverterStringConfig {
    SpatialEdgeChange(String),
    WindowToEdge(String),
    KMeans(String),
    SuperPixels(String),
    Inverted(Box<ConverterStringConfig>),
}

impl FromStr for ConverterStringConfig {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("i:") {
            return Self::from_str(s.replacen("i:", "", 1).as_ref())
                .map(Box::new)
                .map(Self::Inverted);
        }

        let mut config_iter = s.split(":");
        let name = config_iter.next().unwrap_or_default();
        let opts = config_iter.next().map(String::from).unwrap_or_default();

        match name {
            "spatial" => Some(Self::SpatialEdgeChange(opts)),
            "window" => Some(Self::WindowToEdge(opts)),
            "kmeans" => Some(Self::KMeans(opts)),
            "superpixels" => Some(Self::SuperPixels(opts)),
            _ => None,
        }
        .ok_or("Failed to parse image converter type")
    }
}

impl ToString for ConverterStringConfig {
    fn to_string(&self) -> String {
        match self {
            Self::SpatialEdgeChange(opts) => format!("spatial:{}", opts),
            Self::WindowToEdge(opts) => format!("window:{}", opts),
            Self::KMeans(opts) => format!("kmeans:{}", opts),
            Self::SuperPixels(opts) => format!("superpixels:{}", opts),
            Self::Inverted(opts) => format!("i:{}", opts.to_string()),
        }
    }
}
