use crate::ant_colony::runner::{CycleSummary, EpochSummary};
use crate::steganography::data::Data;
use crate::steganography::quality_assessment::QualityOption;

pub enum ExecutionSummary {
    Embed(EmbeddingSummary),
    Extract(ExtractionSummary),
    Tsp(TspSummary),
}

impl ToString for ExecutionSummary {
    fn to_string(&self) -> String {
        match self {
            ExecutionSummary::Embed(summary) => summary.to_string(),
            ExecutionSummary::Extract(summary) => summary.to_string(),
            ExecutionSummary::Tsp(summary) => summary.to_string(),
        }
    }
}

pub struct EmbeddingSummary {
    image_capacity_bits: usize,
    data_size_bits: usize,
    remaining_bits: usize,
    mse: QualityOption,
    psnr: QualityOption,
    ssim: QualityOption,
    dssim: QualityOption,
    phash: QualityOption,
}

impl EmbeddingSummary {
    pub fn new(
        image_capacity_bits: usize,
        data_size_bits: usize,
        remaining_bits: usize,
        mse: QualityOption,
        psnr: QualityOption,
        ssim: QualityOption,
        dssim: QualityOption,
        phash: QualityOption,
    ) -> Self {
        EmbeddingSummary {
            image_capacity_bits,
            data_size_bits,
            remaining_bits,
            mse,
            psnr,
            ssim,
            dssim,
            phash,
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
            MSE: {}\n\
            PSNR: {}dB\n\
            SSIM: {}\n\
            DSSIM: {}\n\
            PHASH: {}",
            self.image_capacity_bits,
            self.data_size_bits,
            self.remaining_bits,
            self.data_size_bits - self.remaining_bits,
            (self.data_size_bits - self.remaining_bits) as f32 / self.data_size_bits as f32,
            self.mse,
            self.psnr,
            self.ssim,
            self.dssim,
            self.phash
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

pub struct TspSummary {
    cycle: CycleSummary,
    epoch: EpochSummary,
}

impl TspSummary {
    pub fn new(cycle: CycleSummary, epoch: EpochSummary) -> Self {
        TspSummary { epoch, cycle }
    }
}

impl ToString for TspSummary {
    fn to_string(&self) -> String {
        format!("{}", self.epoch.to_string())
    }
}
