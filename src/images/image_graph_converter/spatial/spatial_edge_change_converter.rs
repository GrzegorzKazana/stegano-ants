use crate::ant_colony::graph::Graph;
use crate::ant_colony::pheromone::Pheromone;
use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;

use super::super::{FromStrAndPixelMap, ImageGraphConverter};
use super::SpatialImageGraphConverter;

/// Image-graph converted build around 4-kind neighbourhood
/// Attributes distance proportional to pixel distance
/// (the more pixels differ, the higher distance between them)
pub struct SpatialEdgeChangeConverter {
    source_image: PixelMap,
    graph: Graph,
}

impl SpatialEdgeChangeConverter {
    pub fn new(pixel_map: &PixelMap) -> Self {
        SpatialEdgeChangeConverter {
            source_image: pixel_map.clone(),
            graph: Self::construct_graph(pixel_map),
        }
    }
}

impl SpatialImageGraphConverter for SpatialEdgeChangeConverter {
    fn calc_distance_between_pixels(pixel_a: &Pixel, pixel_b: &Pixel) -> f32 {
        const MAX_DISTANCE: f32 = 255.0 * 255.0 * 3.0;

        let r_diff = f32::from(pixel_a.r) - f32::from(pixel_b.r);
        let g_diff = f32::from(pixel_a.g) - f32::from(pixel_b.g);
        let b_diff = f32::from(pixel_a.b) - f32::from(pixel_b.b);
        let pixel_distance = r_diff.powi(2) + g_diff.powi(2) + b_diff.powi(2);

        (pixel_distance / MAX_DISTANCE) + stability_factor!()
    }

    fn get_pixel_neighbours(pixel_map: &PixelMap, pixel: &Pixel) -> Vec<Pixel> {
        pixel_map.get_neighbours_4(pixel.x, pixel.y).collect()
    }
}

impl ImageGraphConverter for SpatialEdgeChangeConverter {
    fn img_to_graph(&self) -> Graph {
        self.graph.clone()
    }

    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        Self::construct_pheromone_visualization(&self.source_image, &self.graph, pheromone)
    }
}

impl FromStrAndPixelMap for SpatialEdgeChangeConverter {
    fn from_str_and_pixel_map(pixel_map: &PixelMap, _: &str) -> Option<Self> {
        Some(Self::new(pixel_map))
    }
}
