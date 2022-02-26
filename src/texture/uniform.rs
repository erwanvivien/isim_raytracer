use crate::color::Color;
use crate::point::Point;
use crate::texture::{GetColor, LightCoefficients, TextureTrait};

pub struct UniformTexture {
    pub kd: f64, // diffusion
    pub ks: f64, // specularite
    pub ka: f64, // ambiance

    pub color: Color,
}

impl LightCoefficients for UniformTexture {
    fn coefficients(&self, _point: Point) -> (f64, f64, f64) {
        (self.kd, self.ks, self.ka)
    }
}

impl GetColor for UniformTexture {
    fn color(&self, _point: Point) -> Color {
        self.color
    }
}

impl TextureTrait for UniformTexture {}
