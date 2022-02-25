use crate::point::Point;
use crate::texture::LightCoefficients;

pub struct UniformTexture {
    pub kd: f64, // diffusion
    pub ks: f64, // specularite
    pub ka: f64, // ambiance
}

impl LightCoefficients for UniformTexture {
    fn coefficients(&self, _point: Point) -> (f64, f64, f64) {
        (self.kd, self.ks, self.ka)
    }
}
