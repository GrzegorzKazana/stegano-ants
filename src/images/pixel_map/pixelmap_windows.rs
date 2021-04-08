use itertools::Itertools;

use crate::common::utils::{ceil_div, measure_chunks, MeasuredChunk};
use crate::images::image::Pixel;

use super::PixelMap;

pub struct PixelMapWindows {
    image: PixelMap,
    n_x_windows: usize,
    n_y_windows: usize,
}

impl PixelMapWindows {
    pub fn new(pixel_map: &PixelMap, n_x_windows: usize, n_y_windows: usize) -> Self {
        PixelMapWindows {
            image: pixel_map.clone(),
            n_x_windows,
            n_y_windows,
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, PixelMap)> + 'a {
        self.generate_offsets().map(
            move |(window_idx, (row_from, row_chunk_len, _), (col_from, col_chunk_len, _))| {
                let pixels = self
                    .image
                    .pixels()
                    .chunks_exact(self.image.width)
                    .skip(row_from)
                    .take(row_chunk_len)
                    .flat_map(|row| row.iter().skip(col_from).take(col_chunk_len))
                    .map(|pixel| pixel.translate(-(col_from as isize), -(row_from as isize)))
                    .collect();

                (
                    window_idx,
                    PixelMap::new(row_chunk_len, col_chunk_len, pixels),
                )
            },
        )
    }

    pub fn map_pixels<F: Fn(&Pixel, usize) -> Pixel>(&self, mapper: F) -> PixelMap {
        let offsets = self.generate_offsets().collect::<Vec<_>>();

        self.image.map(|pixel| {
            let current_window_idx = offsets
                .iter()
                .find(|offsets| Self::is_pixel_in_offset(pixel, offsets))
                .map(|(idx, _, _)| *idx)
                .unwrap_or_default();

            mapper(pixel, current_window_idx)
        })
    }

    fn generate_offsets(&self) -> impl Iterator<Item = (usize, MeasuredChunk, MeasuredChunk)> {
        let n_rows_per_chunk = ceil_div(self.image.height, self.n_y_windows);
        let n_cols_per_chunk = ceil_div(self.image.width, self.n_x_windows);

        let actual_n_rows = measure_chunks(0..self.image.height, n_rows_per_chunk);
        let actual_n_cols = measure_chunks(0..self.image.width, n_cols_per_chunk);

        assert_eq!(
            self.n_x_windows,
            actual_n_cols.len(),
            "window_iter failed to chunk in x axis"
        );
        assert_eq!(
            self.n_y_windows,
            actual_n_rows.len(),
            "window_iter failed to chunk in y axis"
        );

        actual_n_rows
            .into_iter()
            .cartesian_product(actual_n_cols.into_iter())
            .zip(0..)
            .map(|((row_offsets, col_offsets), idx)| (idx, row_offsets, col_offsets))
    }

    fn is_pixel_in_offset(px: &Pixel, offsets: &(usize, MeasuredChunk, MeasuredChunk)) -> bool {
        let (_, (row_from, _, row_to), (col_from, _, col_to)) = offsets;

        px.x >= *col_from && px.x <= *col_to && px.y >= *row_from && px.y <= *row_to
    }
}
