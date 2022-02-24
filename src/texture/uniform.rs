use crate::point::Point;
use crate::texture::Lighting;

pub struct UniformTexture {
    pub kd: f64,
    pub ks: f64,
    pub ka: f64,
}

impl Lighting for UniformTexture {
    fn coefficients(&self, _point: Point) -> (f64, f64, f64) {
        (self.kd, self.ks, self.ka)
    }
}
