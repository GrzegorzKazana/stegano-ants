use itertools::Itertools;

use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;
use crate::steganography::data::{BitIterator, Byte, Data, ExactBitIterator};

use super::EmbedInImage;

/// Assuming we only encode ASCII data, which
/// uses only 7 bits, we can safely reserve this character
const MESSAGE_END_TOKEN: Byte = 0b11111111;

pub struct MaskImageEmbedder {
    mask: PixelMap,
}

impl MaskImageEmbedder {
    pub fn new(mask: &PixelMap) -> Self {
        MaskImageEmbedder { mask: mask.clone() }
    }

    pub fn estimate_embeddable_bytes_and_transform<F: Fn(usize) -> Option<PixelMap>>(
        self,
        transformer: F,
    ) -> Self {
        match transformer(self.estimate_embeddable_bytes()) {
            Option::Some(new_mask) => MaskImageEmbedder { mask: new_mask },
            Option::None => self,
        }
    }

    fn calculate_n_of_bits_to_embed(mask_byte: Byte) -> usize {
        let max_number_of_bits = 8;
        let bin_size = 256 / max_number_of_bits;

        mask_byte as usize / bin_size
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
        let n_bits_to_embed = Self::calculate_n_of_bits_to_embed(mask_pixel_channel);

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
        let n_bits_to_extract = Self::calculate_n_of_bits_to_embed(mask_pixel_channel);

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
    fn estimate_embeddable_bytes(&self) -> usize {
        self.mask
            .pixels()
            .iter()
            .map(|pixel| {
                Self::calculate_n_of_bits_to_embed(pixel.r)
                    + Self::calculate_n_of_bits_to_embed(pixel.g)
                    + Self::calculate_n_of_bits_to_embed(pixel.b)
            })
            .map(|n_bits_per_px| n_bits_per_px / 8)
            .sum()
    }

    fn embed<I: BitIterator>(&self, data: &mut I, pixel_map: &PixelMap) -> PixelMap {
        let mut bits = data
            .chain(Data::byte_to_bits_iter(MESSAGE_END_TOKEN))
            .collect::<Vec<_>>()
            .into_iter();

        let pixels_zipped_with_mask = pixel_map.pixels().iter().zip_eq(self.mask.pixels().iter());

        let pixels = pixels_zipped_with_mask
            .scan(bits.by_ref(), |bits_iter, (transport_pixel, mask_pixel)| {
                Option::Some(Self::embed_pixel(bits_iter, transport_pixel, mask_pixel))
            })
            .collect::<Vec<_>>();

        PixelMap::new(pixel_map.height, pixel_map.width, pixels)
    }

    fn extract(&self, pixel_map: &PixelMap) -> Data {
        let data = pixel_map
            .pixels()
            .iter()
            .zip_eq(self.mask.pixels().iter())
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
