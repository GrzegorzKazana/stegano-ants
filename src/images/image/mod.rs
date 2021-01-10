mod pixel;

use bmp;

use crate::common::errors::AppError;

pub use pixel::Pixel;

pub struct Image {
    path: Option<String>,
    img: bmp::Image,
    pub height: usize,
    pub width: usize,
}

impl Image {
    pub fn load(path: &str) -> Result<Self, AppError> {
        bmp::open(path)
            .map(|img| {
                let height = img.get_height() as usize;
                let width = img.get_width() as usize;

                Image {
                    img,
                    height,
                    width,
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

        let empty_image = bmp::Image::new(width as u32, height as u32);

        let img = pixels.iter().fold(empty_image, |mut img, px| {
            let Pixel { x, y, r, g, b } = px.to_owned();

            img.set_pixel(x as u32, y as u32, bmp::Pixel { r, g, b });
            img
        });

        Image {
            img,
            path: Option::None,
            width,
            height,
        }
    }

    pub fn save(&self, path: &str) -> Result<(), AppError> {
        self.img.save(path).map_err(AppError::ImageSavingError)
    }

    pub fn iter(&self) -> impl Iterator<Item = Pixel> + '_ {
        self.img.coordinates().map(move |(x, y)| {
            let pixel = self.img.get_pixel(x, y);

            Pixel {
                x: x as usize,
                y: y as usize,
                r: pixel.r,
                g: pixel.g,
                b: pixel.b,
            }
        })
    }

    pub fn get_pixels(&self) -> Vec<Pixel> {
        self.iter().collect()
    }
}
