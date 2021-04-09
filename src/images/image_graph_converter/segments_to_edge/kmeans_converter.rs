use itertools::Itertools;
use std::collections::HashMap;

use crate::ant_colony::graph::NodeId;
use crate::ant_colony::pheromone::Pheromone;

use crate::images::image::Pixel;
use crate::images::pixel_map::{ClusterId, ClusterMean, PixelMap, PixelMapClusters};

use super::super::FromStrAndPixelMap;
use super::{SegmentDistances, SegmentId, SegmentIdToNodes, SegmentToEdgeConverter};

pub struct KMeansConverter {
    pixel_map_clusters: PixelMapClusters,
    k_clusters: usize,
    cluster_id_to_node_pair: SegmentIdToNodes,
}

impl KMeansConverter {
    pub fn new(pixel_map: &PixelMap, target_n_nodes: usize) -> Self {
        let k_clusters = target_n_nodes * (target_n_nodes - 1) / 2;

        KMeansConverter {
            pixel_map_clusters: pixel_map
                .clusters(k_clusters, |px| Self::pixel_cost_fn(px, pixel_map)),
            k_clusters,
            cluster_id_to_node_pair: Self::build_segment_idx_node_lookup(target_n_nodes),
        }
    }

    fn pixel_cost_fn(px: &Pixel, pixel_map: &PixelMap) -> f32 {
        let neighbours = pixel_map.get_neighbours_8(px.x, px.y);
        let variance = PixelMap::variance_of_pixels(&neighbours);

        variance
    }

    fn cluster_to_distance(
        (cluster_id, cluster_mean, _pixels_in_cluster): (ClusterId, ClusterMean, Vec<Pixel>),
    ) -> (ClusterId, f32) {
        (cluster_id, cluster_mean)
    }
}

impl SegmentToEdgeConverter for KMeansConverter {
    fn distances(&self) -> Vec<SegmentDistances> {
        self.pixel_map_clusters
            .iter()
            .map(Self::cluster_to_distance)
            .collect()
    }

    fn visualize_normalized_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        self.pixel_map_clusters.map_pixels(|px, cluster_id, _| {
            self.map_pixel_with_segment_id(px, pheromone, cluster_id)
        })
    }

    fn lookup_nodes_by_segment_id(&self, segment_id: SegmentId) -> (NodeId, NodeId) {
        self.cluster_id_to_node_pair
            .get(&segment_id)
            .cloned()
            .unwrap_or_default()
    }

    fn map_image_with_intensity_map(
        &self,
        intensity_by_segment_id: HashMap<SegmentId, u8>,
    ) -> PixelMap {
        self.pixel_map_clusters.map_pixels(|px, cluster_id, _| {
            Pixel::grey(
                px.x,
                px.y,
                intensity_by_segment_id
                    .get(&cluster_id)
                    .cloned()
                    .unwrap_or_default(),
            )
        })
    }
}

impl FromStrAndPixelMap for KMeansConverter {
    fn from_str_and_pixel_map(pixel_map: &PixelMap, opts: &str) -> Option<Self> {
        let (target_n_nodes,): (usize,) = opts
            .splitn(1, ',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect_tuple()?;

        Some(KMeansConverter::new(pixel_map, target_n_nodes))
    }
}
