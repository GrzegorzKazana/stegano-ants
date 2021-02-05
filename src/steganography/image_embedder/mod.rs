mod _tests;
mod image_embedder;

use crate::images::pixel_map::PixelMap;
use crate::steganography::data::Data;

pub use image_embedder::ImageEmbedder;

pub trait EmbedInImage {
    fn embed(data: &Data, pixel_map: &PixelMap, mask: &PixelMap) -> PixelMap;

    fn extract(pixel_map: &PixelMap, mask: &PixelMap) -> Data;
}
