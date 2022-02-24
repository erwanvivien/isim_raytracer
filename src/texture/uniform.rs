use crate::texture::Lighting;

pub struct UniformTexture {
    kd: f64,
    ks: f64,
    ka: f64,
}

impl Lighting for UniformTexture {
    fn coefficients(&self) -> (f64, f64, f64) {
        (self.kd, self.ks, self.ka)
    }
}
