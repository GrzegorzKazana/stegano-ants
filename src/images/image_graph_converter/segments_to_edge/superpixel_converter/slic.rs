use std::{collections::BTreeMap, iter::once};

use crate::images::image::{LABColor, Pixel};
use crate::images::pixel_map::PixelMap;

use crate::common::utils::{compare_float, MapAccumExt, Vec2d, Vec2dCoords};

/// Implementation of Simple Linear Iterative Clustering algorithm
/// https://www.iro.umontreal.ca/~mignotte/IFT6150/Articles/SLIC_Superpixels.pdf
///
/// divides given image into n_superpixels, returns list of labels
/// in vector of size equal to vector of pixels
pub struct Slic {
    n_pixels: usize,
    n_superpixels: usize,
    compactness: usize,
    grid_interval_s: usize,
    width: usize,
    height: usize,
    pixels: Vec2d<LABColor>,
    clusters: Vec<LABColor>,
    labels: Vec2d<usize>,
    distances: Vec2d<f32>,
}

impl Slic {
    pub fn from_pixel_map(pixel_map: &PixelMap, n_superpixels: usize, compactness: usize) -> Self {
        let n_pixels = pixel_map.width * pixel_map.height;
        let grid_interval_s = (n_pixels as f32 / n_superpixels as f32).sqrt() as usize;

        let pixels = pixel_map
            .pixels()
            .iter()
            .map(Pixel::to_lab)
            .collect::<Vec<_>>();

        let pixels = Vec2d::new(pixels, pixel_map.width, pixel_map.height);
        let clusters = Self::generate_inital_clusters(&pixels, n_superpixels, grid_interval_s);

        Slic {
            n_pixels,
            n_superpixels,
            compactness,
            grid_interval_s,
            width: pixel_map.width,
            height: pixel_map.height,
            pixels,
            clusters,
            labels: Vec2d::fill(0, pixel_map.width, pixel_map.height),
            distances: Vec2d::fill(f32::MAX, pixel_map.width, pixel_map.height),
        }
    }

    pub fn run_iterations(self, n_iter: usize) -> Vec<usize> {
        (0..n_iter)
            .fold(self, Self::perform_iteration)
            .enforce_connectivity()
            .enforce_connectivity()
            .enforce_connectivity()
            .labels
            .to_vec()
    }

    fn generate_inital_clusters(
        pixels: &Vec2d<LABColor>,
        n_superpixels: usize,
        grid_interval_s: usize,
    ) -> Vec<LABColor> {
        let cluster_center_idx = Self::generate_initial_cluster_indicies(
            n_superpixels,
            pixels.width,
            pixels.height,
            grid_interval_s,
        );

        cluster_center_idx
            .map(|coords| Self::adjust_cluster_initial_position(pixels, coords))
            .collect()
    }

    fn generate_initial_cluster_indicies(
        n_superpixels: usize,
        width: usize,
        height: usize,
        grid_interval_s: usize,
    ) -> impl Iterator<Item = Vec2dCoords> {
        (0usize..).take(n_superpixels).map(move |i| {
            let px = i * grid_interval_s;
            let x = (px % width).min(width - 1);
            let y = (px / width * grid_interval_s + grid_interval_s / 2).min(height - 1);

            (x, y)
        })
    }

    fn adjust_cluster_initial_position(
        pixels: &Vec2d<LABColor>,
        init_coords: Vec2dCoords,
    ) -> LABColor {
        let initial = pixels[init_coords];

        pixels
            .index_neighbours_8(init_coords)
            .cloned()
            .chain(once(initial))
            .min_by(|px_a, px_b| {
                let gradient_a = Self::calc_gradient_around_cluster(pixels, px_a);
                let gradient_b = Self::calc_gradient_around_cluster(pixels, px_b);

                compare_float(&gradient_a, &gradient_b)
            })
            .unwrap_or(initial)
    }

    fn calc_gradient_around_cluster(pixels: &Vec2d<LABColor>, cluster: &LABColor) -> f32 {
        let top = pixels.index_by_delta((cluster.x, cluster.y), (0, -1));
        let bottom = pixels.index_by_delta((cluster.x, cluster.y), (0, 1));
        let right = pixels.index_by_delta((cluster.x, cluster.y), (1, 0));
        let left = pixels.index_by_delta((cluster.x, cluster.y), (-1, 0));

        match (right, left, top, bottom) {
            (Some(right_px), Some(left_px), Some(top_px), Some(bottom_px)) => {
                right_px.diff_sq(&left_px) + bottom_px.diff_sq(&top_px)
            }
            // we do not want cluster centers to be on image edges
            _ => f32::MAX,
        }
    }

    fn perform_iteration(self, _iter_count: usize) -> Self {
        let Slic {
            clusters,
            labels,
            distances,
            pixels,
            width,
            height,
            grid_interval_s,
            compactness,
            ..
        } = self;

        let indexed_clusters = (0..).zip(clusters.iter());
        let ((labels, distances), clusters) = indexed_clusters
            .map_accum(
                (labels, distances),
                |(labels, distances), (cluster_idx, cluster)| {
                    let pixels_in_cluster_area =
                        Self::pixels_around_cluster(&pixels, cluster, grid_interval_s, compactness);

                    let (labels, distances) = Self::update_labels_and_distances_around_cluster(
                        pixels_in_cluster_area,
                        labels,
                        distances,
                        cluster_idx,
                    );

                    let new_cluster =
                        Self::calculate_new_cluster_center(&pixels, &labels, cluster_idx);

                    ((labels, distances), new_cluster)
                },
            )
            .collect_with_state();

        Slic {
            clusters,
            labels,
            distances,
            pixels,
            width,
            height,
            grid_interval_s,
            compactness,
            ..self
        }
    }

    fn pixels_around_cluster<'a>(
        pixels: &'a Vec2d<LABColor>,
        cluster: &'a LABColor,
        grid_interval_s: usize,
        compactness: usize,
    ) -> impl Iterator<Item = (LABColor, f32)> + 'a {
        pixels
            .iter_block((cluster.x, cluster.y), grid_interval_s)
            .cloned()
            .map(move |px| {
                let distance = Self::lab_distance(&px, &cluster, compactness, grid_interval_s);

                (px, distance)
            })
    }

    fn lab_distance(
        px_a: &LABColor,
        px_b: &LABColor,
        compactness: usize,
        grid_interval_s: usize,
    ) -> f32 {
        let d_lab = px_a.diff_sq(px_b).sqrt();
        let d_x = (px_a.x as isize - px_b.x as isize).pow(2) as f32;
        let d_y = (px_a.y as isize - px_b.y as isize).pow(2) as f32;
        let d_xy = (d_x + d_y).sqrt();

        let ratio = compactness as f32 / grid_interval_s as f32;

        d_lab + ratio * d_xy
    }

    fn update_labels_and_distances_around_cluster<I: Iterator<Item = (LABColor, f32)>>(
        pixels_in_cluster_area: I,
        labels: Vec2d<usize>,
        distances: Vec2d<f32>,
        cluster_idx: usize,
    ) -> (Vec2d<usize>, Vec2d<f32>) {
        pixels_in_cluster_area.fold(
            (labels, distances),
            |(labels, distances), (pixel, distance)| {
                let coords = (pixel.x, pixel.y);

                if distance >= distances[coords] {
                    (labels, distances)
                } else {
                    (
                        labels.assign(coords, cluster_idx),
                        distances.assign(coords, distance),
                    )
                }
            },
        )
    }

    fn calculate_new_cluster_center(
        pixels: &Vec2d<LABColor>,
        labels: &Vec2d<usize>,
        cluster_idx: usize,
    ) -> LABColor {
        let (cluster_coord_sum, cluster_count) = labels
            .iter()
            .filter(|(_, _, assigned_cluster_idx)| **assigned_cluster_idx == cluster_idx)
            .map(|(x, y, _)| pixels[(x, y)])
            .zip(0..)
            .fold((LABColor::empty(), 0), |(acc_color, _), (px, idx)| {
                (acc_color.sum(&px), idx + 1)
            });

        cluster_coord_sum.scale(1.0 / cluster_count as f32)
    }

    fn enforce_connectivity(self) -> Self {
        // 1 would mean 8 pixels around, (-1, 1) in both directions
        const NEIGHBOUR_RANGE: usize = 2;
        const NEIGHBOUR_COUNT: usize = (2 * NEIGHBOUR_RANGE + 1) * (2 * NEIGHBOUR_RANGE + 1) - 1;

        let labels = self
            .labels
            .iter()
            .map(|(x, y, label)| {
                let connected_neighbours = self
                    .labels
                    .index_neighbours_range((x, y), NEIGHBOUR_RANGE)
                    .filter(|neighbour_label| **neighbour_label == *label)
                    .count();

                let is_disconnected = connected_neighbours < NEIGHBOUR_COUNT / 2;

                if !is_disconnected {
                    return *label;
                }

                let neighbour_label_count = self
                    .labels
                    .index_neighbours_range((x, y), NEIGHBOUR_RANGE)
                    .fold(BTreeMap::new(), |mut label_count, label| {
                        let current_count = label_count.get(&label).cloned().unwrap_or(0);
                        label_count.insert(label, current_count + 1);
                        label_count
                    });

                neighbour_label_count
                    .into_iter()
                    .max_by_key(|(_, count)| *count)
                    .map(|(label, _)| *label)
                    .unwrap_or(*label)
            })
            .collect();

        let labels = Vec2d::new(labels, self.width, self.height);

        Slic { labels, ..self }
    }
}
