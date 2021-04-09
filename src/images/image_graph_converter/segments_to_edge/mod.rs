mod _tests;
mod kmeans_converter;
mod window_to_edge_converter;

use itertools::Itertools;
use std::collections::HashMap;
use std::iter::once;

use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node, NodeId};
use crate::ant_colony::pheromone::Pheromone;
use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;

use crate::common::utils::compare_float;

use super::{FromStrAndPixelMap, ImageGraphConverter};

pub use kmeans_converter::KMeansConverter;
pub use window_to_edge_converter::WindowToEdgeConverter;

pub type SegmentId = usize;
pub type SegmentIdToNodes = HashMap<SegmentId, (NodeId, NodeId)>;
pub type SegmentDistances = (SegmentId, f32);

pub trait SegmentToEdgeConverter: FromStrAndPixelMap {
    fn distances(&self) -> Vec<SegmentDistances>;

    fn visualize_normalized_pheromone(&self, pheromone: &Pheromone) -> PixelMap;

    /// maps (0..n_edges) to unique pairs of indicies of graph adjacency matrix
    /// see tests for example
    fn build_segment_idx_node_lookup(n_segments: usize) -> SegmentIdToNodes {
        let n_nodes_u32 = n_segments as u32;

        (0..n_nodes_u32)
            .flat_map(|from| (from + 1..n_nodes_u32).map(move |to| (from, to)))
            .zip(0..)
            .map(|(nodes, idx)| (idx, nodes))
            .collect()
    }

    fn lookup_nodes_by_segment_id(&self, segment_id: SegmentId) -> (NodeId, NodeId);

    fn map_image_with_intensity_map(
        &self,
        intensity_by_segment_id: HashMap<SegmentId, u8>,
    ) -> PixelMap;

    fn map_pixel_with_segment_id(
        &self,
        px: &Pixel,
        pheromone: &Pheromone,
        segment_id: SegmentId,
    ) -> Pixel {
        let (from, to) = self.lookup_nodes_by_segment_id(segment_id);
        let edge_key = AdjacencyListEntry::get_key(from, to);
        let intensity = pheromone.get_pheromone_for_edge(edge_key) * 255.0;

        Pixel::grey(px.x, px.y, intensity as u8)
    }
}

impl<C: SegmentToEdgeConverter> ImageGraphConverter for C {
    fn img_to_graph(&self) -> Graph {
        let distances = self.distances();

        let edges = distances
            .into_iter()
            .flat_map(|(idx, distance)| {
                let (from, to) = self.lookup_nodes_by_segment_id(idx);
                let edge_a = AdjacencyListEntry::new(from, to, distance);
                let edge_b = AdjacencyListEntry::new(to, from, distance);

                once(edge_a).chain(once(edge_b))
            })
            .into_group_map_by(|edge| edge.from);

        let nodes = edges
            .into_iter()
            .map(|(id, adjacency_list)| Node { id, adjacency_list })
            .collect::<Vec<_>>();

        Graph::from_node_vector(nodes)
    }

    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        let pheromone_norm = pheromone.normalize();

        self.visualize_normalized_pheromone(&pheromone_norm)
    }

    fn visualize_conversion(&self) -> Option<PixelMap> {
        let distances = self.distances();

        let distance_max: f32 = distances
            .iter()
            .map(|(_, dist)| *dist)
            .max_by(compare_float)
            .unwrap_or(1.0);

        let intensities_by_segment_id: HashMap<SegmentId, u8> = distances
            .into_iter()
            .map(|(idx, distance)| (idx, (255.0 * distance / distance_max) as u8))
            .collect();

        let image = self.map_image_with_intensity_map(intensities_by_segment_id);

        Some(image)
    }
}
