mod _tests;

use std::fs;

use crate::common::errors::AppError;

#[derive(Debug, PartialEq)]
pub struct Data {
    bytes: Vec<u8>,
}

impl Data {
    pub fn new(bytes: Vec<u8>) -> Self {
        Data { bytes }
    }

    pub fn from_bytes(input: &[u8]) -> Self {
        Data {
            bytes: input.to_vec(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, AppError> {
        fs::read(path)
            .map(Data::new)
            .map_err(AppError::DataLoadingError)
    }

    pub fn from_bits(bits: &[u8]) -> Self {
        debug_assert_eq!(
            bits.len() % 8,
            0,
            "Trying to construct Data::from_bits with uneven byte data"
        );

        let data = bits
            .chunks(8)
            .into_iter()
            .map(|chunk| {
                let zero = 0u8;

                (chunk.get(0).unwrap_or(&zero) << 7)
                    + (chunk.get(1).unwrap_or(&zero) << 6)
                    + (chunk.get(2).unwrap_or(&zero) << 5)
                    + (chunk.get(3).unwrap_or(&zero) << 4)
                    + (chunk.get(4).unwrap_or(&zero) << 3)
                    + (chunk.get(5).unwrap_or(&zero) << 2)
                    + (chunk.get(6).unwrap_or(&zero) << 1)
                    + (chunk.get(7).unwrap_or(&zero) << 0)
            })
            .collect::<Vec<u8>>();

        Data::new(data)
    }

    pub fn iter_bits(&self) -> impl Iterator<Item = u8> + '_ {
        self.bytes.iter().flat_map(|byte| {
            let one = 0b1u8;
            let bits = vec![
                byte >> 7 & one,
                byte >> 6 & one,
                byte >> 5 & one,
                byte >> 4 & one,
                byte >> 3 & one,
                byte >> 2 & one,
                byte >> 1 & one,
                byte >> 0 & one,
            ];

            bits.into_iter()
        })
    }
}
