use itertools::{EitherOrBoth, Itertools};

use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;
use crate::steganography::data::{Bit, Byte, Data};

use super::EmbedInImage;

/// Assuming we only encode ASCII data, which
/// uses only 7 bits, we can safely reserve this character
const MESSAGE_END_TOKEN: u8 = 0b11111111;

trait Foo {}

pub struct ImageEmbedder;

impl ImageEmbedder {
    fn calculate_n_of_bits_to_embed(mask_byte: u8) -> usize {
        (mask_byte as f32 + 1.0).log2().floor() as usize
    }

    fn embed_n_least_significant_bits_in_byte<
        I: Iterator<Item = Bit> + DoubleEndedIterator + ExactSizeIterator,
    >(
        bits_iter: &mut I,
        n_bits: usize,
        transport_byte: Byte,
    ) -> Byte {
        Data::byte_to_bits_iter(transport_byte)
            .rev()
            .zip_longest(bits_iter.take(n_bits).rev())
            .map(|bits| match bits {
                EitherOrBoth::Both(_, hidden_bit) => hidden_bit,
                EitherOrBoth::Left(transport_bit) => transport_bit,
                EitherOrBoth::Right(hidden_bit) => hidden_bit,
            })
            .rev()
            .enumerate()
            .map(|(idx, bit)| bit.shift_left(7 - idx).raw())
            .sum()
    }

    fn embed_pixel_channel<I: Iterator<Item = Bit> + DoubleEndedIterator + ExactSizeIterator>(
        bits_iter: &mut I,
        transport_pixel_channel: Byte,
        mask_pixel_channel: Byte,
    ) -> Byte {
        let n_bits_to_embed = Self::calculate_n_of_bits_to_embed(mask_pixel_channel);

        Self::embed_n_least_significant_bits_in_byte(
            bits_iter,
            n_bits_to_embed,
            transport_pixel_channel,
        )
    }

    fn embed_pixel<I: Iterator<Item = Bit> + DoubleEndedIterator + ExactSizeIterator>(
        bits_iter: &mut I,
        transport_pixel: &Pixel,
        mask_pixel: &Pixel,
    ) -> Pixel {
        Pixel::new(
            transport_pixel.x,
            transport_pixel.y,
            Self::embed_pixel_channel(bits_iter, transport_pixel.r, mask_pixel.r),
            Self::embed_pixel_channel(bits_iter, transport_pixel.g, mask_pixel.g),
            Self::embed_pixel_channel(bits_iter, transport_pixel.b, mask_pixel.b),
        )
    }

    fn extract_pixel_channel(
        transport_pixel_channel: Byte,
        mask_pixel_channel: Byte,
    ) -> impl Iterator<Item = Bit> {
        let n_bits_to_extract = Self::calculate_n_of_bits_to_embed(mask_pixel_channel);

        Data::byte_to_bits_iter(transport_pixel_channel)
            .rev()
            .take(n_bits_to_extract)
            .rev()
    }

    fn extract_from_pixel(
        transport_pixel: &Pixel,
        mask_pixel: &Pixel,
    ) -> impl Iterator<Item = Bit> {
        Self::extract_pixel_channel(transport_pixel.r, mask_pixel.r)
            .chain(Self::extract_pixel_channel(transport_pixel.g, mask_pixel.g))
            .chain(Self::extract_pixel_channel(transport_pixel.b, mask_pixel.b))
    }
}

impl EmbedInImage for ImageEmbedder {
    fn embed(data: &Data, pixel_map: &PixelMap, mask: &PixelMap) -> PixelMap {
        let bits = data
            .iter_bits()
            .chain(Data::byte_to_bits_iter(MESSAGE_END_TOKEN))
            .collect::<Vec<_>>()
            .into_iter();

        let pixels_zipped_with_mask = pixel_map.pixels().iter().zip_eq(mask.pixels().iter());

        let pixels = pixels_zipped_with_mask
            .scan(bits, |bits_iter, (transport_pixel, mask_pixel)| {
                Option::Some(Self::embed_pixel(bits_iter, transport_pixel, mask_pixel))
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
                Self::extract_from_pixel(transport_pixel, mask_pixel)
            })
            .collect::<Vec<_>>()
            .chunks_exact(8)
            .map(|chunk| Data::bits_to_byte(chunk))
            .take_while(|byte| *byte != MESSAGE_END_TOKEN)
            .collect::<Vec<_>>();

        Data::new(data)
    }
}
