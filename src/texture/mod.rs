mod uniform;

pub trait Lighting {
    fn coefficients(&self) -> (f64, f64, f64);
}

pub struct Texture {}

impl Lighting for Texture {
    fn coefficients(&self) -> (f64, f64, f64) {
        unimplemented!()
    }
}
