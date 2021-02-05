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
            .map(|chunk| Data::bits_to_byte(chunk))
            .collect::<Vec<u8>>();

        Data::new(data)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn iter_bits(&self) -> impl Iterator<Item = u8> + '_ {
        self.bytes
            .iter()
            .flat_map(|byte| Data::byte_to_bits(*byte).into_iter())
    }

    pub fn byte_to_bits(byte: u8) -> Vec<u8> {
        let one = 1u8;

        vec![
            byte >> 7 & one,
            byte >> 6 & one,
            byte >> 5 & one,
            byte >> 4 & one,
            byte >> 3 & one,
            byte >> 2 & one,
            byte >> 1 & one,
            byte >> 0 & one,
        ]
    }

    pub fn bits_to_byte(bits: &[u8]) -> u8 {
        debug_assert_eq!(bits.len(), 8, "Tried to construct byte from unaligned bits");

        (bits[0] << 7)
            + (bits[1] << 6)
            + (bits[2] << 5)
            + (bits[3] << 4)
            + (bits[4] << 3)
            + (bits[5] << 2)
            + (bits[6] << 1)
            + (bits[7] << 0)
    }
}
