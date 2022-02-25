//! Texture container
use crate::point::Point;

pub mod uniform;

pub trait Lighting {
    /// Returns lighting coefficients at `p` point
    fn coefficients(&self, point: Point) -> (f64, f64, f64);
}

pub struct Texture {}

impl Lighting for Texture {
    fn coefficients(&self, _point: Point) -> (f64, f64, f64) {
        unimplemented!()
    }
}
