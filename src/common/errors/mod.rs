use bmp;

#[derive(Debug)]
pub enum AppError {
    ImageLoadingError(bmp::BmpError),
    ImageSavingError(std::io::Error),
}
