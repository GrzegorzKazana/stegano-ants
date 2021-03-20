use std::{fmt::Display, process::Command};

use crate::common::utils::split_once;

pub struct ImageMagick;

impl ImageMagick {
    pub fn mse(path_a: &str, path_b: &str) -> QualityOption {
        Self::eval("MSE", path_a, path_b)
    }
    pub fn psnr(path_a: &str, path_b: &str) -> QualityOption {
        Self::eval("PSNR", path_a, path_b)
    }
    pub fn ssim(path_a: &str, path_b: &str) -> QualityOption {
        Self::eval("SSIM", path_a, path_b)
    }
    pub fn dssim(path_a: &str, path_b: &str) -> QualityOption {
        Self::eval("DSSIM", path_a, path_b)
    }
    pub fn phash(path_a: &str, path_b: &str) -> QualityOption {
        Self::eval("PHASH", path_a, path_b)
    }

    fn eval(metric: &str, path_a: &str, path_b: &str) -> QualityOption {
        let outout_img_placeholder = if cfg!(windows) { "nul" } else { "/dev/null" };

        let cmd = Command::new("magick")
            .arg("compare")
            .arg("-metric")
            .arg(metric)
            .arg(path_a)
            .arg(path_b)
            .arg(outout_img_placeholder)
            .output()
            .ok();

        cmd.map(|output| String::from_utf8_lossy(&output.stderr).into_owned())
            .map(|str| QualityOption::from_str(&str))
            .unwrap_or_else(QualityOption::empty)
    }
}

pub enum Quality {
    Value(f32),
    ValueNormalized((f32, f32)),
}

impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Value(val) => write!(f, "{}", val),
            Quality::ValueNormalized((val, norm)) => write!(f, "{} ({})", val, norm),
        }
    }
}

pub struct QualityOption(Option<Quality>);

impl QualityOption {
    fn from_str(s: &str) -> QualityOption {
        match split_once(s, " ") {
            Option::None => QualityOption((s.parse()).map(Quality::Value).ok()),
            Option::Some((val, norm)) => {
                let value = val.parse().ok();
                let normalized = norm
                    .strip_prefix("(")
                    .and_then(|s| s.strip_suffix(")"))
                    .and_then(|s| s.parse().ok());

                QualityOption(value.zip(normalized).map(Quality::ValueNormalized))
            }
        }
    }

    fn empty() -> Self {
        QualityOption(Option::None)
    }
}

impl Display for QualityOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Option::Some(quality) => write!(f, "{}", quality),
            Option::None => write!(f, "---"),
        }
    }
}
