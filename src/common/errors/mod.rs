use std::fmt::Display;

use bmp;

#[derive(Debug)]
pub enum AppError {
    ImageLoadingError(bmp::BmpError),
    ImageSavingError(std::io::Error),

    DataLoadingError(std::io::Error),

    IoError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ImageLoadingError(err) => write!(f, "ImageLoadingError: {}", err),
            AppError::ImageSavingError(err) => write!(f, "ImageSavingError: {}", err),
            AppError::DataLoadingError(err) => write!(f, "DataLoadingError: {}", err),
            AppError::IoError(err) => write!(f, "IoError: {}", err),
        }
    }
}
