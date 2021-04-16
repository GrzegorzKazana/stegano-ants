use chrono;
use std::fs;

use crate::common::errors::AppError;
use crate::common::utils::{extend_basename, prefix_basename};

use crate::cli::Opts;

use crate::images::image::Image;
use crate::images::pixel_map::PixelMap;

use crate::steganography::data::Data;

use super::AppResult;

pub struct DiskIo {
    opts: Opts,
}

impl DiskIo {
    pub fn new(opts: &Opts) -> Self {
        DiskIo { opts: opts.clone() }
    }

    pub fn load_image(&self, path: &str) -> AppResult<PixelMap> {
        Image::load(path)
            .map_err(|_| format!("Failed to load image {}", path))
            .map(Image::into_pixel_map)
            .map_err(AppError::IoError)
    }

    pub fn save_image(&self, path: &str, pixel_map: &PixelMap) -> AppResult<String> {
        let fullpath = if self.opts.verbose_files {
            let prefix = Self::timestamp();
            let infix = self.opts.to_string();

            let path = extend_basename(path, infix.as_str()).unwrap_or(path.to_owned());
            let path = prefix_basename(&path, prefix.as_str()).unwrap_or(path.to_owned());

            path
        } else {
            path.to_owned()
        };

        Image::from_pixel_map(&pixel_map)
            .save(&fullpath)
            .map(|_| fullpath.clone())
            .map_err(|_| format!("Failed to save image: {}", fullpath))
            .map_err(AppError::IoError)
    }

    pub fn save_steg_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_steg")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    pub fn save_pheromone_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_pher")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    pub fn save_scaled_pheromone_image(
        &self,
        name: &str,
        pixel_map: &PixelMap,
    ) -> AppResult<String> {
        extend_basename(name, "_pher_scaled")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    pub fn save_conversion_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_conv")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    pub fn load_data(&self, path: &str) -> AppResult<Data> {
        Data::from_file(path)
            .map_err(|_| format!("Failed to load data {}", path))
            .map_err(AppError::IoError)
    }

    pub fn load_csv(&self, path: &str) -> AppResult<String> {
        fs::read_to_string(path)
            .map_err(|_| format!("Failed to load csv {}", path))
            .map_err(AppError::IoError)
    }

    fn timestamp() -> String {
        chrono::offset::Utc::now().format("%F-%T").to_string()
    }
}
