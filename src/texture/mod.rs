//! Texture container
use crate::point::Point;
use crate::Color;

pub mod random;
pub mod uniform;

pub trait LightCoefficients {
    /// Returns lighting coefficients at `p` point
    fn coefficients(&self, point: Point) -> (f64, f64, f64);
}

pub trait GetColor {
    fn color(&self, p: Point) -> Color;
}

pub trait TextureTrait: GetColor + LightCoefficients {}

pub struct Texture {}

impl LightCoefficients for Texture {
    fn coefficients(&self, _point: Point) -> (f64, f64, f64) {
        unimplemented!()
    }
}

impl GetColor for Texture {
    fn color(&self, _p: Point) -> Color {
        unimplemented!()
    }
}

impl TextureTrait for Texture {}
