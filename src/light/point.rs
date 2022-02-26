use crate::light::LightTrait;
use crate::point::Point;

/// Point Lighting
pub struct PointLight {
    pub point: Point,
}

impl LightTrait for PointLight {}
