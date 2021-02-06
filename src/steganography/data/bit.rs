pub trait BitIterator: Iterator<Item = Bit> + DoubleEndedIterator {}
pub trait ExactBitIterator: Iterator<Item = Bit> + DoubleEndedIterator + ExactSizeIterator {}
impl<T> BitIterator for T where T: Iterator<Item = Bit> + DoubleEndedIterator {}
impl<T> ExactBitIterator for T where
    T: Iterator<Item = Bit> + DoubleEndedIterator + ExactSizeIterator
{
}

#[derive(Debug, PartialEq)]
pub struct Bit(pub u8);

impl Bit {
    pub fn shift_left(&self, shift_n: usize) -> Self {
        Bit(self.0 << shift_n)
    }

    pub fn raw(&self) -> u8 {
        self.0
    }
}
