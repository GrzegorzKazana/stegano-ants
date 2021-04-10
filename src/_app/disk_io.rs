use std::fs;

use crate::common::errors::AppError;
use crate::common::utils::extend_basename;

use crate::images::image::Image;
use crate::images::pixel_map::PixelMap;

use crate::steganography::data::Data;

use super::AppResult;

pub struct DiskIo;

impl DiskIo {
    pub fn load_image(path: &str) -> AppResult<PixelMap> {
        Image::load(path)
            .map_err(|_| format!("Failed to load image {}", path))
            .map(Image::into_pixel_map)
            .map_err(AppError::IoError)
    }

    pub fn save_image(path: &str, pixel_map: &PixelMap) -> AppResult<String> {
        Image::from_pixel_map(&pixel_map)
            .save(path)
            .map(|_| path.to_owned())
            .map_err(|_| format!("Failed to save image: {}", path))
            .map_err(AppError::IoError)
    }

    pub fn save_steg_image(name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_steg")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| Self::save_image(&name_ext, pixel_map))
    }

    pub fn save_pheromone_image(name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_pher")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| Self::save_image(&name_ext, pixel_map))
    }

    pub fn save_scaled_pheromone_image(name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_pher_scaled")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| Self::save_image(&name_ext, pixel_map))
    }

    pub fn save_conversion_image(name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_conv")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| Self::save_image(&name_ext, pixel_map))
    }

    pub fn load_data(path: &str) -> AppResult<Data> {
        Data::from_file(path)
            .map_err(|_| format!("Failed to load data {}", path))
            .map_err(AppError::IoError)
    }

    pub fn load_csv(path: &str) -> AppResult<String> {
        fs::read_to_string(path)
            .map_err(|_| format!("Failed to load csv {}", path))
            .map_err(AppError::IoError)
    }
}
