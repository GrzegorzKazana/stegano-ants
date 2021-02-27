use crate::steganography::data::Data;

pub enum ExecutionSummary {
    Embed(EmbeddingSummary),
    Extract(ExtractionSummary),
}

impl ToString for ExecutionSummary {
    fn to_string(&self) -> String {
        match self {
            ExecutionSummary::Embed(summary) => summary.to_string(),
            ExecutionSummary::Extract(summary) => summary.to_string(),
        }
    }
}

pub struct EmbeddingSummary {
    image_capacity_bits: usize,
    data_size_bits: usize,
    remaining_bits: usize,
    mean_square_error: f32,
    peak_signal_noise_ratio: f32,
}

impl EmbeddingSummary {
    pub fn new(
        image_capacity_bits: usize,
        data_size_bits: usize,
        remaining_bits: usize,
        mean_square_error: f32,
        peak_signal_noise_ratio: f32,
    ) -> Self {
        EmbeddingSummary {
            image_capacity_bits,
            data_size_bits,
            remaining_bits,
            mean_square_error,
            peak_signal_noise_ratio,
        }
    }
}

impl ToString for EmbeddingSummary {
    fn to_string(&self) -> String {
        format!(
            "Bit capacity: {}\n\
            Num of data bits: {}\n\
            Remaining bits: {}\n\
            Embedded bits: {} ({:>5.2}%)\n\
            Mean square error: {}\n\
            Peak signal noise ratio: {}dB",
            self.image_capacity_bits,
            self.data_size_bits,
            self.remaining_bits,
            self.data_size_bits - self.remaining_bits,
            (self.data_size_bits - self.remaining_bits) as f32 / self.data_size_bits as f32,
            self.mean_square_error,
            self.peak_signal_noise_ratio
        )
    }
}

pub struct ExtractionSummary {
    extracted_data: Data,
}

impl ExtractionSummary {
    pub fn new(extracted_data: Data) -> ExtractionSummary {
        ExtractionSummary { extracted_data }
    }
}

impl ToString for ExtractionSummary {
    fn to_string(&self) -> String {
        format!("Extracted:\n{}", self.extracted_data.to_string())
    }
}
