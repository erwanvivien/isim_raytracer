use crate::light::LightTrait;
use crate::point::Point;
use crate::Vector;

/// Point Lighting
pub struct PointLight {
    pub intensity: Vector,
    pub point: Point,
}

impl LightTrait for PointLight {
    fn intensity(&self) -> Vector {
        self.intensity
    }

    fn point(&self) -> Point {
        self.point
    }
}
