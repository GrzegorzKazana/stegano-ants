mod _tests;

use itertools::Itertools;
use std::convert::TryFrom;

use crate::common::utils::identity;
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

    pub fn scan_map<T, F: Fn(&Pixel, T) -> (Pixel, T)>(&self, init: T, mapper: F) -> (PixelMap, T) {
        let mut payload = init;
        let mut pixels = Vec::with_capacity(self.pixels.len());

        for pixel in self.pixels() {
            let (new_pixel, new_payload) = mapper(pixel, payload);

            payload = new_payload;
            pixels.push(new_pixel)
        }

        (PixelMap::new(self.height, self.width, pixels), payload)
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
