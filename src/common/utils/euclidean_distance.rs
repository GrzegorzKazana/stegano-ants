use std::ops::Sub;

pub struct Euclidean;

impl Euclidean {
    pub fn dist<A>(a: (A, A), b: (A, A)) -> f32
    where
        A: Sub,
        f32: From<A>,
    {
        let x_diff = f32::from(a.0) - f32::from(b.0);
        let y_diff = f32::from(a.1) - f32::from(b.1);

        (x_diff.powi(2) + y_diff.powi(2)).sqrt()
    }
}
