mod _tests;

use itertools::Itertools;

use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;
use crate::steganography::data::Data;

pub trait EmbedInImage {
    fn embed(data: &Data, pixel_map: &PixelMap, mask: &PixelMap) -> PixelMap;

    fn extract(pixel_map: &PixelMap, mask: &PixelMap) -> Data;
}

pub struct ImageEmbedder;

impl ImageEmbedder {}

impl EmbedInImage for ImageEmbedder {
    fn embed(data: &Data, pixel_map: &PixelMap, mask: &PixelMap) -> PixelMap {
        let bits = data.iter_bits();

        let pixels = pixel_map
            .pixels()
            .iter()
            .zip_eq(mask.pixels().iter())
            .scan(bits, |bits_iter, (transport_pixel, mask_pixel)| {
                let n_bits_to_embed_r = (mask_pixel.r as f32 + 1.0).log2().floor() as usize;
                let n_bits_to_embed_g = (mask_pixel.g as f32 + 1.0).log2().floor() as usize;
                let n_bits_to_embed_b = (mask_pixel.b as f32 + 1.0).log2().floor() as usize;

                let bits_to_hide_r: u8 = bits_iter
                    .take(n_bits_to_embed_r)
                    .enumerate()
                    .map(|(idx, bit)| bit * 2u8.pow((n_bits_to_embed_r - idx - 1) as u32))
                    .sum();

                let bits_to_hide_g: u8 = bits_iter
                    .take(n_bits_to_embed_g)
                    .enumerate()
                    .map(|(idx, bit)| bit * 2u8.pow((n_bits_to_embed_g - idx - 1) as u32))
                    .sum();

                let bits_to_hide_b: u8 = bits_iter
                    .take(n_bits_to_embed_b)
                    .enumerate()
                    .map(|(idx, bit)| bit * 2u8.pow((n_bits_to_embed_b - idx - 1) as u32))
                    .sum();

                let transport_preserve_mask_r = 255u8 << n_bits_to_embed_r;
                let transport_preserve_mask_g = 255u8 << n_bits_to_embed_g;
                let transport_preserve_mask_b = 255u8 << n_bits_to_embed_b;

                let new_pixel = Pixel::new(
                    transport_pixel.x,
                    transport_pixel.y,
                    (transport_pixel.r & transport_preserve_mask_r) + bits_to_hide_r,
                    (transport_pixel.g & transport_preserve_mask_g) + bits_to_hide_g,
                    (transport_pixel.b & transport_preserve_mask_b) + bits_to_hide_b,
                );

                Option::Some(new_pixel)
            })
            .collect::<Vec<_>>();

        PixelMap::new(pixel_map.height, pixel_map.width, pixels)
    }

    fn extract(pixel_map: &PixelMap, mask: &PixelMap) -> Data {
        let data = pixel_map
            .pixels()
            .iter()
            .zip_eq(mask.pixels().iter())
            .flat_map(|(transport_pixel, mask_pixel)| {
                let n_bits_to_embed_r = (mask_pixel.r as f32 + 1.0).log2().floor() as usize;
                let n_bits_to_embed_g = (mask_pixel.g as f32 + 1.0).log2().floor() as usize;
                let n_bits_to_embed_b = (mask_pixel.b as f32 + 1.0).log2().floor() as usize;

                let hidden_data_mask_r = !(255u8 << n_bits_to_embed_r);
                let hidden_data_mask_g = !(255u8 << n_bits_to_embed_g);
                let hidden_data_mask_b = !(255u8 << n_bits_to_embed_b);

                let hidden_bits_r = transport_pixel.r & hidden_data_mask_r;
                let hidden_bits_g = transport_pixel.g & hidden_data_mask_g;
                let hidden_bits_b = transport_pixel.b & hidden_data_mask_b;

                let bits_r = Data::byte_to_bits(hidden_bits_r)
                    .into_iter()
                    .rev()
                    .take(n_bits_to_embed_r)
                    .rev();

                let bits_g = Data::byte_to_bits(hidden_bits_g)
                    .into_iter()
                    .rev()
                    .take(n_bits_to_embed_g)
                    .rev();

                let bits_b = Data::byte_to_bits(hidden_bits_b)
                    .into_iter()
                    .rev()
                    .take(n_bits_to_embed_b)
                    .rev();

                bits_r
                    .into_iter()
                    .chain(bits_g.into_iter())
                    .chain(bits_b.into_iter())
            })
            .collect::<Vec<_>>()
            .chunks(8)
            .map(|chunk| Data::bits_to_byte(chunk))
            .collect::<Vec<_>>();

        Data::new(data)
    }
}
