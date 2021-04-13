#[derive(Debug, Clone, Copy)]

pub struct XYZColor {
    pub pos_x: usize,
    pub pos_y: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZColor {
    pub fn white() -> Self {
        XYZColor {
            pos_x: 0,
            pos_y: 0,
            x: 0.950456,
            y: 1.0,
            z: 1.088754,
        }
    }

    pub fn to_lab(&self) -> LABColor {
        let epsilon = 0.008856;
        let kappa = 903.3;

        let white = Self::white();

        let xr = self.x / white.x;
        let yr = self.y / white.y;
        let zr = self.z / white.z;

        let xr = iif!(
            xr > epsilon,
            xr.powf(1.0 / 3.0),
            (kappa * xr + 16.0) / 116.0
        );
        let yr = iif!(
            yr > epsilon,
            yr.powf(1.0 / 3.0),
            (kappa * yr + 16.0) / 116.0
        );
        let zr = iif!(
            zr > epsilon,
            zr.powf(1.0 / 3.0),
            (kappa * zr + 16.0) / 116.0
        );

        LABColor {
            x: self.pos_x,
            y: self.pos_y,
            l: 116.0 * yr - 16.0,
            a: 500.0 * (xr - yr),
            b: 200.0 * (yr - zr),
        }
    }
}

#[derive(Debug, Clone, Copy)]

pub struct LABColor {
    pub x: usize,
    pub y: usize,
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl LABColor {
    pub fn empty() -> Self {
        LABColor {
            x: 0,
            y: 0,
            l: 0.0,
            a: 0.0,
            b: 0.0,
        }
    }

    pub fn diff_sq(&self, other: &LABColor) -> f32 {
        (self.l - other.l).powi(2) + (self.a - other.a).powi(2) + (self.b - other.b).powi(2)
    }

    pub fn sum(&self, other: &LABColor) -> Self {
        LABColor {
            x: self.x + other.x,
            y: self.y + other.y,
            l: self.l + other.l,
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }

    pub fn scale(&self, scale: f32) -> Self {
        LABColor {
            x: (self.x as f32 * scale) as usize,
            y: (self.y as f32 * scale) as usize,
            l: self.l * scale,
            a: self.a * scale,
            b: self.b * scale,
        }
    }
}
