use itertools::Itertools;
use ndarray::Array2;
use rkm::{kmeans_lloyd_with_config, Config};
use std::collections::HashMap;

use crate::images::image::Pixel;

use super::PixelMap;

pub type ClusterId = usize;
pub type ClusterMean = f32;
pub type Clusters = Vec<ClusterId>;

pub struct PixelMapClusters {
    image: PixelMap,
    k_clusters: usize,
    clusters: Clusters,
    means_by_cluster_id: HashMap<ClusterId, ClusterMean>,
}

const RANDOM_SEED: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

impl PixelMapClusters {
    pub fn new<F>(pixel_map: &PixelMap, k_clusters: usize, cost_fn: F) -> Self
    where
        F: Fn(&Pixel) -> f32,
    {
        let samples = pixel_map.pixels().iter().map(cost_fn).collect::<Vec<_>>();
        let (clusters, means_by_cluster_id) = Self::execute_k_means(samples, k_clusters);

        assert_eq!(clusters.len(), pixel_map.pixels().len());

        PixelMapClusters {
            image: pixel_map.clone(),
            k_clusters,
            clusters,
            means_by_cluster_id,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (ClusterId, ClusterMean, Vec<Pixel>)> + '_ {
        let map = self
            .clusters
            .iter()
            .cloned()
            .zip(self.image.pixels().iter().cloned())
            .into_group_map();

        map.into_iter()
            .sorted_by_key(|(cluster_id, _)| *cluster_id)
            .filter_map(move |(cluster_id, pixels)| {
                self.means_by_cluster_id
                    .get(&cluster_id)
                    .map(|mean| (cluster_id, *mean, pixels))
            })
    }

    pub fn map_pixels<F: Fn(&Pixel, ClusterId, ClusterMean) -> Pixel>(
        &self,
        mapper: F,
    ) -> PixelMap {
        let pixels = self
            .image
            .pixels()
            .iter()
            .zip(self.clusters.iter().cloned())
            .filter_map(|(px, cluster_id)| {
                self.means_by_cluster_id
                    .get(&cluster_id)
                    .map(|mean| (px, cluster_id, *mean))
            })
            .map(|(px, cluster_id, mean)| mapper(px, cluster_id, mean))
            .collect::<Vec<_>>();

        PixelMap::new(self.image.height, self.image.width, pixels)
    }

    fn execute_k_means(
        samples_1d: Vec<f32>,
        k_clusters: usize,
    ) -> (Clusters, HashMap<ClusterId, ClusterMean>) {
        // we just add additional dimension, no panic risk
        let data: Array2<f32> = Array2::from_shape_vec((samples_1d.len(), 1), samples_1d).unwrap();

        let config = Config::from(Some(RANDOM_SEED), None, None);
        let (means, clusters) = kmeans_lloyd_with_config(&data.view(), k_clusters, &config);
        let means_by_cluster_id = (0..).zip(means.into_iter().cloned()).collect();

        (clusters, means_by_cluster_id)
    }
}
