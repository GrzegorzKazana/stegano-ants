use bmp;

pub enum AppError {
    ImageLoadingError(bmp::BmpError),
    ImageSavingError(std::io::Error),
}
