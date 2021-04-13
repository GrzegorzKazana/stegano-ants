use itertools::Itertools;

use crate::common::utils::{ExactChainExt, MapAccumExt};
use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;
use crate::steganography::data::{BitIterator, Byte, Data, ExactBitIterator};

use super::EmbedInImage;

/// Assuming we only encode ASCII data, which
/// uses only 7 bits, we can safely reserve this character
const MESSAGE_END_TOKEN: Byte = 0b11111111;

/// Image embedder/extractor using a mask image for calculating how many bits to embed
/// in given pixel.
/// Mask and transport image must have exact same size.
pub struct MaskImageEmbedder {
    mask: PixelMap,
}

impl MaskImageEmbedder {
    pub fn new(mask: &PixelMap) -> Self {
        MaskImageEmbedder { mask: mask.clone() }
    }

    pub fn scale_mask_to_fit(self, target_bits: usize) -> PixelMap {
        let embeddable_bits_in_image = self.estimate_embeddable_bits();

        let pixels = self
            .mask
            .pixels()
            .iter()
            .map_accum(
                (embeddable_bits_in_image, target_bits),
                |(remaining_to_embed, remaining_to_target), pixel| {
                    let ratio = remaining_to_target as f32 / remaining_to_embed as f32;
                    // since `calculate_n_of_bits_to_embed` splits byte range into 32 bit bins
                    // we risk loosing to much capacity by scaling down. Therefore we add
                    // addiional 16 to roughly compensate that. For upscaling we do the opposite.
                    let increment = iif!(ratio > 1.0, -16, 16);
                    let capacity = Self::calculate_n_bits_to_embed_in_pixel(pixel);

                    let scaled_pixel = pixel.scale(ratio).increment(increment);
                    let scaled_pixel_capacity =
                        Self::calculate_n_bits_to_embed_in_pixel(&scaled_pixel)
                            .min(remaining_to_target);

                    let new_accumulator = (
                        remaining_to_embed - capacity,
                        remaining_to_target - scaled_pixel_capacity,
                    );

                    (new_accumulator, scaled_pixel)
                },
            )
            .collect::<Vec<_>>();

        PixelMap::new(self.mask.height, self.mask.width, pixels)
    }

    fn calculate_n_of_bits_to_embed_in_byte(mask_byte: Byte) -> usize {
        let max_number_of_bits = 8;
        let bin_size = 256 / max_number_of_bits;

        mask_byte as usize / bin_size
    }

    fn calculate_n_bits_to_embed_in_pixel(mask_pixel: &Pixel) -> usize {
        Self::calculate_n_of_bits_to_embed_in_byte(mask_pixel.r)
            + Self::calculate_n_of_bits_to_embed_in_byte(mask_pixel.g)
            + Self::calculate_n_of_bits_to_embed_in_byte(mask_pixel.b)
    }

    fn embed_n_bits_in_byte<I: ExactBitIterator>(
        bits_iter: &mut I,
        n_bits: usize,
        transport_byte: Byte,
    ) -> Byte {
        // postfix is only applicable in cases when we run out of bits to embed while processing the byte
        // it is important to note that for given input:
        // n_bits: 5, transport_byte: 0b00000000, bits_iter: 1,1,1 (end of iterator)
        // we produce output of 0b00011100 instead of 0b00000111
        let transport_bits_a = Data::byte_to_bits_iter(transport_byte);
        let transport_bits_b = Data::byte_to_bits_iter(transport_byte);

        let prefix = transport_bits_a.take(8 - n_bits);
        let infix = bits_iter.take(n_bits);

        let postfix_length = n_bits - infix.len();
        let postfix = transport_bits_b.rev().take(postfix_length).rev();

        prefix
            .chain(infix)
            .chain(postfix)
            .enumerate()
            .map(|(idx, bit)| bit.shift_left(7 - idx).raw())
            .sum()
    }

    fn embed_pixel_channel<I: ExactBitIterator>(
        bits_iter: &mut I,
        transport_pixel_channel: Byte,
        mask_pixel_channel: Byte,
    ) -> Byte {
        let n_bits_to_embed = Self::calculate_n_of_bits_to_embed_in_byte(mask_pixel_channel);

        Self::embed_n_bits_in_byte(bits_iter, n_bits_to_embed, transport_pixel_channel)
    }

    fn embed_pixel<I: ExactBitIterator>(
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
    ) -> impl BitIterator {
        let n_bits_to_extract = Self::calculate_n_of_bits_to_embed_in_byte(mask_pixel_channel);

        Data::byte_to_bits_iter(transport_pixel_channel)
            .rev()
            .take(n_bits_to_extract)
            .rev()
    }

    fn extract_from_pixel(transport_pixel: &Pixel, mask_pixel: &Pixel) -> impl BitIterator {
        Self::extract_pixel_channel(transport_pixel.r, mask_pixel.r)
            .chain(Self::extract_pixel_channel(transport_pixel.g, mask_pixel.g))
            .chain(Self::extract_pixel_channel(transport_pixel.b, mask_pixel.b))
    }
}

impl EmbedInImage for MaskImageEmbedder {
    fn estimate_embeddable_bits(&self) -> usize {
        self.mask
            .pixels()
            .iter()
            .map(Self::calculate_n_bits_to_embed_in_pixel)
            .sum()
    }

    fn embed<I: ExactBitIterator>(&self, data: &mut I, pixel_map: &PixelMap) -> PixelMap {
        let mut bits = data.chain_exact(Data::byte_to_bits_iter(MESSAGE_END_TOKEN));

        let pixels_zipped_with_mask = pixel_map.pixels().iter().zip_eq(self.mask.pixels().iter());

        let pixels = pixels_zipped_with_mask
            .scan(bits.by_ref(), |bits_iter, (transport_pixel, mask_pixel)| {
                Option::Some(Self::embed_pixel(bits_iter, transport_pixel, mask_pixel))
            })
            .collect::<Vec<_>>();

        PixelMap::new(pixel_map.height, pixel_map.width, pixels)
    }

    fn extract(&self, pixel_map: &PixelMap) -> Data {
        let bytes = pixel_map
            .pixels()
            .iter()
            .zip_eq(self.mask.pixels().iter())
            .flat_map(|(transport_pixel, mask_pixel)| {
                Self::extract_from_pixel(transport_pixel, mask_pixel)
            })
            .collect::<Vec<_>>()
            .chunks_exact(8)
            .map(Data::bits_to_byte)
            .take_while(|byte| *byte != MESSAGE_END_TOKEN)
            .collect::<Vec<_>>();

        Data::new(bytes)
    }
}
