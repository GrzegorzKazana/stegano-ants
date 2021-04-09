use std::str::FromStr;

use crate::ant_colony::graph::Graph;
use crate::ant_colony::pheromone::Pheromone;

use crate::images::pixel_map::PixelMap;

use super::{
    ConverterStringConfig, FromStrAndPixelMap, ImageGraphConverter, SpatialEdgeChangeConverter,
    WindowToEdgeConverter,
};

/// using an enum instead of run-time
/// polymorhism to avoid cost of dynamic dispatch
pub enum Converters {
    SpatialEdgeChange(SpatialEdgeChangeConverter),
    WindowToEdge(WindowToEdgeConverter),
}

impl ImageGraphConverter for Converters {
    fn img_to_graph(&self) -> Graph {
        match self {
            Self::SpatialEdgeChange(converter) => converter.img_to_graph(),
            Self::WindowToEdge(converter) => converter.img_to_graph(),
        }
    }

    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        match self {
            Self::SpatialEdgeChange(converter) => converter.visualize_pheromone(pheromone),
            Self::WindowToEdge(converter) => converter.visualize_pheromone(pheromone),
        }
    }

    fn visualize_conversion(&self) -> Option<PixelMap> {
        match self {
            Self::SpatialEdgeChange(converter) => converter.visualize_conversion(),
            Self::WindowToEdge(converter) => converter.visualize_conversion(),
        }
    }
}

impl Converters {
    pub fn from_string_config_and_pixel_map(
        pixel_map: &PixelMap,
        config: &ConverterStringConfig,
    ) -> Option<Converters> {
        match config {
            ConverterStringConfig::SpatialEdgeChange(opts) => {
                SpatialEdgeChangeConverter::from_str_and_pixel_map(pixel_map, opts)
                    .map(Self::SpatialEdgeChange)
            }
            ConverterStringConfig::WindowToEdge(opts) => {
                WindowToEdgeConverter::from_str_and_pixel_map(pixel_map, opts)
                    .map(Self::WindowToEdge)
            }
        }
    }

    pub fn default(pixel_map: &PixelMap) -> Self {
        Self::SpatialEdgeChange(SpatialEdgeChangeConverter::new(pixel_map))
    }
}

impl FromStrAndPixelMap for Converters {
    fn from_str_and_pixel_map(pixel_map: &PixelMap, config_str: &str) -> Option<Self> {
        let config = ConverterStringConfig::from_str(config_str).ok()?;

        Self::from_string_config_and_pixel_map(pixel_map, &config)
    }
}
