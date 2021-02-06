use std::cmp::{Ord, Ordering};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(x: usize, y: usize, r: u8, g: u8, b: u8) -> Self {
        Pixel { x, y, r, g, b }
    }

    pub fn black(x: usize, y: usize) -> Self {
        Pixel {
            x,
            y,
            r: 0,
            g: 0,
            b: 0,
        }
    }

    pub fn grey(x: usize, y: usize, level: u8) -> Self {
        Pixel {
            x,
            y,
            r: level,
            g: level,
            b: level,
        }
    }

    pub fn scale(&self, scaler: f32) -> Self {
        Pixel {
            x: self.x,
            y: self.y,
            r: (self.r as f32 * scaler).min(255.0) as u8,
            g: (self.g as f32 * scaler).min(255.0) as u8,
            b: (self.b as f32 * scaler).min(255.0) as u8,
        }
    }

    pub fn cmp_by_coords(&self, other: &Self) -> Ordering {
        let x_ord = self.x.cmp(&other.x);
        let y_ord = self.y.cmp(&other.y);

        match (y_ord, x_ord) {
            (Ordering::Less, _) => Ordering::Less,
            (Ordering::Greater, _) => Ordering::Greater,
            (Ordering::Equal, ord) => ord,
        }
    }

    pub fn channel_sum(&self) -> usize {
        self.r as usize + self.g as usize + self.b as usize
    }

    pub fn intensity(&self) -> u8 {
        (self.channel_sum() as f32 / 3.0) as u8
    }
}
