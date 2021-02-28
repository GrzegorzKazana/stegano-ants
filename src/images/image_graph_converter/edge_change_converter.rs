use crate::ant_colony::graph::Graph;
use crate::ant_colony::pheromone::Pheromone;
use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;

use super::spatial_image_graph_converter::SpatialImageGraphConverter;
use super::ImageGraphConverter;

/// Image-graph converted build around 4-kind neighbourhood
pub struct EdgeChangeConverter {
    source_image: PixelMap,
    graph: Graph,
}

impl SpatialImageGraphConverter for EdgeChangeConverter {
    fn calc_distance_between_pixels(pixel_a: &Pixel, pixel_b: &Pixel) -> f32 {
        const MAX_DISTANCE: f32 = 255.0 * 255.0 * 3.0;

        let r_diff = f32::from(pixel_a.r) - f32::from(pixel_b.r);
        let g_diff = f32::from(pixel_a.g) - f32::from(pixel_b.g);
        let b_diff = f32::from(pixel_a.b) - f32::from(pixel_b.b);
        let pixel_distance = r_diff.powi(2) + g_diff.powi(2) + b_diff.powi(2);

        (pixel_distance / MAX_DISTANCE) + stability_factor!()
    }

    fn get_pixel_neighbours(pixel_map: &PixelMap, pixel: &Pixel) -> Vec<Pixel> {
        pixel_map.get_neighbours_4(pixel.x, pixel.y)
    }
}

impl ImageGraphConverter for EdgeChangeConverter {
    fn initialize(pixel_map: &PixelMap) -> Self {
        EdgeChangeConverter {
            source_image: pixel_map.clone(),
            graph: Self::construct_graph(pixel_map),
        }
    }

    fn img_to_graph(&self) -> Graph {
        self.graph.clone()
    }

    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        Self::construct_pheromone_visualization(&self.source_image, &self.graph, pheromone)
    }
}
