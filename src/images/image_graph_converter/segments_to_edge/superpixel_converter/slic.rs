use itertools::Itertools;
use std::iter::{once, repeat};

use crate::images::image::{LABColor, Pixel};
use crate::images::pixel_map::PixelMap;

use crate::common::utils::{compare_float, MapAccumExt};

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
    pixels: Vec<LABColor>,
    clusters: Vec<LABColor>,
    labels: Vec<usize>,
    distances: Vec<f32>,
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

        Slic {
            n_pixels,
            n_superpixels,
            compactness,
            grid_interval_s,
            width: pixel_map.width,
            height: pixel_map.height,
            pixels,
            clusters: Self::generate_inital_clusters(pixel_map, n_superpixels, grid_interval_s),
            labels: repeat(0).take(n_pixels).collect(),
            distances: repeat(f32::MAX).take(n_pixels).collect(),
        }
    }

    pub fn empty() -> Self {
        Slic {
            n_pixels: 0,
            n_superpixels: 0,
            compactness: 10,
            grid_interval_s: 0,
            width: 0,
            height: 0,
            pixels: vec![],
            clusters: vec![],
            labels: vec![],
            distances: vec![],
        }
    }

    pub fn run_iterations(self, n_iter: usize) -> Vec<usize> {
        (0..n_iter)
            .fold(self, Self::perform_iteration)
            .enforce_connectivity()
            .labels
    }

    fn generate_inital_clusters(
        pixel_map: &PixelMap,
        n_superpixels: usize,
        grid_interval_s: usize,
    ) -> Vec<LABColor> {
        let pixels = pixel_map.pixels();
        let cluster_center_idx = Self::generate_initial_cluster_indicies(
            n_superpixels,
            pixel_map.width,
            pixel_map.height,
            grid_interval_s,
        );

        cluster_center_idx
            .map(|idx| Self::adjust_cluster_initial_position(pixel_map, pixels[idx]))
            .map(|px| px.to_lab())
            .collect()
    }

    fn generate_initial_cluster_indicies(
        n_superpixels: usize,
        width: usize,
        height: usize,
        grid_interval_s: usize,
    ) -> impl Iterator<Item = usize> {
        (0usize..).take(n_superpixels).map(move |i| {
            let px = i * grid_interval_s;
            let x = (px % width).min(width - 1);
            let y = (px / width * grid_interval_s + grid_interval_s / 2).min(height - 1);

            y * width + x
        })
    }

    fn adjust_cluster_initial_position(pixel_map: &PixelMap, init_pos: Pixel) -> Pixel {
        pixel_map
            .get_neighbours_8(init_pos.x, init_pos.y)
            .chain(once(init_pos))
            .min_by(|px_a, px_b| {
                let gradient_a = Self::calc_gradient_around_cluster(pixel_map, px_a);
                let gradient_b = Self::calc_gradient_around_cluster(pixel_map, px_b);

                compare_float(&gradient_a, &gradient_b)
            })
            .unwrap_or(init_pos)
    }

    fn calc_gradient_around_cluster(pixel_map: &PixelMap, px: &Pixel) -> f32 {
        let top = pixel_map.get_pixel_by_delta(px.x, px.y, 0, -1);
        let bottom = pixel_map.get_pixel_by_delta(px.x, px.y, 0, 1);
        let right = pixel_map.get_pixel_by_delta(px.x, px.y, 1, 0);
        let left = pixel_map.get_pixel_by_delta(px.x, px.y, -1, 0);

        match (right, left, top, bottom) {
            (Some(right_px), Some(left_px), Some(top_px), Some(bottom_px)) => {
                right_px.to_lab().diff_sq(&left_px.to_lab())
                    + bottom_px.to_lab().diff_sq(&top_px.to_lab())
            }
            // we do not want cluster centers to be on image edges
            _ => f32::INFINITY,
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
                    let pixels_in_cluster_area = Self::pixels_around_cluster(
                        &pixels,
                        cluster,
                        width,
                        height,
                        grid_interval_s,
                        compactness,
                    );

                    let (labels, distances) = Self::update_labels_and_distances_around_cluster(
                        pixels_in_cluster_area,
                        labels,
                        distances,
                        cluster_idx,
                    );

                    let new_cluster =
                        Self::calculate_new_cluster_center(&labels, &pixels, cluster_idx);

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
        pixels: &'a [LABColor],
        cluster: &'a LABColor,
        width: usize,
        height: usize,
        grid_interval_s: usize,
        compactness: usize,
    ) -> impl Iterator<Item = (usize, f32, &'a LABColor)> + 'a {
        let (cluster_x, cluster_y, interval) = (
            cluster.x as isize,
            cluster.y as isize,
            grid_interval_s as isize,
        );

        let y_start = (cluster_y - interval).max(0);
        let y_end = (cluster_y + interval).min(height as isize - 1);
        let y_range = y_start..=y_end;

        y_range
            .flat_map(move |y| {
                let x_start = (cluster_x - interval).max(0);
                let x_end = (cluster_x + interval).min(width as isize - 1);

                let row_starting_idx = width * y as usize;
                let i_start = row_starting_idx + x_start as usize;
                let i_end = row_starting_idx + x_end as usize;
                let i_range = i_start..=i_end;

                i_range.clone().zip_eq(pixels[i_range].iter())
            })
            .map(move |(idx, px)| {
                let distance = Self::lab_distance(px, &cluster, compactness, grid_interval_s);

                (idx, distance, px)
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

    fn update_labels_and_distances_around_cluster<'a, I>(
        pixels_in_cluster_area: I,
        labels: Vec<usize>,
        distances: Vec<f32>,
        cluster_idx: usize,
    ) -> (Vec<usize>, Vec<f32>)
    where
        I: 'a + Iterator<Item = (usize, f32, &'a LABColor)>,
    {
        pixels_in_cluster_area.fold(
            (labels, distances),
            |(mut labels, mut distances), (pixel_idx, distance, _)| {
                if distance < distances[pixel_idx] {
                    labels[pixel_idx] = cluster_idx;
                    distances[pixel_idx] = distance;
                }

                (labels, distances)
            },
        )
    }

    fn calculate_new_cluster_center(
        labels: &Vec<usize>,
        pixels: &[LABColor],
        cluster_idx: usize,
    ) -> LABColor {
        let (cluster_coord_sum, cluster_count) = (0..)
            .zip(labels.iter())
            .filter(|(_, assigned_cluster_idx)| **assigned_cluster_idx == cluster_idx)
            .map(|(idx, _)| pixels[idx])
            .zip(0..)
            .fold((LABColor::empty(), 0), |(acc_color, _), (px, idx)| {
                (acc_color.sum(&px), idx + 1)
            });

        cluster_coord_sum.scale(1.0 / cluster_count as f32)
    }

    fn index<A: Copy>(&self, data: &[A], x: isize, y: isize) -> Option<A> {
        let is_x_valid = x >= 0 && x < self.width as isize;
        let is_y_valid = y >= 0 && y < self.height as isize;
        let index = self.width as isize * y + x;
        let is_index_valid = index >= 0 && index < self.n_pixels as isize;

        iif!(
            is_x_valid && is_y_valid && is_index_valid,
            data.get(index as usize).cloned(),
            Option::None
        )
    }

    fn index_neighbours<A: Copy>(&self, data: &[A], x: usize, y: usize) -> impl Iterator<Item = A> {
        let (x, y) = (x as isize, y as isize);

        self.index(data, x + 1, y)
            .into_iter()
            .chain(self.index(data, x - 1, y).into_iter())
            .chain(self.index(data, x, y + 1).into_iter())
            .chain(self.index(data, x, y - 1).into_iter())
            .chain(self.index(data, x - 1, y - 1).into_iter())
            .chain(self.index(data, x + 1, y - 1).into_iter())
            .chain(self.index(data, x - 1, y + 1).into_iter())
            .chain(self.index(data, x + 1, y + 1).into_iter())
    }

    fn enforce_connectivity(self) -> Self {
        let pixel_dimensions_1d = (0..self.n_pixels).map(|idx| {
            let x = idx % self.width;
            let y = idx / self.width;

            (x, y)
        });

        let labels = self
            .labels
            .iter()
            .zip(pixel_dimensions_1d)
            .map(|(label, (x, y))| {
                let is_disconnected = self
                    .index_neighbours(&self.labels, x, y)
                    .all(|neighbour_label| neighbour_label != *label);

                if !is_disconnected {
                    return *label;
                }

                self.index_neighbours(&self.labels, x, y)
                    .dedup_with_count()
                    .max_by_key(|(count, _)| *count)
                    .map(|(_, label)| label)
                    .unwrap_or(*label)
            })
            .collect();

        Slic { labels, ..self }
    }
}
