mod _tests;
mod bit;
mod byte;

use std::fs;

use crate::common::errors::AppError;

pub use bit::{Bit, BitIterator, ExactBitIterator};
pub use byte::Byte;

#[derive(Debug, PartialEq)]
pub struct Data {
    bytes: Vec<Byte>,
}

impl Data {
    pub fn new(bytes: Vec<Byte>) -> Self {
        Data { bytes }
    }

    pub fn from_bytes(input: &[Byte]) -> Self {
        Data {
            bytes: input.to_vec(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, AppError> {
        fs::read(path)
            .map(Data::new)
            .map_err(AppError::DataLoadingError)
    }

    pub fn from_bits(bits: &[Bit]) -> Self {
        debug_assert_eq!(
            bits.len() % 8,
            0,
            "Trying to construct Data::from_bits with uneven byte data"
        );

        let data = bits
            .chunks_exact(8)
            .into_iter()
            .map(|chunk| Data::bits_to_byte(chunk))
            .collect::<Vec<Byte>>();

        Data::new(data)
    }

    pub fn bytes(&self) -> &[Byte] {
        &self.bytes
    }

    pub fn num_of_bytes(&self) -> usize {
        self.bytes.len()
    }

    pub fn num_of_bits(&self) -> usize {
        self.bytes.len() * 8
    }

    pub fn take(self, n_bytes: usize) -> Self {
        Data {
            bytes: self.bytes.into_iter().take(n_bytes).collect::<Vec<_>>(),
        }
    }

    pub fn iter_bits(&self) -> impl BitIterator + '_ {
        self.bytes
            .iter()
            .flat_map(|byte| Data::byte_to_bits_iter(*byte))
    }

    pub fn byte_to_bits_iter(byte: Byte) -> impl ExactBitIterator {
        (0..8).rev().map(move |idx| Bit(byte >> idx & 1u8))
    }

    pub fn bits_to_byte(bits: &[Bit]) -> Byte {
        debug_assert_eq!(bits.len(), 8, "Tried to construct byte from unaligned bits");

        (bits[0].raw() << 7)
            + (bits[1].raw() << 6)
            + (bits[2].raw() << 5)
            + (bits[3].raw() << 4)
            + (bits[4].raw() << 3)
            + (bits[5].raw() << 2)
            + (bits[6].raw() << 1)
            + (bits[7].raw() << 0)
    }
}
