mod superpixel_impl;

use itertools::Itertools;
use std::collections::HashMap;

use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node, NodeId};
use crate::ant_colony::pheromone::Pheromone;
use crate::images::image::{Image, Pixel};
use crate::images::pixel_map::PixelMap;

use super::{FromStrAndPixelMap, ImageGraphConverter};

use superpixel_impl::segment;

pub struct SuperPixelsVariance {
    image: PixelMap,
    n_super_pixels: usize,
    labels: Vec<i32>,
    pixels_by_group_id: HashMap<i32, Vec<Pixel>>,
}

impl SuperPixelsVariance {
    pub fn new(pixel_map: &PixelMap, target_superpixels: usize) -> Self {
        let (n_super_pixels, labels) = segment(pixel_map, target_superpixels, 20.0);
        let pixels_by_group_id = labels
            .iter()
            .cloned()
            .zip(pixel_map.pixels().iter().cloned())
            .into_group_map();

        SuperPixelsVariance {
            image: pixel_map.clone(),
            n_super_pixels,
            labels,
            pixels_by_group_id,
        }
    }
}

impl ImageGraphConverter for SuperPixelsVariance {
    fn img_to_graph(&self) -> Graph {
        todo!()
    }

    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        todo!()
    }

    fn visualize_conversion(&self) -> Option<PixelMap> {
        None
    }
}

impl FromStrAndPixelMap for SuperPixelsVariance {
    fn from_str_and_pixel_map(pixel_map: &PixelMap, config: &str) -> Option<Self> {
        todo!()
    }
}
