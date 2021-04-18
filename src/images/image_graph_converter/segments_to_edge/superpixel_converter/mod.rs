mod slic;

use itertools::Itertools;
use std::collections::HashMap;

use crate::ant_colony::graph::NodeId;
use crate::ant_colony::pheromone::Pheromone;
use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;

use super::super::FromStrAndPixelMap;
use super::{SegmentDistances, SegmentId, SegmentToEdgeConverter};

use slic::Slic;

pub struct SuperPixelConverter {
    image: PixelMap,
    n_superpixels: usize,
    labels: Vec<usize>,
    pixels_by_group_id: HashMap<usize, Vec<Pixel>>,
    segment_to_node_pair: HashMap<SegmentId, (NodeId, NodeId)>,
}

impl SuperPixelConverter {
    pub fn new(pixel_map: &PixelMap, target_n_nodes: usize) -> Self {
        let n_superpixels = target_n_nodes * (target_n_nodes - 1) / 2;
        let labels = Slic::from_pixel_map(pixel_map, n_superpixels, 50).run_iterations(10);

        let pixels_by_group_id = labels
            .iter()
            .cloned()
            .zip(pixel_map.pixels().iter().cloned())
            .into_group_map();

        SuperPixelConverter {
            image: pixel_map.clone(),
            n_superpixels,
            labels,
            pixels_by_group_id,
            segment_to_node_pair: Self::build_segment_idx_node_lookup(target_n_nodes),
        }
    }

    fn super_pixel_to_cost((segment_id, pixels): (&SegmentId, &Vec<Pixel>)) -> (SegmentId, f32) {
        let variance = PixelMap::variance_of_pixels(pixels);

        (*segment_id, 1.0 / (variance + stability_factor!()))
    }
}

impl SegmentToEdgeConverter for SuperPixelConverter {
    fn distances(&self) -> Vec<SegmentDistances> {
        self.pixels_by_group_id
            .iter()
            .map(Self::super_pixel_to_cost)
            .sorted_by_key(|(id, _)| id.clone())
            .collect()
    }

    fn visualize_normalized_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        let pixels = self
            .image
            .pixels()
            .iter()
            .zip(self.labels.iter().cloned())
            .map(|(px, segment_id)| self.map_pixel_with_segment_id(px, pheromone, segment_id))
            .collect();

        PixelMap::new(self.image.height, self.image.width, pixels)
    }

    fn lookup_nodes_by_segment_id(&self, segment_id: SegmentId) -> (NodeId, NodeId) {
        self.segment_to_node_pair
            .get(&segment_id)
            .cloned()
            .unwrap_or_default()
    }

    fn map_image_with_intensity_map(
        &self,
        intensity_by_segment_id: HashMap<SegmentId, u8>,
    ) -> PixelMap {
        let pixels = self
            .image
            .pixels()
            .iter()
            .zip(self.labels.iter().cloned())
            .map(|(px, segment_id)| {
                Pixel::grey(
                    px.x,
                    px.y,
                    intensity_by_segment_id
                        .get(&segment_id)
                        .cloned()
                        .unwrap_or_default(),
                )
            })
            .collect();

        PixelMap::new(self.image.height, self.image.width, pixels)
    }
}

impl FromStrAndPixelMap for SuperPixelConverter {
    fn from_str_and_pixel_map(pixel_map: &PixelMap, opts: &str) -> Option<Self> {
        let (target_n_nodes,): (usize,) = opts
            .splitn(1, ',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect_tuple()?;

        Some(SuperPixelConverter::new(pixel_map, target_n_nodes))
    }
}
