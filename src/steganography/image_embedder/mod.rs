mod _tests;
mod mask_image_embedder;

use crate::images::pixel_map::PixelMap;
use crate::steganography::data::{Data, ExactBitIterator};

pub use mask_image_embedder::MaskImageEmbedder;

pub trait EmbedInImage {
    fn estimate_embeddable_bytes(&self) -> usize;

    fn embed<I: ExactBitIterator>(&self, data: &mut I, pixel_map: &PixelMap) -> PixelMap;

    fn extract(&self, pixel_map: &PixelMap) -> Data;
}
