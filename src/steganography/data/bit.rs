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
