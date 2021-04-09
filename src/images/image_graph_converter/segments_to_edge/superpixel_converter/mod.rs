mod _tests;
mod superpixel_impl;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::ant_colony::graph::NodeId;
use crate::ant_colony::pheromone::Pheromone;
use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;

use super::super::FromStrAndPixelMap;
use super::{SegmentDistances, SegmentId, SegmentToEdgeConverter};

use superpixel_impl::segment;

pub struct SuperPixelConverter {
    image: PixelMap,
    n_super_pixels: usize,
    labels: Vec<usize>,
    pixels_by_group_id: HashMap<usize, Vec<Pixel>>,
    segment_to_node_pair: HashMap<SegmentId, (NodeId, NodeId)>,
}

impl SuperPixelConverter {
    pub fn new(pixel_map: &PixelMap, target_n_nodes: usize) -> Self {
        let target_superpixels = target_n_nodes * (target_n_nodes - 1) / 2;
        let (n_super_pixels, labels) = segment(pixel_map, target_superpixels, 20.0);
        let labels = Self::fix_unexact_superpixel_count(target_superpixels, n_super_pixels, labels);

        let pixels_by_group_id = labels
            .iter()
            .cloned()
            .zip(pixel_map.pixels().iter().cloned())
            .into_group_map();

        SuperPixelConverter {
            image: pixel_map.clone(),
            n_super_pixels,
            labels,
            pixels_by_group_id,
            segment_to_node_pair: Self::build_segment_idx_node_lookup(target_n_nodes),
        }
    }

    /// unfortunately SLIC superpixel algorithm does not guarantee that we will get
    /// exact amount of superpixels we requested. Here we make some sketchy operations
    /// to adjust number of pixels (either by creating single pixel clusters or switching clusters)
    fn fix_unexact_superpixel_count(
        expected: usize,
        actual: usize,
        labels: Vec<usize>,
    ) -> Vec<usize> {
        if expected == actual {
            return labels;
        } else if actual < expected {
            let n_missing = expected - actual;
            let labels_to_replace: HashSet<usize> = (0..n_missing).collect();

            labels
                .into_iter()
                .scan(labels_to_replace, |labels_to_replace, label| {
                    let new_label = match labels_to_replace.take(&label) {
                        Some(_) => label + actual,
                        _ => label,
                    };

                    Some(new_label)
                })
                .collect()
        } else {
            let n_surplus = actual - expected;

            labels
                .into_iter()
                .map(|label| iif!(label >= expected, label - n_surplus, label))
                .collect()
        }
    }
}

impl SegmentToEdgeConverter for SuperPixelConverter {
    fn distances(&self) -> Vec<SegmentDistances> {
        self.pixels_by_group_id
            .iter()
            .map(|(segment_id, pixels)| (*segment_id, PixelMap::variance_of_pixels(pixels)))
            .sorted_by_key(|(id, _)| *id)
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
