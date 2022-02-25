use crate::light::Lighting;
use crate::point::Point;

/// Point Lighting
pub struct PointLight {
    pub point: Point,
}

impl Lighting for PointLight {}
