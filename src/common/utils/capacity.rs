use std::str::FromStr;

use super::split_once;

#[derive(Debug, Clone, Copy)]
pub enum Capacity {
    Bytes(usize),
    Kilobytes(usize),
    MegaBytes(usize),
}

impl Capacity {
    pub fn bits(&self) -> usize {
        match self {
            Capacity::Bytes(bytes) => 8 * bytes,
            Capacity::Kilobytes(kbytes) => 8 * 1024 * kbytes,
            Capacity::MegaBytes(mbytes) => 8 * 1024 * 1024 * mbytes,
        }
    }
}

impl FromStr for Capacity {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Option::Some((n_kbytes, _)) = split_once(s, "kB") {
            n_kbytes
                .parse()
                .map(Capacity::Kilobytes)
                .map_err(|_| "Failed to parse KiloBytes")
        } else if let Option::Some((n_mbytes, _)) = split_once(s, "MB") {
            n_mbytes
                .parse()
                .map(Capacity::MegaBytes)
                .map_err(|_| "Failed to parse MegaBytes")
        } else if let Option::Some((n_bytes, _)) = split_once(s, "B") {
            n_bytes
                .parse()
                .map(Capacity::Bytes)
                .map_err(|_| "Failed to parse Bytes")
        } else {
            Err("Failed to parse Capacity")
        }
    }
}
