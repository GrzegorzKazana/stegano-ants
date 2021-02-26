use crate::images::pixel_map::PixelMap;

pub trait AssessSteganogramQuality {
    fn eval(transport: &PixelMap, steganogram: &PixelMap) -> f32;
}

/// Average square difference between each channel in each pixel.
/// \sum \left ( \frac{(r_a-r_b)^2 + (g_a-g_b)^2 + (b_a-b_b)^2}{3} \right )
pub struct MeanSquareError;

impl AssessSteganogramQuality for MeanSquareError {
    fn eval(transport: &PixelMap, steganogram: &PixelMap) -> f32 {
        let transport_len = transport.pixels().len();
        let steganogram_len = steganogram.pixels().len();
        let n_of_channels_per_pixel = 3;
        let n_of_components = steganogram_len * n_of_channels_per_pixel;

        debug_assert_eq!(
            transport_len, steganogram_len,
            "Cannot assess MSE for images of different size"
        );

        let transport_channels = transport.pixels().iter().flat_map(|px| px.iter_channels());
        let steganogram_channels = steganogram
            .pixels()
            .iter()
            .flat_map(|px| px.iter_channels());

        let square_error: isize = transport_channels
            .zip(steganogram_channels)
            .map(|(channel_a, channel_b)| (channel_a as isize - channel_b as isize).pow(2))
            .sum();

        square_error as f32 / n_of_components as f32
    }
}

/// https://en.wikipedia.org/wiki/Peak_signal-to-noise_ratio
/// Measured in dB.
/// 10 * \log_{10} \frac{max^2}{mse}
pub struct PeakSignalNoiseRatio;

impl AssessSteganogramQuality for PeakSignalNoiseRatio {
    fn eval(transport: &PixelMap, steganogram: &PixelMap) -> f32 {
        debug_assert_eq!(
            transport.pixels().len(),
            steganogram.pixels().len(),
            "Cannot assess PSNR for images of different size"
        );

        let max_signal_value = transport
            .pixels()
            .iter()
            .flat_map(|px| px.iter_channels())
            .max()
            .unwrap_or(0) as f32;

        let mse = MeanSquareError::eval(transport, steganogram);

        10.0 * (max_signal_value.powi(2) / mse).log10()
    }
}
