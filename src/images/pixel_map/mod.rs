mod _tests;

use itertools::Itertools;
use std::convert::TryFrom;

use crate::common::utils::{ceil_div, identity, measure_chunks};
use crate::images::image::Image;
use crate::images::image::Pixel;

#[derive(Debug, Clone, PartialEq)]
pub struct PixelMap {
    pub height: usize,
    pub width: usize,
    pixels: Vec<Pixel>,
}

impl PixelMap {
    pub fn new(height: usize, width: usize, pixels: Vec<Pixel>) -> Self {
        let pixels_sorted = pixels.into_iter().sorted_by(Pixel::cmp_by_coords).collect();

        PixelMap {
            height,
            width,
            pixels: pixels_sorted,
        }
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    pub fn map<F: Fn(&Pixel) -> Pixel>(&self, mapper: F) -> PixelMap {
        let pixels = self.pixels().iter().map(mapper).collect();

        PixelMap::new(self.height, self.width, pixels)
    }

    pub fn scale(&self, scaler: f32) -> PixelMap {
        self.map(|pixel| pixel.scale(scaler))
    }

    pub fn increment(&self, increment: isize) -> PixelMap {
        self.map(|pixel| pixel.increment(increment))
    }

    pub fn resize(&self, width: usize, height: usize) -> Self {
        Image::from_pixel_map(self)
            .resize(width, height)
            .into_pixel_map()
    }

    pub fn invert(&self) -> Self {
        self.map(Pixel::invert)
    }

    pub fn get_neighbours_4(&self, x: usize, y: usize) -> Vec<Pixel> {
        vec![
            self.get_pixel_by_delta(x, y, 0, -1),
            self.get_pixel_by_delta(x, y, 1, 0),
            self.get_pixel_by_delta(x, y, 0, 1),
            self.get_pixel_by_delta(x, y, -1, 0),
        ]
        .into_iter()
        .filter_map(identity)
        .collect::<Vec<_>>()
    }

    pub fn get_neighbours_8(&self, x: usize, y: usize) -> Vec<Pixel> {
        vec![
            self.get_pixel_by_delta(x, y, 0, -1),
            self.get_pixel_by_delta(x, y, 1, -1),
            self.get_pixel_by_delta(x, y, 1, 0),
            self.get_pixel_by_delta(x, y, 1, 1),
            self.get_pixel_by_delta(x, y, 0, 1),
            self.get_pixel_by_delta(x, y, -1, 1),
            self.get_pixel_by_delta(x, y, -1, 0),
            self.get_pixel_by_delta(x, y, -1, -1),
        ]
        .into_iter()
        .filter_map(identity)
        .collect::<Vec<_>>()
    }

    pub fn get_pixel_by_delta(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<Pixel> {
        let pixel_x = usize::try_from(x as isize + dx).ok();
        let pixel_y = usize::try_from(y as isize + dy).ok();

        pixel_x
            .zip(pixel_y)
            .and_then(|(px_x, px_y)| self.index(px_x, px_y))
            .and_then(|idx| self.pixels.get(idx))
            .cloned()
    }

    pub fn window_iter<'a>(
        &'a self,
        num_x_slices: usize,
        num_y_slices: usize,
    ) -> impl Iterator<Item = PixelMap> + 'a {
        let n_rows_per_chunk = ceil_div(self.height, num_y_slices);
        let n_cols_per_chunk = ceil_div(self.width, num_x_slices);

        let actual_n_rows = measure_chunks(0..self.height, n_rows_per_chunk);
        let actual_n_cols = measure_chunks(0..self.width, n_cols_per_chunk);

        assert_eq!(
            num_x_slices,
            actual_n_cols.len(),
            "window_iter failed to chunk in x axis"
        );
        assert_eq!(
            num_y_slices,
            actual_n_rows.len(),
            "window_iter failed to chunk in y axis"
        );

        actual_n_rows
            .into_iter()
            .cartesian_product(actual_n_cols.into_iter())
            .map(
                move |((row_from, row_chunk_len, _), (col_from, col_chunk_len, _))| {
                    let pixels = self
                        .pixels
                        .chunks_exact(self.width)
                        .skip(row_from)
                        .take(row_chunk_len)
                        .flat_map(|row| row.iter().skip(col_from).take(col_chunk_len))
                        .map(|pixel| pixel.translate(-(col_from as isize), -(row_from as isize)))
                        .collect();

                    PixelMap::new(row_chunk_len, col_chunk_len, pixels)
                },
            )
    }

    pub fn avg(&self) -> f32 {
        let sum: usize = self
            .pixels
            .iter()
            .map(|pixel| pixel.intensity() as usize)
            .sum();

        sum as f32 / self.pixels.len() as f32
    }

    pub fn variance(&self) -> f32 {
        let avg_intensity = self.avg();
        let diffs: f32 = self
            .pixels
            .iter()
            .map(|pixel| (pixel.intensity() as f32 - avg_intensity).powi(2))
            .sum();

        diffs / self.pixels.len() as f32
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        let is_x_valid = x < self.width;
        let is_y_valid = y < self.height;

        iif!(
            is_x_valid && is_y_valid,
            Option::Some(y * self.width + x),
            Option::None
        )
    }
}
