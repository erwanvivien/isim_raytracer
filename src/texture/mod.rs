//! Texture container
use crate::point::Point;

pub mod uniform;

pub trait LightCoefficients {
    /// Returns lighting coefficients at `p` point
    fn coefficients(&self, point: Point) -> (f64, f64, f64);
}

pub struct Texture {}

impl LightCoefficients for Texture {
    fn coefficients(&self, _point: Point) -> (f64, f64, f64) {
        unimplemented!()
    }
}
