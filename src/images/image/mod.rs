mod _tests;
mod colors;
mod pixel;

use image::{DynamicImage, GenericImageView, Pixel as ImagePixel};

use crate::common::errors::AppError;
use crate::images::pixel_map::PixelMap;

pub use colors::{LABColor, XYZColor};
pub use pixel::Pixel;

pub struct Image {
    path: Option<String>,
    img: image::DynamicImage,
    pub height: usize,
    pub width: usize,
}

impl Image {
    pub fn load(path: &str) -> Result<Self, AppError> {
        image::open(path)
            .map(|img| {
                let (width, height) = img.dimensions();

                Image {
                    img,
                    height: height as usize,
                    width: width as usize,
                    path: Option::Some(path.to_owned()),
                }
            })
            .map_err(AppError::ImageLoadingError)
    }

    pub fn from_pixels(pixels: &[Pixel]) -> Self {
        debug_assert_ne!(pixels.len(), 0, "Trying to create image of 0 pixels");

        let width = pixels
            .iter()
            .max_by_key(|px| px.x)
            .map(|px| px.x + 1)
            .unwrap_or(1);

        let height = pixels
            .iter()
            .max_by_key(|px| px.y)
            .map(|px| px.y + 1)
            .unwrap_or(1);

        Self::from_pixels_and_known_dimensions(width, height, pixels)
    }

    pub fn from_pixel_map(pixel_map: &PixelMap) -> Self {
        Self::from_pixels_and_known_dimensions(
            pixel_map.width,
            pixel_map.height,
            pixel_map.pixels(),
        )
    }

    pub fn save(&self, path: &str) -> Result<(), AppError> {
        self.img.save(path).map_err(AppError::ImageSavingError)
    }

    pub fn iter(&self) -> impl Iterator<Item = Pixel> + '_ {
        self.img.pixels().map(move |(x, y, pixel)| {
            let colors_rgb = pixel.to_rgb().0;

            Pixel {
                x: x as usize,
                y: y as usize,
                r: colors_rgb[0],
                g: colors_rgb[1],
                b: colors_rgb[2],
            }
        })
    }

    pub fn get_pixels(&self) -> Vec<Pixel> {
        self.iter().collect()
    }

    pub fn into_pixel_map(self) -> PixelMap {
        PixelMap::new(self.height, self.width, self.get_pixels())
    }

    pub fn resize(&self, width: usize, height: usize) -> Self {
        Image {
            path: Option::None,
            width,
            height,
            img: self.img.resize_exact(
                width as u32,
                height as u32,
                image::imageops::FilterType::Lanczos3,
            ),
        }
    }

    fn from_pixels_and_known_dimensions(width: usize, height: usize, pixels: &[Pixel]) -> Self {
        let empty_image = image::ImageBuffer::new(width as u32, height as u32);

        let img = pixels.iter().fold(empty_image, |mut img, px| {
            let Pixel { x, y, r, g, b } = px.to_owned();

            img.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
            img
        });

        Image {
            img: DynamicImage::ImageRgb8(img),
            path: Option::None,
            width,
            height,
        }
    }
}
